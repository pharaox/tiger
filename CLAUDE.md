# Tiger

Tiger is a validator for Paradox game mods (CK3, Vic3, Imperator, EU5, HOI4). It reports errors in mod and vanilla script, localization, and GUI files.

GitHub: [amtep/tiger](https://github.com/amtep/tiger) (upstream). Wiki: [Overview for coders](https://github.com/amtep/tiger/wiki/Overview-for-coders).

## Concepts

- **Vanilla**: The unmodified base game files.
- **Item**: A named game object of some type, e.g. a trait or a faith. Item types are the `Item` enum in `tiger-tables/src/item.rs`, which also gives the folder each type is loaded from.
- **Scope**: The type of game object a piece of script operates on, e.g. character or title. Scope types are the `Scopes` bitflags in `src/scopes.rs`.
- **Tables**: Per-game lists of the engine's built-in triggers, effects, iterators, event targets, on_actions, and modifiers, with their scopes and argument types.
- **Script docs**: The `.log` files the game writes with the `script_docs` console command (`effects.log`, `triggers.log`, `event_targets.log`, `on_actions.log`, `modifiers.log`, ...). Datatypes come from the `dump_data_types` command.
- **`.info` files**: Documentation files in vanilla game folders that describe the syntax of the files in that folder.

## Context

### Layout

- `src/` (crate `tiger-lib`): Nearly all the code, built for one game at a time through a cargo feature (`ck3`, `vic3`, ...).
  - `src/*.rs`: Shared code, e.g. `validator.rs` (field-by-field validation), `trigger.rs`, `effect.rs`, `everything.rs` (loading and validation order).
  - `src/data/`: Loaders and validators shared between games (e.g. localization, CoAs, DLCs, portraits, scripted modifiers).
  - `src/parse/`, `src/gui/`: The script parser (including reader exports) and GUI validation.
  - `src/<game>/data/`: Loaders and validators per item type.
  - `src/<game>/tables/`: The game's tables.
- `tiger-tables/`: The `Item` enum and the datatypes generated from `dump_data_types` (`<game>/include/`).
- `<game>-tiger/`: Thin binaries. `src/lib.rs` contains `GAME_CONSTS`, including the supported game version.
- `utils/`: Generators for the tables, `munch-data-types` and `munch-script-docs`.
- `mod-test-vanilla/`: An empty mod for running tiger on vanilla only, with `--show-vanilla`.

### Build and Check

- If `cargo` is not on `PATH`, use `$HOME/.cargo/bin/cargo`.
- Check other games **one at a time**, e.g. `cargo check -p vic3-tiger`. Checking several at once enables several game features together and fails.
- Run tiger on vanilla:

  ```shell
  ./target/release/ck3-tiger --game <game-dir> --config <conf> --show-vanilla --json mod-test-vanilla/descriptor.mod > <out>.json
  ```

## Guidelines

### General

- **Keep tiger strict.** Never relax a check because vanilla trips it. Vanilla has real bugs, and mods such as the CK3 Unofficial Patch rely on tiger to find them.
- **Change a check only with evidence** that the game accepts the construct. In order of strength:
  1. `.info` files and the script docs
  2. Consistent vanilla usage that demonstrably works
  3. The game binary: in `strings <game-binary>` output, parser keys appear as standalone strings, while docs text appears as whole sentences. Only a hit counts: some keys, such as most GUI properties, live in shared engine code and aren't in the strings.
  4. An in-game test with the debug console (e.g. `effect ...`, then check `error.log`); ask the user to run it
- **Treat forks as references only.** Other people's versions of a change may contain relaxations. Review every hunk before reusing it.
- **Keep diffs minimal and upstream-ready.** Other games must keep compiling.

### Conventions

Follow upstream's conventions for marking judgement calls:

- **Docs mismatch**: ``vd.advice_field("x", "docs say `x` but it's `y`")``.
- **Removed field**: `vd.advice_field("x", "removed in <version>")`. **Renamed field**: `vd.replaced_field("old", "new")`, with the new name bare (no backticks), unless it's a description such as ``"`new` in faiths and rites"``.
- **Deprecated form** (the docs call it deprecated or old, or say not to use it anymore, but it still works): `warn(ErrorKey::Deprecated)` with "should use `new` instead of `old`". Keep mappings from old to new values in a table. Not for alternative forms that the docs merely stop showing, such as `name = yes` in parameter lists.
- **Alias** (the docs say an old field name is still read as an alias of a new one, e.g. a "legacy alias" or a "compatibility option"): the same, but `untidy`, since it works just like the new name.
- **Dropped from the docs** (a field, value, scope or root the new docs no longer mention, and vanilla doesn't use): stop accepting it, so it is reported as unknown. Keep it only if tiger has evidence that it still works, such as a comment or TODO in the code. If the docs or vanilla name a successor, use `replaced_field`.
- **Removed trigger or effect**: `Removed("<version>", "<replacement or empty>")`. Removed targets, iterators and modifiers go in the matching `*_REMOVED` tables.
- **Undocumented** but used by vanilla: `// undocumented` (`# undocumented` in `on_action.rs`). Search all `.info` files first: a field can be documented in another folder's file. Keep a field that neither the docs nor vanilla use only if the game binary has it: `// undocumented, but in the game binary`.
- **Optional scope** in the on_action and rule tables, when the docs say it may not be set: `# may be unset`.
- **Guess**: `TODO: verify ...` (`# TODO: verify scope type` in `on_action.rs`). This includes following vanilla where the docs are silent and vanilla uses the form only a few times (e.g. an undocumented root), which only a game test can settle.
- **Version markers**: `LAST UPDATED <GAME> VERSION <version>` above a table means the whole table was checked against the source its comment names, for that version. Update the marker only after such a check.
- **Argument types** in the trigger and effect tables:
  - Triggers follow the `Traits:` line of the docs: "Database key" means `Item`, scope or target means `Scope`, both mean `ScopeOrItem`.
  - Effects follow the docs examples plus vanilla usage. If vanilla uses bare keys anywhere, use `ScopeOrItem`. Confirm with a vanilla run.

### Idioms

- **Scope contexts per field**: use `field_*_rooted` when a field needs only a root, and `field_*_builder` otherwise, so that reports point at the field. For several fields with the same scopes, pass one closure that captures nothing (it is `Copy`) to each. Don't build a context up front for a single field.
- **`field_item`** rather than `field_value` plus `data.verify_exists`.
- **Repeats and clashes**: find them in one pass with a `TigerHashSet` or `TigerHashMap`, and report each once, with a `loc_msg` to the first. A repeat within one list is `untidy`, a clash between items an error. Checks across all items of a type go in a `validate_all` hook (registered in `everything.rs`), in load order (sorted by `key.loc`), so that reports point at the later item.
- **Table lookups per item**: a `static LazyLock<TigerHashMap>` next to the table, behind a lookup function (like `scope_to_scope` in `targets.rs`), rather than a search of the table.
- **Loc keys the docs name**, including the default key of an absent field: `if !vd.field_item(...) { data.verify_exists(...) }`. Use `mark_used` only for keys the docs call optional.
- **Script value fields**: a range or sum check applies only when the value is a plain number (`get_number()`); `field_numeric_range` on the same field would reject blocks. Don't make a field that the docs and vanilla show as a number a script value without evidence.
- **Reports**: choose the severity by the definitions in `src/report/report_struct.rs` (a warning is something the player will notice), and an `ErrorKey` that names the problem (`MissingItem` only for an item that doesn't exist). Name the clashing value in the message, and point at it with `loc_msg`.
- **No duplication**: put a scope context or logic used twice in one function, after the `impl` that uses it, and don't add a second item type for the same thing.
- **Names**: `loca` for loc keys, `loc` for file locations.

### Style

- **Match the surrounding code.** Follow its naming, idioms, and formatting.
- **Use ASCII only.** Don't use non-ASCII characters in comments or prose (arrows, long dashes, ellipses, etc.).
- **Comment sparingly.** Keep comments concise. Follow upstream in which comments are appropriate where, and in their style (`///` doc comments on items, `//` comments for non-obvious code, the markers in [Conventions](#conventions)).

### Validation

- Before committing, run:
  - `cargo fmt`
  - `cargo clippy -p <game>-tiger --all-targets`: don't add new warnings
  - `cargo test -p tiger-lib`
  - `cargo check -p <other-game>-tiger` for each other game, one at a time
- Tiger output is not fully deterministic between runs. Confirm a surprising difference between two runs with a repeat run before attributing it to a change.

### Git

- **Never** add Claude or Claude Code attributions (e.g. `Co-Authored-By` trailers or "Generated with Claude Code") to commit messages, pull request descriptions, issues, or comments.
- **Never** change the git staging area (e.g. `git stash`, `git restore --staged`, `git reset`) without asking first. The user stages changes to review them. For a baseline of the committed state, use a temporary `git worktree`.
- **Stage by path.** Avoid `git add -A`, `git add .`, and `git commit -a`, which can sweep in local-only changes.
- **One logical change per commit.** Use a `<game>:` prefix (none for shared code) and a short imperative summary, e.g. `ck3: update modifiers for 1.20`. The body says what changed and why, including any verified game behaviour.
- **Push only when asked.**

## Docs

Notes kept across updates, in `.claude/docs/`:

- [tiger-architecture.md](docs/tiger-architecture.md): How tiger works, and where each part of an update fits.
- [info-mapping.md](docs/info-mapping.md): Every `.info` file and the tiger code that validates it.
- [follow-up.md](docs/follow-up.md): Known gaps in tiger's CK3 support (F numbers), with their upstream issues.
- [game-tests.md](docs/game-tests.md): In-game tests (T numbers) and their results.
- [changed-checks.md](docs/changed-checks.md): Existing checks that each update changed, for reviewers.

Keep them current: remove done follow-ups (the numbers stay), and record test results and changed checks as they happen.

## Skills

- Use the [update-ck3 skill](skills/update-ck3/SKILL.md) to update tiger to a new CK3 version.
