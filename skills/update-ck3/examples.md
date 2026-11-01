# Examples from Past Updates

Lessons from past updates, for situations that are likely to come up again.

## Docs Are Sometimes Wrong (1.20)

- `add_saint` is documented with `target`, but the game takes `character`. An in-game test showed that `target` is rejected.
- `ai_start_best_war` is documented with `max_accumulative_strength`, but the binary only contains `max_cumulative_strength`.
- Both got ``vd.advice_field("...", "docs say `...` but it's `...`")``, following upstream.
- The `history/faiths` `.info` text says rite entries go in a `rites = { ... }` block, but its example shows them directly in the date block. An in-game test showed that only the `rites` block works, so the example is misleading. A docs example and its text can disagree; test before accepting either.

## Testing Undocumented Forms in Game (1.20)

- The 1.20 docs list doctrine `parameters` as bare names, but three vanilla doctrines still use `name = yes`. Parsing without errors doesn't show that the parameter is registered, so the user ran console tests that write `debug_log` lines to `game.log`:

  ```text
  effect if = { limit = { rite:<rite> = { rite_has_parameter = <param> } } debug_log = "yes" } else = { debug_log = "no" }
  ```

- Always include controls: the setup holds (the rite exists and has the doctrine), a documented form gives yes, and an absent value gives no. Here the `= yes` form gave yes, so it is accepted with an `// undocumented` comment.

## Testing Whether the Parser Knows a Key (1.20)

- To test whether a database file accepts a key, add it to a copy of the vanilla file in a test mod, together with a made-up control key (e.g. `tiger_test_bogus_key = yes`). On load, `error.log` reports unknown keys as `Unexpected token: <key>`. If the control is reported and the tested key isn't, the parser knows it.
- In 1.20 this showed that faiths reject `piety_icon_group` and `tenet_background_icon` (despite a stale comment in the religion `.info`) and `holy_order_maa`, and that a faith without holy sites loads and works.

## Testing a New Loader with Deliberate Mistakes (1.20)

- A new loader that produces no vanilla reports may simply not be running. Write a small test mod (a copy of `mod-test-vanilla/descriptor.mod` plus one file in the new folder) with one deliberate mistake per check, run tiger on it without `--show-vanilla`, and confirm that each mistake gives exactly one report and that valid forms give none.
- For `history/faiths` this found that an empty `rite = ""` was reported as "unknown value", because the value validator was never told the value was accepted (`maybe_is("")` fixes that).

## Check Fallbacks Before Reporting Missing Loc (1.20)

- `set_relation` reasons may have a `<reason>_corresponding` key for the target's side. Without it, the game shows `<reason>` with the characters swapped, which is fine for symmetric texts. So the missing key isn't reported, and a `TODO` comment records the verified behaviour.

## Usage Summaries Hide Rare Cases (1.20)

- A summary of the top vanilla values for `force_set_personal_tenet` showed only scopes, so it was made scope-only. A vanilla run then showed bare tenet keys elsewhere, and it became `ScopeOrItem`. Before making an argument stricter, grep vanilla or do a run.

## Largest Report Groups (1.20)

- New history fields (`rite = X` in characters and provinces): about 80k reports.
- Doctrine `parameters = { a b }` as bare values instead of `a = yes`: about 16k reports.
- Laws moved to `law_groups`, faiths to `faith_types`, and new holy site type fields: several thousand each.
- Check for similar format changes early, since they hide smaller problems.

## Format Changes Can Silently Disable Checks (1.20)

- Tiger found a doctrine's group through the group's `doctrine_types` list. 1.20 removed that list and put `doctrine_group_type` on each doctrine instead, so the "two doctrines from the same group" check silently stopped reporting. Likewise, the `unreformed` property read `parameters`, which moved to `special_parameters`.
- When a field is removed or moved, grep the code for every place that reads it, not only the validator of that item type.
- Groups that disappear compared with V_old are not always fixes. Check whether they are real bugs that tiger stopped finding.

## Claims That Something Is No Longer Required (1.20)

- A reference update claimed that faith `_desc` and law `_effects` loc were "no longer required since 1.20". V_old showed that the faith keys were already missing in 1.19, and in-game tests showed empty texts without a fallback, so both checks stayed. Verify such claims against V_old and in game.

## Dead Checks (1.20)

- The bookmark check that compares characters with their history compared a `faith` field that bookmarks never had (they use `religion`), so it never fired. When a check never reports anything in vanilla, make sure the fields it reads exist.

## Docs Rules Contradicted by Vanilla (1.20)

- The puppet types `.info` says every action should be used by only one puppet type, but vanilla shares 82 of its 98 actions. The user decided to drop the check. Holy site `_effects` loc, in contrast, is "used if exists" in the docs and was verified in game.

## Searching for Unknown Field Names Is Not Enough (1.20)

- The first pass over the remaining changed `.info` files only looked for documented field names that the code never mentions. For activity types this missed `scope:province` added to about 30 triggers and effects, `scope:special_option` in `province_score`, three new province filters, phase `pulse_actions`, and `chance_of_no_event` changing from a number to a script value. The user found it by diffing the file; fixing it removed 214 vanilla false positives. Go through every changed line of a diff.
- New checks for fields that vanilla doesn't use were not confirmed with a test mod at first either. Without a vanilla signal, only a deliberate mistake shows that a check works.
- To test whether a named scope is defined, use it for real (`scope:x = { ... }`, `scope:x = flag:y`, `scope:x > 1`). `exists = scope:x` never warns, so a test that uses it shows nothing.

## Renamed Fields in the Binary (1.20)

- The buildings `.info` swapped every `*_faith_modifier` for `*_rite_modifier`. `strings` on the binary showed `province_rite_modifier` and `duchy_capital_county_rite_modifier` but none of the faith forms, so the old forms became `replaced_field`. `county_rite_modifier` was missing too, but only because the linker stores a string that is the tail of another one (`duchy_capital_county_rite_modifier`) once. A missing key is no proof when it is a suffix of a key that is present.

## Lifting a Ban Exposes New Script (1.20)

- Allowing lease liege shares for every theocratic lease (not only `theocracy_lease`) validated script that tiger had never looked at, and gave 3 new scope reports: vanilla uses character triggers on the share's root, which the docs don't give. It got the same root as the neighbouring `ruler` share, for which vanilla does the same. After lifting a ban or a `req_field`, read the new reports before calling them vanilla bugs.

## Unchanged Docs Contradicting Tiger (1.20)

- Both `.info` versions say casus belli `on_*_desc` use "the same scope as the effects", but tiger gave them the trigger scopes, without `scope:war`. Vanilla uses `scope:war` there, and the fix removed 306 V19 "unknown datafunction war" loc reports. Decision `advice` is a "collection" of hints in both versions, but tiger allowed one `per_title_hint`, which gave 20 V19 reports. A docs line that didn't change can still contradict tiger.

## Docs That Stop Mentioning a Form (1.20)

- The domicile types `.info` called `rename_window = none` the default in 1.19, and no longer mentions it in 1.20; the dynasty perks `.info` dropped `trait = ...`; the governments `.info` moved the `religious` and `administrative` rules to `mechanic_type`. None of these is used in vanilla. At first they stayed accepted with a `TODO: verify`, which left 13 such TODOs. The user reversed the rule: a form that both the docs and vanilla dropped is most likely gone, or was a docs error, so tiger stops accepting it, unless its code already had a comment or TODO saying otherwise. Where the docs or vanilla point to a successor (`mechanic_type`), `replaced_field` names it. The vanilla run didn't change, which confirmed that nothing used them.

## Shared Docs, Shared Validator (1.20)

- The 1.20 docs say culture pillars and traditions share the structure of `_cultural_traits.info`. Tiger's two validators had drifted apart: pillars validated `ai_will_do` with a breakdown, lacked `can_pick_for_hybridization` and `faith_character_modifier`, and accepted `desc` only for ethos; neither had `icon`. One `validate_culture_trait` plus the per-type extras fixed all of it.

## Uses Outside the Item's Folder (1.20)

- Tiger defined the casus belli effect lists `attackers` and `defenders` as titles. A search of the casus belli folder found no uses, but the vanilla run showed 3 V19 reports in `tgp_faction_events`, an event called from a casus belli, which runs character effects on `list = attackers`. Scope contexts flow into the events that script fires, so search the whole tree.

## Splitting a Scope Context Surfaces Hidden Reports (1.20)

- Tiger reports a missing named scope once per scope context. Casus belli descs and `should_invalidate` shared one context, so the desc reports hid the 6 `scope:war` uses in `should_invalidate`. Giving the descs their own context made 3 new reports appear. After splitting a shared context, rerun vanilla.

## Effect Scopes Flow into Events (1.20)

- The 1.20 scheme types `.info` gives the scheme effects (`on_phase_completed`, `on_monthly`, ...) `scope:scheme`, `scope:owner` and `scope:target`; tiger rooted them in the scheme with no named scopes. Adding the scopes removed about 1,400 vanilla reports, most of them in the scheme events and their loc, which inherit the scopes of the effect that fires them. A missing scope in an item's effects shows up far from the item, so measure such a fix with a vanilla run, not a folder search.

## Stale Markers When the Docs Catch Up (1.20)

- Upstream's `advice_field("x", "docs say`x` but it's `y`")` and `// undocumented` markers become wrong once the docs are fixed (1.20: `title_maa_setup`, `base_maximum_success_chance`, `modifier_named_sets`, `dna_modifier`, `investor_cooldown`, and many `// undocumented` fields). When a hunk documents a field, look for these markers in the validator and remove them; the old name becomes an ordinary unknown field, or `replaced_field` if it was once real (`vassal_contract`).

## Generated Modifier Formats Only When Used (1.20)

- The vassal stances `.info` added `<key>_prestige_contribution_add/mult` to the generated modifiers. Requiring their formats for every stance, like the levy and tax ones, gave 14 vanilla reports: vanilla doesn't use them and defines no formats. Tiger already checks such formats only when a modifier is used (`lookup_modif`), as its comment on the `_ai_*` ones says. Don't extend the always-required list with new generated modifiers.

## Dead Checks on Old Field Names (1.20)

- Governments required `<key>_vassals_label` only `if block.has_key("vassal_contract")`, a field vanilla had replaced with `vassal_contract_group` long before, so the check never fired. When a field is renamed, search the validator for other uses of the old name, such as `has_key` or `get_field_value` conditions.

## Vanilla-Only Groups Can Be Update Gaps (1.20)

- The old tiger reported these groups on the new vanilla too, so they looked like new vanilla content checked the old way. Three were update gaps: 20 CE3 sound events missing from the sounds table (refreshed from an object explorer dump), the GUI properties `wrap_basedon` and `overflowed_items_visibility` and CK3's `wrap_length` (binary strings next to `wrap_count`), and the `(int_bool)` cast (listed with the base datatypes in the binary). The rest of those groups were vanilla bugs or pre-existing tiger behaviour.

## Engine Fallbacks for Icons (1.20)

- 1.20 removed 82 doctrine icons, and tiger reported each missing file. In game, a doctrine without an icon of its own shows its doctrine group's icon (the faith window draws `DoctrineType.GetIcon`), and a group without an icon shows nothing wrong. A `_default.dds` in the same folders and a `/_default.dds` binary string suggested another fallback, which the test didn't confirm. When vanilla drops many files of one kind at once, look for a fallback and test it before reporting them.

## The Binary Shows Presence, Not Absence (1.20)

- GUI property names such as `visible` or `texture`, and loc `#TOOLTIP:` tags, aren't standalone strings in the CK3 binary, although they work: they live in shared engine code. New properties such as `wrap_basedon` were there. Only a hit counts as evidence.

## Stale Tables Behind Old Markers (1.20)

- `CUSTOM_RELIGION_LOCAS` existed twice: in `tiger-tables` (for `Faith.Custom` in loc) and in `src/ck3/tables/misc.rs` (for faith definitions). Only the second had been updated since 1.18, so loc using 18 terms was reported. It is now one table, in `tiger-tables`. `DLC_MUSIC` stopped at 1.12. Most other tables behind old markers were complete.
- A stale table gives false positives only where a mod uses values vanilla doesn't, and hides errors where the game dropped a value, so a vanilla run shows neither. Compare each table with its source.
- Docs lists can be incomplete too: the `create_title_and_vassal_change` docs list 7 "valid types for use in script", but the same text names others, and vanilla uses 17 (`independency` 100 times). Tiger keeps the full list from the `recent_history` docs.

## Rewriting Branch History

- To fold fix-up commits into earlier ones, or to fix commit messages before pushing: back up the branch, rebase with a scripted todo list, then check that `git diff <backup> HEAD` is empty and run clippy on every rewritten commit. Ask the user before touching the staging area or stashing.
