# How tiger works

An overview for game updates. The steps named here are those of the [update-ck3 skill](../skills/update-ck3/SKILL.md).

## Crates

| Crate | Role |
| --- | --- |
| `tiger-lib` (root, `src/`) | Almost all the code. Built once per game through a cargo feature (`ck3`, `vic3`, ...), so `#[cfg(feature = "ck3")]` is everywhere. That's why other games are checked one at a time: a combined check turns on two features at once. |
| `tiger-tables` | Data that is shared and generated: the `Item` enum (`item.rs`) and the datatypes from the game's `data_types` logs (`ck3/include/*.rs`, step 3.2). |
| `ck3-tiger`, `vic3-tiger`, ... | Thin binaries: `GAME_CONSTS` (name, version (step 3.1), Steam id) plus argument parsing from `tiger-bin-shared`. |
| `utils` | Generators: `munch-data-types` (step 3.2) and `munch-script-docs` (step 3.7). |

## One run, end to end

1. **Fileset** (`fileset.rs`): collects every file from vanilla plus the mod(s), resolving overrides (a mod file replaces a vanilla file with the same path, `replace_path`, ...).
2. **Parse** (`parse/`, `parse/pdxfile.rs`): each script file becomes a `Block`, a tree of `key = value` / `key = { ... }` items. Every value is a `Token`, which carries its `Loc` (file:line:col). That's how reports point at a location. Reader exports (`@:define` and similar) are expanded here.
3. **Load** (`Everything::load_all`):
   - Most item kinds go into the generic **`Db`** (`db.rs`). Each kind registers a loader with `inventory::submit! { ItemLoader::Normal(GameFlags::Ck3, Item::X, X::add) }`. The loader runs over the `Item::X.path()` folder (for example `common/religion/rites`) and stores each top-level key with its block.
   - A few kinds have their own structs in `Everything`: events, characters, titles, history, localization, ...
   - After loading, `add_subitems` lets an item register items it defines inside itself. Example: a tenet defines `DoctrineParameter` keys.
4. **Validate** (`Everything::validate_all`): every item's `validate()` runs in parallel (rayon). That's why the output order and a few reports vary between runs; the skill's hasher patch plus `RAYON_NUM_THREADS=1` makes runs byte-identical.
5. **Report** (`report/`): each report has a severity (Tips, Untidy, Warning, Error, Fatal), a confidence (Weak/Reasonable/Strong) and an `ErrorKey`. The `.conf` filter (a mod's `ck3-tiger.conf`) decides what gets printed.

## Validating one item: `Validator`

`validator.rs` walks a block field by field. Each call names an allowed field and checks its value:

```rust
let mut vd = Validator::new(block, data);
vd.field_item("icon", Item::RiteIcon);          // value must be a key of that item type
vd.field_validated_block("is_shown", |b, data| validate_trigger(b, data, &mut sc, Tooltipped::No));
vd.advice_field("terrain", "removed in 1.20");  // known but obsolete, reported as advice
```

- Any field nobody asked about is reported as unknown when `vd` is dropped. That's the main source of "new field" false positives after an update.
- `data.verify_exists(Item::X, token)` is the cross-reference check: "does the item this field names exist?"
- Step 5 writes and updates exactly these validators, in `src/ck3/data/*.rs` (or `src/data/*.rs` for loaders shared between games), from the `.info` files and vanilla usage. [info-mapping.md](info-mapping.md) says which file validates which `.info`.

## Script: scopes, triggers, effects

CK3 script is typed by **scope**: character, title, faith, rite, ... These are the `Scopes` bitflags in `scopes.rs` (step 3.3). A `ScopeContext` (`context.rs`) tracks which scope type `root`, `this`, `prev` and saved scopes hold at each point.

`validate_trigger` (`trigger.rs`) and `validate_effect` (`effect.rs`) look up every key in the game's **tables** in `src/ck3/tables/`:

| Table | Answers | Step |
| --- | --- | --- |
| `targets.rs` `SCOPE_TO_SCOPE` / `SCOPE_PREFIX` | `faith.rite`, `rite:foo`: what scope you get, and from which scope it's allowed | 3.4 |
| `triggers.rs` `TRIGGER` | `has_tenet = x`: which scopes it's valid in and how the right side is checked (`Item(Tenet)`, `Scope(Rite)`, `ScopeOrItem`, `CompareValue`, `Block(...)`, ...) | 3.7 |
| `triggers.rs` `TRIGGER_COMPLEX` | the call form `trigger(arg)` used in script values and targets | 3.7 |
| `effects.rs` | the same for effects; complex ones get a function in `ck3/effect_validation.rs` | 3.7 |
| `iterators.rs` | `any_/every_/random_/ordered_X`: input and output scopes; extra fields go in `validate_inside_iterator` (`validate.rs`) | 3.7 |
| `on_action.rs` | on_actions the engine fires, with their scopes (written in script syntax) | 3.5 |
| `modifs.rs` | modifier names, how they combine, and loc keys | 3.6 |
| `defines.rs` | the defines and their types | 3.9 |
| `misc.rs`, `sounds.rs`, `localization.rs` | `COMMON_DIRS`, DLC features and music, sounds, loc tables, misc lists | 3.8, 3.10 |

A key that isn't found gives "unknown trigger/effect". A table entry with the wrong scope or argument type gives "wrong scope" / "expected X". These are the false positives a game update causes, and fixing them is the mechanical update (step 3).

`*_REMOVED` tables and `Removed("<version>", "use y")` entries give the "this was removed, use y" message instead of a plain "unknown".

## Datatypes and localization

- Localization is checked by `localization.rs`. `[Rite.GetName]` and similar are datatype calls, checked against the generated `tiger-tables/src/ck3/include/datatypes.rs` (step 3.2).
- Items also say which loc keys they need, for example `<rite>_name`, `<rite>_desc`, and the special modifier loc keys in `SPECIAL_MODIF_LOC_TABLE` (step 3.6).

## How the parts of an update fit

- **Mechanical update** (step 3): the engine's vocabulary (scopes, targets, triggers, effects, iterators, on_actions, modifiers, datatypes, folders, defines). It's mostly table edits, with the game's logs as the reference.
- **Game file changes** (step 5): the content folders, with a loader and validator per item kind, written by hand from `.info` files, vanilla usage and, when unsure, the binary (`strings binaries/ck3`).
- **Acceptance** (step 6): vanilla and mod runs, each remaining report classified as either a tiger gap or a real bug.

## Where to look in the code

- `src/everything.rs`: load and validate order, and the item databases.
- `src/validator.rs`: field helpers (`field_*`, `req_field`, `advice_field`, `replaced_field`, `multi_field_*`).
- `src/ck3/data/rites.rs`, `tenets.rs`: small, complete examples of a loader.
- `src/trigger.rs` `validate_trigger_internal` / `match_trigger_bv`: how a table entry turns into checks.
