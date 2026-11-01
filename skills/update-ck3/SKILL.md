---
name: update-ck3
description: Update tiger to a new CK3 version without relaxing any checks
argument-hint: "<ck3-version>"
---

# Update Tiger to a New CK3 Version

Update tiger's CK3 support to a new game version: the engine tables, the loaders and validators for new and changed game files, and acceptance runs on vanilla and on mods. The goal is to remove the false positives caused by the update without relaxing any check.

## Concepts

- **Mechanical update**: Changes to the tables and other generated or engine-defined parts. The game's script docs and `data_types` logs are the main source.
- **Game file changes**: New or changed file formats in game folders, which need new or updated loaders and validators. The `.info` files and vanilla usage are the main source.
- **Run**: A tiger run on vanilla or a mod, saved as JSON (`--json`).
- **Group**: Reports with the same key, area (folder), and message after normalizing names and numbers. Runs are compared by groups.
- **Baselines**: Runs made before any changes:
  - **V_old**: The old tiger on the old vanilla.
  - **V_new0**: The old tiger on the new vanilla. It shows the size of the job.
  - The same for important mods, with and without the sections of their tiger configs that hide false positives from the outdated tiger.

## Context

- Read the project [CLAUDE.md](../../CLAUDE.md) first. Its principles and conventions apply to every step.
- Ask the user where the following inputs are, and prepare any that are missing:
  - The script docs and `data_types` logs of the old and the new version, sorted with `sort-log.py`.
  - Frozen copies of the old and the new game folders (e.g. `game-<version>` next to `game`). Compare them with `diff -r`.
  - The mods to use for acceptance, and where the user's installed tiger binary is.
- Runs are not deterministic, so the same binary gives slightly different reports. For comparisons, make them byte-identical with a local patch to `src/helpers.rs` that gives `TigerHashMap` a hasher with fixed seeds, plus `RAYON_NUM_THREADS=1`. Never commit that patch. Check first whether upstream has fixed this.
- Keep every run, named in step order (e.g. `tmp/A3.json`, `tmp/B12.json`). The vanilla classification in step 6 traces each report group through them.

## Arguments

- **CK3 version** (position $0): The new game version, e.g. `1.20.1.1`.

## Guidelines

- **Stop after every step.** Show what changed, explain the tiger concepts involved, and list every judgement call. Commit only after the user has reviewed the step.
- **Working on your own**: When the user lets you continue without stopping, apply the rules already agreed, and log every judgement call in `tmp/judgement-calls.md` instead of stopping: the docs text, the vanilla evidence, what you did, and whether a game test could settle it. The user reviews the log at the end.
- **Self-audit before showing a step.** Check it against the [Conventions](../../CLAUDE.md#conventions). Confirm any decision that makes a check stricter, e.g. making an argument scope-only, with a vanilla run.
- **Fix only false positives caused by the update.** Pre-existing tiger gaps are out of scope, except mismatches between the tables and the logs or game files, and contradictions between the docs (old or new) and tiger that vanilla settles in favour of the docs. Fix those alongside, and record them in the catalogue's "not caused by the update" section.
- **Keep upstream bugs out of the update.** A bug found along the way that the update didn't cause (1.20: `field_script_value_no_breakdown_rooted` did the same as `field_script_value_rooted`) goes in a separate commit. Don't trust a helper's name: read what it does.
- **Keep a working plan** in `.claude/plans/ck3-<version>-update.md` with progress, run results, findings, and a "How to resume" section. Update it after every step.
- **Keep a catalogue of changed pre-existing checks** (e.g. `tmp/changed-checks.md`), split by cause: not caused by the update (gaps fixed alongside), caused by the update, and restored (checks the update silently disabled). Update it after every step; it is what reviewers need most.
- **Keep this skill up to date.** After every step, add new lessons here or to [examples.md](examples.md).
- **Import references as branches.** Import other people's updates for the same version (forks, archives) as local branches `ref/<name>`, and browse them with `git diff main ref/<name> -- <path>`. Treat every hunk as untrusted, and reject relaxations.
- **Tests first, if they exist.** If tiger has end-to-end tests for the affected area (test mods under `tests/files/` run by `tests/*.rs`), update them to the new `.info` files **before** changing `src/`, then measure progress by how many of them pass, alongside the vanilla runs. Tests that already passed must keep passing.
- If in doubt, **stop and ask the user**.

## Workflow

### 1. Prepare

1. Create the branch `adapt/ck3-<version>` from an up-to-date `main`.
2. Diff the `.info` files of the old and new game folders, and the old and new logs, by name. Record the additions, removals, and changes in the plan. This is the map of the work.
3. List the `.info` files that didn't change too. Step 5 reviews only the changed ones, so plan an audit of the unchanged ones against tiger, or record them as a follow-up (1.20: 41 of 203 were unchanged and went unaudited until the end).

### 2. Make Baselines

Build the old tiger and make the baseline runs:

```shell
RAYON_NUM_THREADS=1 ./target/release/ck3-tiger --game <game-dir> --config <empty-conf> --show-vanilla --json mod-test-vanilla/descriptor.mod > tmp/<run>.json
```

Summarize them with `scripts/groups.py`.

### 3. Mechanical Update

One commit per sub-step.

1. **Version**: `GAME_CONSTS.version` in `ck3-tiger/src/lib.rs`.
2. **Datatypes**: Run `cargo run -p utils --bin munch-data-types -- --game ck3 --logs <logs>/data_types --out tiger-tables/src/ck3/include/`. Review the diff, restore any hand refinements the tool overwrote, and add new datatypes until it compiles.
3. **Scopes**: Add and remove `Scopes` flags in `src/scopes.rs`, together with the CK3 name and display mappings, then follow the compile errors.
4. **Event targets** (`src/ck3/tables/targets.rs`, from `event_targets.log`):
   - Targets with `Requires Data: yes` go in `SCOPE_PREFIX`, all others in `SCOPE_TO_SCOPE`.
   - Removed targets go in `SCOPE_TO_SCOPE_REMOVED`.
   - Check existing targets for changed input or output scopes.
5. **On actions** (`src/ck3/tables/on_action.rs`, from `on_actions.log`):
   - Add the `From Code: Yes` entries, with scopes from the comments in vanilla `common/on_action`.
   - Mark scopes that are used but undocumented with `# undocumented`, and guessed scope types with `# TODO: verify scope type`.
   - Check for code on_actions that tiger never had.
6. **Modifiers** (`src/ck3/tables/modifs.rs`, from `modifiers.log`):
   - Check the `lookup_modif` patterns first, since many modifiers are generated (per terrain, per skill, ...).
   - Renamed modifiers go in `MODIF_REMOVED_TABLE`.
   - Check every static modifier against the localization. A modifier without the standard keys usually needs a `SPECIAL_MODIF_LOC_TABLE` entry.
7. **Triggers, effects, iterators**: Run `munch-script-docs --game ck3 --gamefiles <new-game-dir> --logs <logs> --out src/ck3/tables`. It adds `UncheckedTodo` entries for new items and marks removed ones with `// TODO: REMOVED`. Then, by hand:
   - Give every new entry real validation, based on the docs and vanilla usage. A worksheet with the docs entry, any reference entry, and the vanilla usage per name helps.
     - Complex block effects get a function in `src/ck3/effect_validation.rs`.
     - Fields specific to an iterator go in `validate_inside_iterator` (`src/ck3/validate.rs`).
     - New `trigger(arg)` forms go in `TRIGGER_COMPLEX`.
   - Turn `// TODO: REMOVED` markers into `Removed(...)` entries or `ITERATOR_REMOVED` entries.
   - The tool **doesn't detect changed docs** of existing entries. Diff the old and new logs per entry and review each change.
   - Values of item types that aren't loaded yet stay `Unchecked` with a `TODO: <version>` marker, to be resolved in step 5.
   - Split this into several commits, e.g. iterators and removals, new entries, changed entries.
8. **Common dirs**: Regenerate `COMMON_DIRS` in `src/ck3/tables/misc.rs` from `find game/common -type d`, then prune by hand.
9. **Defines** (`src/ck3/tables/defines.rs`): Run `scripts/defines_diff.py <old-root> <new-root>`. Add the new defines in sorted order, with the types of their closest siblings in the table (upstream uses `Number` even for whole numbers), and remove the ones the new version dropped. A vanilla run shows missing ones as "unknown define".
   - Each root needs its own copy of `jomini/`, frozen with `game/`. Roots that link the live `jomini/` hide every jomini change (1.20 was diffed this way).
   - A file in `game/common/defines` replaces the `jomini/common/defines` file of the same path, so the replaced file's defines don't count.
10. **Other per-version tables**:
    - **Sounds** (`src/ck3/tables/sounds.rs`): the bank files store event names compressed, so ask the user for a dump. In game with `-debug_mode`, the console command `explorer` opens the object explorer; its "Audio Events" table can be dumped to CSV (rows with a `Path` are the events and snapshots). Merge the new paths next to their neighbours.
    - **DLC music** (`DLC_MUSIC` in `src/ck3/tables/misc.rs`): the top-level keys under `game/dlc/*/music/`, except `music_player_categories`.
    - **Religion terms** (`CUSTOM_RELIGION_LOCAS` in `tiger-tables/src/ck3/misc.rs`): the `Faith.random_` values in the datatypes dump.
    - **Loc tables** (`src/ck3/tables/localization.rs`): code-supplied `$PARAM$` placeholders (`BUILTIN_MACROS`) and `#TOOLTIP:` tags (`COMPLEX_TOOLTIPS`, from `localization/english/tooltip_structs_l_english.yml`). Upstream confirmed the tag value types in game.
    - **Version markers**: list every `LAST UPDATED CK3 VERSION` below the new version (`grep -rn 'LAST UPDATED CK3' src tiger-tables`). Check each table against the source its comment names, and update the marker only for tables checked in full (see the [Conventions](../../CLAUDE.md#conventions)).

### 4. Checkpoint

Build, run on the new vanilla, and compare with V_old using `scripts/compare_runs.py`. Most of the new groups that remain should point to step 5.

### 5. Game File Changes

One commit per item type, largest report groups first.

- **New item types**:
  - Add a `DbKind` in `src/ck3/data/<type>.rs`, registered with `inventory::submit! { ItemLoader::Normal(GameFlags::Ck3, Item::X, X::add) }`.
  - Add `Item::X` and its folder in `tiger-tables/src/item.rs`.
  - Validate from the `.info` file and vanilla usage, including the required localization keys.
  - Use `add_subitems` for items defined inside other items (e.g. parameters).
- **Changed `.info` files**: Diff old and new, then update the matching validator. Removed fields get `advice_field("x", "removed in <version>")`. Go through every hunk, not only new field names: most changes are new scopes in `# root = ...` / `# scope:x = ...` comments, new filter or enum values, and changed types of known fields, and a search for unknown field names finds none of these (1.20: activity types had 214 vanilla false positives left after such a search). A list of documented field names that the code never mentions is still a useful extra check; many are placeholders from examples (`pillar_key`, gene or title names), so skip those.
- **History files** (`characters.rs`, `prov_history.rs`, `title_history.rs`): Check them for new fields. These tend to cause the largest report groups.
- **New folders outside `common/`**: Diff the folder lists of the whole game tree, not only `common/` (e.g. 1.20 added `history/faiths`).
- **Moved structures**: When items move to their own folder (1.20: faiths out of religions, laws out of law groups, doctrines naming their group), update everything that read the old structure, e.g. checks that looked up the group's member list. Fix the folder named in `item.rs` too; it appears in "not defined in ..." messages.
- **Docs vs. old tiger code**: When docs contradict existing tiger code, check vanilla. If vanilla uses the documented form, fix tiger. If it doesn't, tiger may be right and the docs wrong: leave tiger as it is with a `// TODO: verify ...` comment. This applies to docs that didn't change too (see [examples.md](examples.md#unchanged-docs-contradicting-tiger-120)).
- **Forms the new docs drop**: A field, value, scope or root that the new docs no longer mention, and that vanilla doesn't use, is most likely gone or was a docs error. Stop accepting it, unless tiger's code has evidence that it works (see the [Conventions](../../CLAUDE.md#conventions)). Confirm with a vanilla run that nothing uses it.
- **Shared structures**: When an `.info` file says its items share a structure documented elsewhere (1.20: culture pillars and traditions, `_cultural_traits.info`), validate the shared part with one function and only the extras per item type, and audit all the related `.info` files together.
- **Rules in the docs**: Check them against vanilla before adding them. Some apply only at the top level (bookmark characters), and some are contradicted by consistent vanilla use; ask the user before dropping one.
- **Legacy forms**: Search all `.info` files for "still work", "legacy", "deprecated" and "compatibility", and give each such form a deprecated warning (see [Conventions](../../CLAUDE.md#conventions)).
- **Checks without a vanilla signal**: Confirm every new or changed check that vanilla doesn't exercise (new loaders, fields vanilla doesn't use, error sides vanilla never trips) with a scratchpad test mod: one deliberate mistake per check, plus the valid and legacy forms, which must give no report (or only the deprecated warning). Give the files a UTF-8 BOM, or tiger skips them, and give the mod its own localization file, or missing-loc reports don't appear.
- After each item type, run vanilla again, and explain every group that disappears. Run `cargo test -p tiger-lib` too: a new required field can break the upstream test mods under `tests/files/`, including their line-number assertions.
- **Scope uses outside the folder**: Before deciding that vanilla doesn't use a scope or list, search the whole game tree, and trust the vanilla run over the search (see [examples.md](examples.md#uses-outside-the-items-folder-120)).
- **Markers the docs make stale**: When a hunk documents a field, remove the validator's `// undocumented` and "docs say" markers for it, and search for other uses of a renamed field's old name, e.g. in `has_key` conditions (see [examples.md](examples.md#stale-markers-when-the-docs-catch-up-120)).
- **Generated modifiers**: Don't require modifier formats for newly documented generated modifiers; tiger checks them when a modifier is used (see [examples.md](examples.md#generated-modifier-formats-only-when-used-120)).

### 6. Accept

1. **Vanilla**: Every group that is new compared with V_old is either a tiger gap (fix it) or a new vanilla bug (keep it). The user approves the classifications. Compare exact keys too: a group can change only because a language was added to the loc, or because the new version adds content with the same old problem.
   - Trace every changed group, grown or shrunk, through the runs with `scripts/timeline.py`. A "vanilla" group comes from new or changed vanilla content checked the way the old tiger did. A "tiger" group changed in a step, whose commit must explain it; every drop needs such an explanation, never an unexplained relaxation.
   - "Vanilla" groups can still be update gaps, where the new vanilla needs engine knowledge tiger lacks. Check those families one by one: unknown sounds, GUI fields, datafunctions and cast datatypes, defines and icons (1.20: the sounds table, three GUI properties, the `int_bool` cast). See [examples.md](examples.md#vanilla-only-groups-can-be-update-gaps-120).
   - Write the classification to a doc for the user (1.20: `tmp/vanilla-classification.md`): the fixes it proposes, the new vanilla bugs, the pre-existing tiger behaviour that new content trips, and the explained drops.
2. **Mods**: The user runs the mods without the sections that hide false positives from the outdated tiger. Explain every remaining report. The goal is that those sections can be deleted.

### 7. Install

Back up the user's installed tiger binary, then copy `target/release/ck3-tiger` over it.

## Scripts

- `scripts/groups.py <run>.json <out>.txt`: Report counts by severity and key, and by group.
- `scripts/compare_runs.py <old>.json <new>.json [--min N] [--area <prefix>]`: Group count changes between two runs, largest first.
- `scripts/timeline.py <V_old>.json <V_new0>.json <step>.json ... [--up|--down] [--key REGEX]`: For each group that changed between the first and the last run, whether vanilla or a tiger step changed it, and the steps where its count changed. Loading many runs takes a minute or two.
- `scripts/defines_diff.py <old-root> <new-root> [<table>]`: Defines added and removed between two game versions, compared with tiger's defines table.
- `scripts/toplevel.py <file>... [--field <name>]`: How many items use each top-level field, or the values of one field. Its `items()` function parses a file for ad hoc surveys of vanilla usage.
