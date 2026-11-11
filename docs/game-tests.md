# CK3 game tests

Tests for questions that the docs, vanilla and the binary don't settle, run in game with the debug console. Each test is kept with its result, so it can be repeated after a later update; the open ones have no result yet. Tests are grouped by the occasion that raised them. T numbers are never reused, and [follow-up.md](follow-up.md) refers to them.

Each test gives the files to create in the test mod, `tmp/test_mod` in the tiger checkout (outside git), as `zz_tiger_test*` files with a UTF-8 BOM. Up to T39 the files went into the Unop instead. Each batch gets its own file prefix and event namespace (`zz_tiger_test2`, `zz_tiger_test3`, ...). Include a control in every test: a form that must work and one that must fail, so a silent result means something.

## Setup

- The test mod has its own `descriptor.mod`. The launcher file `tmp/tiger_test.mod` (the same, plus `path` to the mod) goes into `~/.local/share/Paradox Interactive/Crusader Kings III/mod/`.
- Launch CK3 with only the test mod enabled, with `-debug_mode` (for the console).
- Start a game as a landed ruler, a count or above.
- Logs: `~/.local/share/Paradox Interactive/Crusader Kings III/logs/error.log` and `debug.log` (`debug_log` and `debug_log_scopes` write to `debug.log`).
- First check: search `error.log` right after loading for `zz_tiger_test`.

Cleanup afterwards: delete the files of the tests that have a result, keeping `descriptor.mod`. A run of tiger on the test mod (`ck3-tiger --game <game-dir> tmp/test_mod/descriptor.mod`) should report exactly the forms under test, which shows the files say what the tests mean.

## CK3 1.20: `TODO: verify` markers (T1 to T20)

Questions about `TODO: verify` markers in the code that the 1.20 update touched. The first batch (T2 to T10, T12, T14 to T16, and the first runs of T1, T11 and T13) was applied in `4a0c557c`, which describes the results; the files of that batch were not kept. The later results were applied in `f063dca0` and the commits they name.

### T1. Important action `<key>_unimportant` loc

- Tiger: `important_actions.rs` requires `<key>_unimportant` when `unimportant` is set (`TODO: verify`). 3 vanilla diarchy actions lack it.
- First run (1.20.1.2): the action never appeared, so the test was inconclusive.
- Files:
  - `common/important_actions/zz_tiger_test_actions.txt`:

    ```paradox
    zz_tiger_test_action = {
     type = alert
     icon = action_being_raided
     check_create_action = {
      try_create_important_action = {
       important_action_type = zz_tiger_test_action
      }
     }
     unimportant = { always = yes }
     effect = { }
    }
    ```

  - `localization/english/zz_tiger_test_l_english.yml`, with `l_english:` and `zz_tiger_test_action: "Tiger test action"`, `zz_tiger_test_action_desc` and `zz_tiger_test_action_click` (no `_unimportant` key).
- Check:
  1. Wait at least a month, since the game may run `check_create_action` only periodically. Look for "Tiger test action" in the HUD, including any collapsed or hidden list of unimportant actions.
  2. If it never appears, change the file to `unimportant = { always = no }` and restart: if it shows up now, unimportant actions are hidden, and the loc is not shown anywhere visible.
  3. If it appears as unimportant, hover it: does the tooltip show the raw key `zz_tiger_test_action_unimportant`, or nothing?
- Raw key: keep the requirement and drop the TODO; the 3 vanilla reports are Unop bugs. Nothing, or never visible: make it optional (`localization.suggest`), and the 3 reports go away.
- **Result (1.20.1.2):** the tooltip shows the raw key `zz_tiger_test_action_unimportant`. The requirement stays and the TODO is gone (`f063dca0`); the 3 vanilla diarchy actions are real bugs (low).

### T2. Great project `icon`

- Tiger: `greatprojects.rs`, `icon` is required; the short `icon = <path>` form is accepted (TODO).
- Files (not kept): `common/great_projects/types/zz_tiger_test_great_projects.txt`.
- Check: `error.log` for `zz_tiger_test_gp_no_icon` and `zz_tiger_test_gp_value_icon`. A missing texture `great_projects/zz_tiger_test_gp_no_icon.dds` means the game still falls back to the key (no such file exists); an error about a missing icon or fallback entry means it doesn't. Then the great project planning list: does "Tiger test project (path icon)" show the Great Wall icon?
- Missing-icon error: keep `req_field`. Missing texture for the key: go back to the key as the default icon. Path icon works: drop its TODO.
- **Result (1.20.1.2):** "has no icons defined, or there is no fallback icon (with empty trigger)" for the no-icon project; no error for the path icon. Kept `req_field`, dropped both TODOs (the value form is marked `// undocumented`).

### T3. Province terrain in other files

- Tiger: `prov_terrain.rs` reads terrains only from `*province_terrain.txt` and properties only from `*province_properties.txt` (TODO); it ignores `zz_tiger_test_terrain.txt` entirely.
- Files (not kept): `common/province_terrain/zz_tiger_test_terrain.txt`.
- Check: Paris (province 2333): is its terrain Desert? Is it under winter all year (`fixed_winter_override = 1.0`)? Also `error.log` for `2333` or `province_terrain`.
- Both work: rewrite both loaders to read every file and take assignments and blocks from any file (F12). Neither: drop the TODO.
- **Result (1.20.1.2):** Paris is desert and under harsh winter in September, no errors: the game reads both from any file. TODOs now say "read every file"; follow-up F12 is confirmed.

### T4. Scripted character template fields

- Tiger: `character_templates.rs` accepts all `create_character` fields (TODO), and `random_faith_in_religion` takes a religion key or a faith scope (TODO); `= catholic` is an error.
- Files (not kept): `common/scripted_character_templates/zz_tiger_test_templates.txt`.
- Check, in the console:
  - `effect create_character = { template = zz_tiger_test_template_fields location = root.capital_province }`
  - `effect create_character = { template = zz_tiger_test_template_religion location = root.capital_province }` Then `error.log`, and the two new characters: female with high fertility and your ethnicity (1st), a Christian faith (both).
- Errors for `fertility`, `ethnicity` or `dynasty = default`: drop those fields from templates. `random_faith_in_religion`: keep whichever key form works (maybe both).
- **Result (1.20.1.2):** "Failed to find a valid event target link" for both `catholic` and `christianity_religion`, so `random_faith_in_religion` takes only a faith scope (tiger now uses `field_target`). No errors for `fertility`, `ethnicity`, `template_character` or `dynasty = default`; dropped that TODO. Both characters failed for lack of a faith ("Invalid faith in HandleRite"), so their looks were not seen.

### T5. Tax obligation contribution fields

- Tiger: `tax_slots.rs` accepts subject contract contribution fields as script values, with guessed scopes (TODO).
- Files (not kept): `common/tax_slots/obligations/zz_tiger_test_obligations.txt`.
- Check: `error.log` for `zz_tiger_test_obligation`, `tax_factor`, `prestige`, `min_piety`. (The obligation isn't offered anywhere, so this only tests parsing.)
- Errors: go back to numeric `tax_factor`/`levies_factor` only.
- **Result (1.20.1.2):** No errors: the contribution fields parse as script values. Their scopes stay a guess.

### T6. Province effect `secondary`

- Tiger: `prov_effects.rs` accepts `secondary` (TODO); `is_secondary` is an unknown field.
- Files (not kept): `gfx/map/province_effects/zz_tiger_test_province_effects.txt`.
- Check: `error.log` for `zz_tiger_test_effect_secondary` / `is_secondary` / `secondary`.
- If `is_secondary` is the one accepted: swap them.
- **Result (1.20.1.2):** "Unexpected token: is_secondary"; no error for `secondary`. Dropped the TODO.

### T7. Music `subsequent_playback_chance`

- Tiger: `src/data/music.rs` requires exactly 3 numbers (TODO); both test groups are errors.
- Files (not kept): `music/in_game/zz_tiger_test_music.txt`.
- Check: `error.log` for `group_zz_tiger_test_two` / `four` / `subsequent_playback_chance`.
- No errors: allow any count (CK3 only, since the validator is shared).
- **Result:** not run. The 1.20 docs allow any number of chances, and vanilla agrees, so tiger accepts any count (`b9389eab`).

### T8. Memory type without `duration`

- Tiger: `memories.rs`, an error with the info "the duration is only 1 day" (TODO on the info; the docs say it logs an error and defaults to `DEFAULT_MEMORY_DURATION_YEARS`).
- Files (not kept): `common/character_memory_types/zz_tiger_test_memories.txt`.
- Check: `error.log` at load; then `effect create_character_memory = { type = zz_tiger_test_memory }`, open your character's memories, and see whether the memory is gone after a few days.
- Keep the error either way; fix the info text to the duration seen.
- **Result (1.20.1.2):** "missing 'duration' property, defaulting to 10 years"; the memory stayed for over a week. The error stays; its info now says so.

### T9. Culture tradition dynamic `desc`

- Tiger: `cultures.rs` expects a loc key for `desc` (TODO; both docs versions say "dynamic desc").
- Files (not kept): `common/culture/traditions/zz_tiger_test_traditions.txt`.
- Check: `effect culture = { add_culture_tradition = zz_tiger_test_tradition }`, then open your culture: does the tradition's description read "Tiger test tradition: dynamic description"?
- It does: accept dynamic descs for traditions and ethos (`validate_desc`).
- **Result (1.20.1.2):** The dynamic description is shown. Culture traditions and pillars now accept a dynamic `desc` rooted in the culture (as the docs say).

### T10. Nested `scripted_animation` in `triggered_animation`

- Tiger: `scripted_animations.rs`, unknown field (TODO; both docs versions allow it).
- Files (not kept): `common/scripted_animations/zz_tiger_test_animations.txt` and `events/zz_tiger_test_events.txt`.
- Check: `effect trigger_event = zz_tiger_test.1`. Does the portrait show `rage` (works) or `idle`? Also `error.log` for `zz_tiger_test_scripted_animation`.
- Rage: accept the field (`field_item(Item::ScriptedAnimation)`).
- **Result (1.20.1.2):** the portrait showed `rage`: tiger now accepts the field.

### T11. Code on_action scopes (remaining parts)

- Tiger: `src/ck3/tables/on_action.rs` guesses (`# TODO: verify scope type`):
  - `on_rite_updated`: `changed_tenets_or_doctrines` and `consume_rite_edit_token` as bools.
  - `on_rite_created`: `new_rite` as a rite (vanilla uses it once).
  - `on_artifact_durability_low`: root character ("verify the doc"; the vanilla comment says "root = the artifact owner").
- First run (1.20.1.2): `death_reason` and `tradition` were settled; the rite edit and the artifact durability on_actions were not seen firing.
- File `common/on_action/zz_tiger_test_on_actions.txt`:

  ```paradox
  on_rite_updated = {
   effect = { debug_log = "zz_tiger_test on_rite_updated" debug_log_scopes = yes }
  }
  on_rite_created = {
   effect = { debug_log = "zz_tiger_test on_rite_created" debug_log_scopes = yes }
  }
  on_artifact_durability_low = {
   effect = { debug_log = "zz_tiger_test on_artifact_durability_low" debug_log_scopes = yes }
  }
  ```

- Check, then `debug.log` for "zz_tiger_test":
  - `effect add_piety = 5000`, then edit your faith's rite in the religion window and confirm the change (on_rite_updated).
  - Create a new rite in the religion window (on_rite_created): is `new_rite` among the saved scopes?
  - Bring an artifact of yours to a durability at or below `ARTIFACT_LOW_DURABILITY` (20) but above 0, e.g. `effect random_character_artifact = { add_durability = -40 }` on one with 60, repeating if needed (a large negative value may destroy it instead).
- Fix each scope type to what is logged and drop its TODO.
- **Result (1.20.1.2):** `on_rite_created`: root is the character, and the saved scopes are only `old_faith` and `old_rite`, so tiger's `new_rite` is gone. `on_artifact_durability_low`: root is the owner and `scope:artifact` is set (it fired once per artifact), so tiger adds it. `on_rite_updated` didn't fire (editing a rite needs more than piety); it is taken to match its vanilla comment, like `on_rite_created`. Applied in `f063dca0`.

### T12. `set_faith_name` / `set_rite_name` without `name`

- Tiger: `effect_validation.rs` requires `name` (TODO).
- Check: `effect faith = { set_faith_name = { desc = zz_tiger_test_faith_desc } }` and `effect rite = { set_rite_name = { desc = zz_tiger_test_faith_desc } }`: errors in `error.log`? Does the faith (rite) description change while its name stays?
- No error and it works: make `name` optional.
- **Result (1.20.1.2):** No errors, but without `name` the rite lost its name. `name` stays required, with a comment instead of the TODO. The faith was not visible.

### T13. Several blocks of the same kind in one on_action (retest)

- Tiger: `src/data/on_actions.rs` warns "not sure if multiple ... blocks work" for `first_valid`, `first_valid_on_action` and `random_on_action` (upstream TODOs). Several `events` and `on_actions` blocks were shown to work and no longer warn.
- First run (1.20.1.2): one entry fired for each of the three, which "merged" and "second block ignored" would both give. This retest makes the first block's only entry invalid.
- Files:
  - `events/zz_tiger_test_events.txt`:

    ```paradox
    namespace = zz_tiger_test
    zz_tiger_test.7 = {
     type = character_event
     hidden = yes
     immediate = { debug_log = "zz_tiger_test.7 fired" }
    }
    zz_tiger_test.8 = {
     type = character_event
     hidden = yes
     trigger = { always = no }
     immediate = { debug_log = "zz_tiger_test.8 fired" }
    }
    ```

  - Add to `common/on_action/zz_tiger_test_on_actions.txt`:

    ```paradox
    zz_tiger_test_log_9 = { effect = { debug_log = "zz_tiger_test_log_9 fired" } }
    zz_tiger_test_never = {
     trigger = { always = no }
     effect = { debug_log = "zz_tiger_test_never fired" }
    }
    zz_tiger_test_two_first_valid_blocks = {
     first_valid = { zz_tiger_test.8 }
     first_valid = { zz_tiger_test.7 }
    }
    zz_tiger_test_two_first_valid_on_action_blocks = {
     first_valid_on_action = { zz_tiger_test_never }
     first_valid_on_action = { zz_tiger_test_log_9 }
    }
    zz_tiger_test_two_random_on_action_blocks = {
     random_on_action = { 100 = zz_tiger_test_never }
     random_on_action = { 100 = zz_tiger_test_log_9 }
    }
    ```

- Check, one at a time, then `debug.log` for "zz_tiger_test":
  - `effect trigger_event = { on_action = zz_tiger_test_two_first_valid_blocks }` (.7 fires?)
  - `effect trigger_event = { on_action = zz_tiger_test_two_first_valid_on_action_blocks }` (log_9?)
  - `effect trigger_event = { on_action = zz_tiger_test_two_random_on_action_blocks }` (log_9? Run it a few times, since a random pick may not skip invalid entries.)
- The second block's entry fires: the blocks are merged; drop the warning for that kind. Nothing fires: the second block is ignored; turn the warning into an error, like `random_events`.
- **Result (1.20.1.2):** the second block's entry fired for `first_valid` and `first_valid_on_action`, and for `random_on_action` 8 times in 8. The warnings are gone (`f063dca0`). This result can't tell "merged" from "the last block replaces the earlier ones".

### T14. `highest_skill = prowess`

- Tiger: `triggers.rs` accepts any skill (upstream TODO; `highest_skill_including_prowess` exists).
- Check: `effect add_prowess_skill = 50`, then `effect if = { limit = { highest_skill = prowess } add_gold = 1000 }`. Gold added, or an error?
- Error or no gold: restrict `highest_skill` to the non-prowess skills.
- **Result (1.20.1.2):** Gold added: `highest_skill = prowess` works. Dropped the TODO.

### T15. Government fields from before 1.13

- Tiger: `government.rs` accepts `primary_cultures`, `ai.imprison` and `ai.start_murders` ("TODO: test whether this was removed in 1.13"); no reports on the test government.
- Files (not kept): `common/governments/zz_tiger_test_governments.txt`.
- Check: `error.log` at load for `zz_tiger_test_government`, `primary_cultures`, `imprison`, `start_murders`. Silence is inconclusive (the game doesn't always log unknown fields).
- Errors: remove those fields.
- **Result (1.20.1.2):** "Unexpected token" for `primary_cultures`, `imprison` and `start_murders`: removed from tiger, so they are unknown fields. The test government also got "Could not find the preregistered modifier type ... included in the define GOVERNMENT_TYPES?" (follow-up F18).

### T16. Vanilla-exercised TODOs (watch only)

These TODOs follow a form vanilla already uses, so a normal observer-mode run tests them. Search `error.log` for errors in these files:

- `common/activities/activity_types/*` (feast, tour): `scope:activity` in phase `is_valid` (`activities.rs`).
- `common/casus_belli_types/*` and `events/.../tgp_faction_events.txt`: the `attackers`/`defenders` lists, and `scope:war` in `should_invalidate` (`casusbelli.rs`).
- `common/lease_contracts/*`: character triggers on the share roots (`lease_contracts.rs`).
- `common/religion/tenet_types/*`: `selected_doctrines` in `piety_cost` (`tenets.rs`).
- `common/culture/eras/*`, `common/culture/innovations/*`: `men_at_arms` in `maa_upgrade` (`src/ck3/validate.rs`).
- `common/decisions/*`: `scope:puppeteer` in decision effects (`decisions.rs`), if any vanilla decision uses it. No errors after a long run: drop the TODOs.
- **Result (1.20.1.2):** 20 years of observer mode: no errors in error.log that relate to these TODOs (the errors in these files are unrelated vanilla bugs). Dropped them, keeping `// undocumented` markers.

### T17. `religion` in province history

- Tiger: `prov_history.rs` accepts `religion = <faith>` silently. The 1.20 province docs and vanilla use only `faith` and `rite`; the character docs call `religion` a "compatibility option".
- File `history/provinces/zz_tiger_test_provinces.txt` (redefines Paris with `religion` and no `rite`):

  ```paradox
  2333 = { #PARIS
   culture = french
   religion = orthodox
   holding = castle_holding
  }
  ```

- Check: start a game and look at Paris: what is its faith? Also `error.log` for `2333`, `religion` or `zz_tiger_test_provinces`.
- Orthodox: `religion` still works; warn Deprecated ("should use `faith` instead of `religion`"), as for characters. Catholic (the vanilla rite) or no faith: stop accepting it. If Paris keeps its vanilla buildings, the file merged with vanilla's instead of replacing it, and the test is void.
- **Result (1.20.1.2):** Paris had no Notre Dame, so the test entry replaced vanilla's, and Paris was catholic with no errors: `religion` has no effect. Tiger reports it as unknown (`f063dca0`).

### T18. Default icon for doctrines and doctrine groups without their own icon

- Tiger: `doctrines.rs` requires `gfx/interface/icons/faith_doctrines/<icon>.dds` and `faith_doctrine_groups/<group>.dds` (70 + 28 vanilla "file does not exist" reports). 1.20 removed 82 doctrine icons and added `_default.dds` to both folders (a book with a raised hand; the same file in both). The binary has a `/_default.dds` string. The docs say nothing about a fallback.
- No files needed (vanilla only; the Unop doesn't touch these icons).
- Check:
  1. Start a game as a Catholic ruler (Roman rite) and open the faith window.
  2. Doctrines: compare "Clerical Marriage: Disallowed" (`doctrine_clerical_marriage_disallowed`, no own icon in 1.20) with a doctrine that has one, e.g. the gender doctrine "Male Dominated" (a male symbol). Does the first show the book-with-hand icon, an empty frame, or a pink/missing texture?
  3. Doctrine groups: their icons are shown in the rite creation window and the personal beliefs window. Compare the gender group (`doctrine_gender`, no own icon) with the adultery group (`doctrine_adultery_men`, has one).
  4. `error.log`: search for `faith_doctrines` and `faith_doctrine_groups`.
- Book-with-hand icon, no error: the engine falls back to `_default.dds`; tiger should accept a missing icon when `_default.dds` exists in the folder (separate commit, not caused by 1.20). Empty or missing texture, or an error: keep the check; the reports are vanilla bugs.
- **Result (1.20.1.2):** "Clerical Marriage: Disallowed" shows the clerical marriage group's icon in the faith window, which draws `DoctrineType.GetIcon`; `_default.dds` never appeared. The rite creation window shows each doctrine's own icon, and the personal beliefs window shows nothing wrong for groups without one. A doctrine without an icon now falls back to its group's, and group icons are optional (`84307280`).

### T19. Audio event list for the sounds table

- Tiger: `src/ck3/tables/sounds.rs` lists the audio events "taken from the object browser, under audio events" and was last refreshed for 1.19. 1.20 vanilla uses 20 `event:/DLC/CE3/...` events that it lacks (43 reports). The bank files store the names compressed, so they can't be read out of the game files.
- Check (needs `-debug_mode` and the CE3 DLC enabled):
  1. In a running game, open the console and type `explorer` (the binary's help text: "Shows an object explorer window"; if the name differs, the console's autocomplete or `help` should show it).
  2. Pick the "Audio Events" table. If it has a filter box, filter on `DLC/CE3`.
  3. Either copy each path with the row's copy button ("Copy the sound effect's path to clipboard") into a text file, or take screenshots of the whole filtered list (scrolling as needed) and save them somewhere I can read, e.g. `~/Pictures/ck3-audio/`.
  4. Note the total number of rows of the unfiltered table, if it's shown: comparing it with the table's 900 or so entries tells whether other 1.20 events are missing too.
- Every one of the 20 events vanilla uses must appear in the list; that also checks the transcription. Then the table is regenerated with the CE3 events (one commit).
- **Result (1.20.1.2):** the Audio Events table was dumped to CSV: 21 new CE3 events (20 used by vanilla) and a wanderer lifestyle UI event, plus 13 MP1 and 1 CP4 music tracks the table already lacked. The table now holds all 959 events and snapshots (`b60ec080`).

### T20. Loc tooltip tags new in 1.20 (optional)

- Tiger: `COMPLEX_TOOLTIPS` in `src/ck3/tables/localization.rs` (marked 1.12.3) says, per `#TOOLTIP:<tag>,<value>` tag, whether the value is a database key (checked) or a runtime id (unchecked). A tag missing from the table gets its value checked as a loc key. 1.20's `tooltip_structs_l_english.yml` uses 20 tags the table lacks (7 new in 1.20) and no longer uses `GAME_FAITH_DOCTRINE`. Vanilla only passes `$DESC_KEY$` to them, so this matters for mods only. Upstream confirmed each entry in game; the binary doesn't list the tags.
- Files:
  - `events/zz_tiger_test_events.txt` (or add the event to T13's file of that name):

    ```paradox
    namespace = zz_tiger_test
    zz_tiger_test.20 = {
     type = character_event
     title = zz_tiger_test.20.t
     desc = zz_tiger_test.20.desc
     theme = default
     option = { name = zz_tiger_test.20.a }
    }
    ```

  - `localization/english/zz_tiger_test_l_english.yml` (UTF-8 BOM):

    ```yaml
    l_english:
     zz_tiger_test.20.t: "Tiger tooltip tags"
     zz_tiger_test.20.a: "OK"
     zz_tiger_test.20.desc: "#TOOLTIP:FAITH,catholic 1 FAITH (control)#!\n#TOOLTIP:GAME_FAITH_DOCTRINE,doctrine_monogamy 2 GAME_FAITH_DOCTRINE#!\n#TOOLTIP:DOCTRINE,doctrine_monogamy 3 DOCTRINE#!\n#TOOLTIP:TENET,tenet_dulia 4 TENET#!\n#TOOLTIP:RITE,roman_rite 5 RITE#!\n#TOOLTIP:HOLY_ORDER,ho_christian_knights_templar 6 HOLY_ORDER#!\n#TOOLTIP:PUPPET_TYPE,theological_agent_puppet 7 PUPPET_TYPE#!\n#TOOLTIP:LEASE_CONTRACT_TYPE,theocracy_lease 8 LEASE_CONTRACT_TYPE#!\n#TOOLTIP:SCHEME_AGENT_SLOT_TYPE,agent_hitman 9 SCHEME_AGENT_SLOT_TYPE#!\n#TOOLTIP:CONTRACT,overdue_taxes 10 CONTRACT#!\n#TOOLTIP:DOMICILE_BUILDING,camp_main_01 11 DOMICILE_BUILDING#!\n#TOOLTIP:SUBJECT_CONTRACT_GROUP,feudal_vassal 12 SUBJECT_CONTRACT_GROUP#!\n#TOOLTIP:SITUATION_TYPE,the_great_steppe 13 SITUATION_TYPE#!\n#TOOLTIP:SITUATION_SUB_REGION,steppe_west 14 SITUATION_SUB_REGION#!\n#TOOLTIP:SITUATION_PARTICIPANT_GROUP_TYPE,herders 15 SITUATION_PARTICIPANT_GROUP_TYPE#!\n#TOOLTIP:SITUATION_PHASE_TYPE,the_great_steppe,situation_steppe_white_zud_season 16 SITUATION_PHASE_TYPE#!\n#TOOLTIP:HOUSE_RELATION_LEVEL,feud 17 HOUSE_RELATION_LEVEL#!"
    ```

- Check: `event zz_tiger_test.20` in the console, then hover each numbered line. For each: does the tooltip describe the item (name, description), show an empty or generic tooltip, or show nothing? Line 1 is the control (FAITH is known to take a faith key). Also `error.log` for `TOOLTIP` and `zz_tiger_test`.
- A line whose tooltip describes the item: add the tag to the table with that item type. Nothing or empty: the value isn't that key (perhaps a runtime id); leave the tag out. Line 2 failing: remove `game_faith_doctrine` from the table.
- **Result (1.20.1.2):** lines 2 and 10 show raw keys ("Unlocalized text 'GAME_FAITH_DOCTRINE,doctrine_monogamy'", "... 'CONTRACT,overdue_taxes'"); 5, 6, 14 and 17 show nothing; the others describe the item. Applied in `f063dca0`: the item types, runtime ids for RITE, HOLY_ORDER, SITUATION_SUB_REGION and HOUSE_RELATION_LEVEL, and no `game_faith_doctrine`. The remaining tags are T37.

## CK3 1.20: Unop report classification (T21 to T25)

Questions that decide whether Unop reports are tiger gaps or real bugs, and how severe. Their follow-ups are in [follow-up.md](follow-up.md) and their rules in the Unop conf.

### T21. Liege entries dated when the liege has no living holder

**Question:** tiger reports `liege = X` in title history when X has no living holder on that date ("setting the liege will not have effect here"; 1,044 Unop/vanilla reports). Does the game skip the entry for good, or does it apply the title's latest liege entry at the start date, when X does have a holder? This decides whether 483 of these reports are medium (a wrong realm at a bookmark start) or low (harmless).

**Example:** `history/titles/k_aquitaine.txt`, `c_angouleme`: the last `liege` entry is `806.1.1 = { liege = "d_aquitaine" }`, when d_aquitaine's holder (Hunoald, 768) was dead. Later entries only change the holder. At 1066.9.15, c_angouleme is held by Foulques I (40106, from 1048.12.1) and d_aquitaine has a holder.

**Steps (vanilla, no mods needed):**

1. Start the 1066 bookmark with any character.
2. Open the county of Angouleme and its count, Foulques I.
3. Note his liege. Repeat at 867 if convenient (count Wulgrim I, 40080; duke Ramnulf II, 90128).

**Expected:**

- Foulques's liege is the Duke of Aquitaine: the game applies the latest entry at the start date, and the reports are low.
- Foulques is independent or someone else's vassal: the dated entry was lost, and the live reports are medium.
- **Result (1.20.1.2):** at the 1066 start, Foulques is the Duke of Aquitaine's vassal: the game applies the latest liege entry at a start date. Follow-up F53; the Unop rule is a tiger FP.

### T22. `$key|q$` escaping and literal casts in loc

- Tiger: in `'$key|q$'`, tiger substitutes the key's text unchanged, so an apostrophe in it ends the string literal ("Unexpected character", 16 vanilla reports on the PAM scripture keys, e.g. `[Glossary('$pam_psalm_24_1_english|q$','PSALM_24_1_GLOSS')]` with "the Lord's"). Tiger also rejects literal casts such as `['(int32)1']` (`gui/window_ledger.gui`, 3) and `'(CVector2f)0.0,0.04'` (`data_binding/tgp_data_bindings.txt`, 2). The question is whether `|q` escapes the text and whether `'(int32)N'` gives N.
- Files:
  - `events/zz_tiger_test_events.txt` (add to the file of that name):

    ```paradox
    zz_tiger_test.22 = {
     type = character_event
     title = zz_tiger_test.22.t
     desc = zz_tiger_test.22.desc
     theme = default
     option = { name = zz_tiger_test.22.a }
    }
    ```

  - `localization/english/zz_tiger_test_l_english.yml` (add the keys):

    ```paradox
     zz_tiger_test.22.t: "Tiger q escaping"
     zz_tiger_test.22.a: "OK"
     zz_tiger_test_apos: "The Lord's earth"
     zz_tiger_test_gloss: "Gloss text"
     zz_tiger_test.22.desc: "1 [Glossary('$zz_tiger_test_apos|q$','zz_tiger_test_gloss')]\n2 [Glossary('$zz_tiger_test_apos$','zz_tiger_test_gloss')] (control)\n3 ['(int32)7']"
    ```

- Check: `event zz_tiger_test.22`, then `error.log`.
  - Line 1 shows "The Lord's earth" with a tooltip, and line 2 is broken or logs a loc error: `|q` escapes, and tiger should apply it.
  - Line 3 shows 7: literal casts work, and tiger should accept them.
- **Result (1.20.1.2):** line 1 shows "The Lord's earth" with a tooltip, line 2 logs "Expected ','" and fails, line 3 shows 7. Follow-up F54.

### T23. `exists` guards inside a negated block

- Tiger ignores `exists = scope:x` anywhere inside `NOT`/`NOR` (`trigger.rs`, `if !negated`), so `NOR = { AND = { exists = scope:x  this = scope:x } }` still counts as an unguarded use of `scope:x` (conf rule L401, `councillor_spouse_learning_2603_scope_save_effect`). The question is whether the game evaluates `this = scope:x` there when `scope:x` doesn't exist.
- File `events/zz_tiger_test_events.txt` (add):

  ```paradox
  zz_tiger_test.23 = {
   type = character_event
   hidden = yes
   immediate = {
    if = {
     limit = { NOR = { AND = { exists = scope:zz_nope this = scope:zz_nope } } }
     debug_log = "zz_tiger_test.23 guarded passed"
    }
    if = {
     limit = { NOR = { this = scope:zz_nope } }
     debug_log = "zz_tiger_test.23 unguarded passed"
    }
   }
  }
  ```

- Check: `event zz_tiger_test.23`, then `debug.log` for both lines and `error.log` for `zz_nope`.
  - An error only for the unguarded block (the control): the guard works inside `NOR`, and tiger should honour `exists` inside a negated `AND`.
  - Errors for both: the game evaluates the whole block, and the reports are real.
- **Result (1.20.1.2):** "guarded passed" without errors; the unguarded control logs "Undefined event target 'zz_nope'". Follow-up F55.

### T24. `filter` in an ordinary `any_` iterator

- Tiger accepts `filter = { }` only in `any_in_de_facto_hierarchy` and `any_in_de_jure_hierarchy` (from vanilla usage; the docs mention it for no iterator). Vanilla also uses it in `any_vassal = { filter = { is_powerful_vassal = yes } count = all opinion = { ... } }` (`defenders_of_highgod_decision`, `00_major_decisions_east_europe.txt`) and once in `any_held_title`. The question is whether `filter` restricts the counted vassals there.
- File `events/zz_tiger_test_events.txt` (add):

  ```paradox
  zz_tiger_test.24 = {
   type = character_event
   hidden = yes
   immediate = {
    if = {
     limit = { any_vassal = { filter = { always = no } count = all always = no } }
     debug_log = "zz_tiger_test.24 filter excludes"
    }
    else = {
     debug_log = "zz_tiger_test.24 filter ignored"
    }
   }
  }
  ```

- Check: as a ruler with at least one vassal, `event zz_tiger_test.24`, then `debug.log` and `error.log` (`filter`, `zz_tiger_test`).
  - "filter excludes" (no vassal is counted, so `count = all` holds): `filter` works in `any_vassal`; tiger should accept it in all `any_` iterators.
  - "filter ignored", or an error about `filter`: the vanilla decision is a real bug (medium).
- **Result (1.20.1.2):** "filter excludes", no errors. Follow-up F56.

### T25. Plain `culture` inside a ruler iterator

- The ruler iterator docs (identical in 1.19 and 1.20) name the filters `culture_filter`, `faith_filter`, `government_type_filter`, `tier_filter`; their usage example shows the plain `government_type` and `tier`, and tiger accepts the plain `culture`/`faith` as filters. Vanilla writes `random_ruler = { ... culture = { save_scope_as = mystic_culture } }` (`learning_scholarship_events.txt:2714`), which only works if `culture` there is the link.
- File `events/zz_tiger_test_events.txt` (add):

  ```paradox
  zz_tiger_test.25 = {
   type = character_event
   hidden = yes
   immediate = {
    random_ruler = {
     limit = { is_ai = yes }
     culture = { save_scope_as = zz_ruler_culture }
    }
    if = {
     limit = { exists = scope:zz_ruler_culture }
     debug_log = "zz_tiger_test.25 culture is the link"
    }
    else = { debug_log = "zz_tiger_test.25 culture is a filter" }
   }
  }
  ```

- Check: `event zz_tiger_test.25`, then `debug.log` and `error.log` (`culture`, `zz_tiger_test`). "culture is the link": the plain names are not filters; tiger should stop accepting them as such (F46), and the 3 reports are tiger FPs. "culture is a filter" or an error: the plain names are still filters, and the vanilla event and `00_societal_traditions.txt:421` are bugs (low).
- **Result (1.20.1.2):** "culture is the link", no errors. Follow-up F46.

## CK3 1.20: "Unclear" Unop conf rules (T26 to T35)

These tests cover the Unop conf rules marked "Unclear" that had no test yet. Their files go into new files named `zz_tiger_test2*` (UTF-8 BOM), with their own namespace `zz_tiger_test2`, so they don't touch the earlier test files. Commas in define lists (conf rule "Unclear, low: commas in define lists") have no test: only code reads those defines, and nothing in game shows whether a comma breaks a list.

Settled without a game test:

- **`switch` scope in `add_to_gift_pool_effect`** (conf rule "possibly tiger: scope inside a switch", 16 reports). This is a vanilla bug (low), not a tiger bug. A copy of the effect in a test mod reproduces the reports. In the herd case, `change_variable = { name = feast_herd_total add = $AMOUNT$_herd_value }` runs at the host character, but the herd values are domicile values (`owner.culture`, `owner.primary_title`, `owner.dominance_value`). So tiger infers a domicile scope for the effect and reports the gold case's character values against it. In game the herd value comes out 0, so the Tsagaan Sar summary tooltip (`feast_tsagaan_sar_events.0010.a_tt`) always shows 0 herd received. The fix is `add = scope:giver.domicile.$AMOUNT$_herd_value`, matching the `pay_herd` line below it.

**Shared file:** `events/zz_tiger_test2_events.txt` starts with `namespace = zz_tiger_test2` and holds every event below. `common/on_action/zz_tiger_test2_on_actions.txt` and `common/character_interactions/zz_tiger_test2_interactions.txt` hold T31 and T28.

### T26. `max` in a `random` effect

- Tiger: `max` is accepted only in `ordered_` iterators, `random_list` and `duel`. The Unop's `07_dlc_ep3_scripted_effects.txt:2489` (as in vanilla) has `random = { chance = 0 modifier = { add = ... } ... max = 95 add_to_list = independent_vassals }`. The docs for `random` mention only `chance` and `modifier`. Question: does `max` cap the chance, is it ignored, or is it an unknown effect?
- Event:

  ```paradox
  zz_tiger_test2.26 = {
   type = character_event
   hidden = yes
   immediate = {
    while = {
     count = 20
     random = {
      chance = 0
      modifier = { add = 100 }
      max = 0
      debug_log = "zz_tiger_test2.26 capped fired"
     }
    }
    random = { chance = 100 debug_log = "zz_tiger_test2.26 control fired" }
   }
  }
  ```

- Check: `event zz_tiger_test2.26`, then count "capped fired" in `debug.log`, and search `error.log` for `max`.
  - None of 20: `max` caps the chance. Tiger should accept `max` in `random`, and the 2 reports are tiger FPs.
  - All 20, or an error about `max`: `max` does nothing there. The vanilla effect then lets the chance exceed 95 (low), and the reports are real.
- **Result (1.20.1.2):** "capped fired" 0 times in 20, the control fired, no errors: `max` caps the chance. Follow-up F57.

### T27. `trigger_situation_catalyst` with a county title

- Tiger: `county` takes a province (the docs: `county = scope:province`, with a `TODO: verify`). Vanilla passes a county title in 3 places (`10_dlc_tgp_silk_road_scripted_effects.txt:408/486/506`, `scope:county_temp`). Question: does a title work, or does the catalyst ignore it and hit every sub-region (or none)?
- No files. Needs the silk road situation (All Under Heaven). `c_merv` is in the Central Asia sub-region (`tgp_silk_road_central_asia_region`).
- Check:
  1. Open the Silk Road situation window and note each sub-region's points.
  2. `effect situation:silk_road_situation = { trigger_situation_catalyst = { catalyst = catalyst_silk_road_raided county = title:c_merv } }`, and note which sub-regions changed.
  3. `effect situation:silk_road_situation = { trigger_situation_catalyst = { catalyst = catalyst_silk_road_raided county = title:c_merv.title_province } }` (the documented form), and note which sub-regions changed.
  4. Search `error.log` for `trigger_situation_catalyst` and `county`.
  - If no sub-region changes for either form, the catalyst has no value in the current phase; try another catalyst from `common/situation/catalysts/10_tgp_silk_road_catalysts.txt`.
- Both forms change only Central Asia: titles work. Tiger should accept a county title (`// undocumented`), and the 3 reports are FPs. The title form changes every sub-region or none, or logs an error: the 3 vanilla uses are bugs (medium: silk road raids, markets and Mongol devastation affect the wrong sub-regions).
- **Result (1.20.1.2):** the title form logs "trigger_situation_catalyst county: scope type landed_title, expected province" and "Validation failed", and changes nothing; the province form changes only Central Asia. Tiger is right (the TODO was replaced with this result in `196d7fde`); the vanilla uses are a medium bug (Unop issue #790).

### T28. `target_titles` in title interactions

- Tiger doesn't know `list target_titles` in character interactions, so `grant_title_holy_order_patronage_warning_effect` (which reads it) is reported as "expects list target_titles to exist" in `grant_titles_interaction` and `grant_governorship_interaction` (2 reports). Vanilla uses `list = target_titles` throughout `00_grant_titles_interaction.txt`; the interaction docs don't mention it. Question: does the game provide the list, and only for the special grant-titles interface or for every `target_type = title` interaction?
- `common/character_interactions/zz_tiger_test2_interactions.txt`:

  ```paradox
  zz_tiger_test2_plain_title_interaction = {
   category = interaction_category_vassal
   common_interaction = yes
   target_type = title
   target_source = actor
   target_filter = domain_titles
   is_shown = { scope:actor != scope:recipient }
   auto_accept = yes
   on_accept = {
    debug_log = "zz_tiger_test2.28 plain"
    debug_log_scopes = yes
    if = {
     limit = { any_in_list = { list = target_titles always = yes } }
     debug_log = "zz_tiger_test2.28 plain has target_titles"
    }
    every_in_list = { list = target_titles debug_log_scopes = yes }
   }
  }
  zz_tiger_test2_grant_title_interaction = {
   category = interaction_category_vassal
   common_interaction = yes
   special_interaction = grant_titles_interaction
   interface = grant_titles
   target_type = title
   target_source = puppet_or_actor
   target_filter = domain_titles
   is_shown = { scope:actor != scope:recipient }
   auto_accept = yes
   on_accept = {
    debug_log = "zz_tiger_test2.28 grant"
    if = {
     limit = { any_in_list = { list = target_titles always = yes } }
     debug_log = "zz_tiger_test2.28 grant has target_titles"
    }
    every_in_list = { list = target_titles debug_log_scopes = yes }
   }
  }
  ```

  Add `zz_tiger_test2_plain_title_interaction: "Tiger plain title test"` and `zz_tiger_test2_grant_title_interaction: "Tiger grant title test"` to `localization/english/zz_tiger_test2_l_english.yml`.
- Check: on a courtier, use "Tiger plain title test" and pick one of your titles, then "Tiger grant title test" with one or two baronies you don't need. The special interface probably transfers the titles itself, so use throwaway titles. Then search `debug.log` for "zz_tiger_test2.28" and `error.log` for `target_titles` and `zz_tiger_test2`.
  - "grant has target_titles": the list exists for the grant-titles interface. Tiger should provide it there (`// undocumented`), and the 2 reports are FPs. If "plain has target_titles" also shows, it should provide it for every title interaction.
  - Neither: the list doesn't exist, the warning in `grant_title_holy_order_patronage_warning_effect` never shows, and the reports are real (low: a missing warning).
  - The special interface rejects a second interaction (an error, or the interaction is missing): the test is void for that part; the plain one still answers the general case.
- **Result (1.20.1.2):** the grant-titles interface sets `list target_titles` (the chosen titles) and `scope:landed_title`; the plain title interaction has neither. Follow-up F58.

### T29. Root of a domicile type's `move_cost`

- Tiger roots `move_cost` in the domicile (undocumented). Vanilla estate costs use `domicile ?= { has_domicile_building_or_higher = estate_main_05 }` (`00_domicile_types.txt`, estate `move_cost`, 9 reports in the Unop's copy), which only works if the root is the owner character.
- No files. Needs an administrative realm: start 1066 as a Byzantine governor with an estate (for example a theme governor).
- Check:
  1. Give the estate main building a level: `effect domicile = { add_domicile_building = estate_main_02 }` (or upgrade in the estate window), and note its level.
  2. Open the estate's move dialog and hover the gold cost. The base is 20, plus 20, 40, 60 or 80 for main building level 2, 3, 4 or 5.
  - The cost includes the level bonus: the root is the character. Tiger should root `move_cost` in the character, and the 9 reports are FPs.
  - The cost is 20 at every level: the root is the domicile, tiger is right, and the vanilla costs are a bug (low: moving a large estate costs too little).
- **Result (1.20.1.2):** an estate with main building level 2 costs 40 gold to move (20 plus the level-2 bonus), so the root is the character. Follow-up F59.

### T30. `war_targets` list in a war scope

- `coronation_events_6000_valid_switch_title_trigger` (Unop's `coronation_events_6.txt`, as in vanilla) checks `any_character_war = { ... any_in_list = { list = war_targets ... } }` inside a war, but nothing there saves `war_targets`. Other vanilla scripts build their own `war_targets` lists. The docs give wars no target title list; casus belli have `any_target_title`. Question: does a war scope provide `war_targets`?
- Event:

  ```paradox
  zz_tiger_test2.30 = {
   type = character_event
   hidden = yes
   immediate = {
    every_ruler = {
     limit = { is_at_war = yes }
     every_character_war = {
      limit = { casus_belli ?= { any_target_title = { always = yes } } }
      if = {
       limit = { any_in_list = { list = war_targets always = yes } }
       debug_log = "zz_tiger_test2.30 war has war_targets"
      }
      else = { debug_log = "zz_tiger_test2.30 war has no war_targets" }
     }
    }
   }
  }
  ```

- Check: a few days into the 1066 start (there are wars for titles), `event zz_tiger_test2.30`, then `debug.log`.
  - "has war_targets": the list exists. Tiger should provide it in wars (`// undocumented`), and the 5 reports are FPs.
  - Only "has no war_targets": the check never sees the war's titles, so a coronation can switch the de jure liege of a title its de jure liege is fighting for (low). The fix is `casus_belli = { any_target_title = { ... } }`.
- **Result (1.20.1.2):** "war has no war_targets" for every war. Tiger is right; the coronation trigger is a vanilla bug (low).

### T31. Several `random_events` blocks in one on_action

- Tiger reports a second `random_events` block as an error ("do not work"; the code says upstream verified this for CK3). The conf marks the vanilla case (`ep1_inspirations_on_actions.txt:157`, the second block of a fund-inspiration on_action) as unclear. Question: in 1.20, does the second block fire, the first, both, or neither?
- Events:

  ```paradox
  zz_tiger_test2.311 = {
   type = character_event
   hidden = yes
   immediate = { debug_log = "zz_tiger_test2.311 first block fired" }
  }
  zz_tiger_test2.312 = {
   type = character_event
   hidden = yes
   immediate = { debug_log = "zz_tiger_test2.312 second block fired" }
  }
  ```

- `common/on_action/zz_tiger_test2_on_actions.txt`:

  ```paradox
  zz_tiger_test2_two_random_events_blocks = {
   random_events = { 100 = zz_tiger_test2.311 }
   random_events = { 100 = zz_tiger_test2.312 }
  }
  ```

- Check: `effect trigger_event = { on_action = zz_tiger_test2_two_random_events_blocks }` a few times, then `debug.log`.
  - Both fire every time: the blocks work, and tiger should drop the error.
  - Only one, or neither: the error stands. The conf rule becomes "Vanilla issue (medium)": the fund-inspiration events in the ignored block (artifact dedication, family motto, modifier choices) never fire.
- **Result (1.20.1.2):** only the first block fired, 5 times in 5. Tiger's error stands, and its comment is already right; the vanilla case is a medium bug (Unop issue #790).

### T32. `recent_history = { days <= 1 }`

- Tiger reports `days <= 1` (the docs show `days = Y`). Vanilla `war_on_actions.txt:1837` uses it to find counties that changed hands in the war just won, for the "become adventurer" path. Question: is `<=` read like `=`, ignored (the window is dropped), or rejected?
- Event:

  ```paradox
  zz_tiger_test2.32 = {
   type = character_event
   hidden = yes
   immediate = {
    every_county = {
     limit = { recent_history = { days = 1 } }
     debug_log = "zz_tiger_test2.32 days = 1 matches"
     debug_log_scopes = yes
    }
    every_county = {
     limit = { recent_history = { days <= 1 } }
     debug_log = "zz_tiger_test2.32 days <= 1 matches"
     debug_log_scopes = yes
    }
    if = {
     limit = { any_county = { count >= 50 recent_history = { days <= 1 } } }
     debug_log = "zz_tiger_test2.32 days <= 1 matches 50 or more"
    }
   }
  }
  ```

- Check: paused, grant one of your counties to a courtier with the vanilla Grant Titles interaction. Then, on the same day, `event zz_tiger_test2.32`, then `debug.log` and `error.log` (`recent_history`, `days`).
  - Both forms match only the granted county: `<=` works like `=`. Tiger should accept it, and the report is a FP.
  - `<=` matches nothing, or logs an error: the vanilla check never passes, so a ruler who loses all land in a war never becomes an adventurer through this path (medium).
  - `<=` matches 50 or more: the window is ignored, and every county with any history entry passes (medium: wrong adventurer candidates).
- **Result (1.20.1.2):** `days <= 1` matched the same two counties as `days = 1`, with no error. Follow-up F60.

### T33. Scripted GUI `scope = innovation_type`

- Tiger reports `scope = innovation_type` in `on_silk_road_innovation_gui` (`tgp_scripted_guis.txt`) as an unknown scope type. The docs and the binary know only `culture_innovation`. The GUI reaches it through the macro `IsOnTheSilkRoad` (`data_binding/tgp_data_bindings.txt`), which shows the `[silk_road_innovation_i]` icon on innovations in the culture window.
- Event (logs the innovations that should show the icon):

  ```paradox
  zz_tiger_test2.33 = {
   type = character_event
   hidden = yes
   immediate = {
    situation:silk_road_situation ?= {
     every_situation_sub_region = {
      limit = { exists = var:innovation }
      var:innovation = { save_scope_as = zz_innovation }
      debug_log = "zz_tiger_test2.33 sub-region innovation"
      debug_log_scopes = yes
     }
    }
   }
  }
  ```

- Check: after loading, search `error.log` for `on_silk_road_innovation_gui`, `innovation_type` and `tgp_scripted_guis`. Run `event zz_tiger_test2.33` and note the logged innovations. Then open the culture window of a culture that doesn't have one of them yet: does it show the silk road icon?
  - The icon shows: `innovation_type` works as a scope name. Tiger should accept it (`// undocumented`), and the 2 reports are FPs.
  - No icon, or a load error: the scripted GUI is broken, and the silk road icon never shows (low).
- **Result (1.20.1.2):** the culture window shows the silk road icon on Fire Medicine for the Viet culture (the innovation of the Tibet and Central Asia sub-regions). Follow-up F61.

### T34. Text format names not defined in `gui/` (optional, low)

- Tiger reports `format_override` names that no `textformatting` block in `gui/` defines: `W`, `e`, `l`, `v`, `dark`, `white`, `game_concept`, `value_red`, `value_green`, `value_yellow`, `negative_value_toast`, `color:gray`, `color:red` (19 reports, e.g. `gui/preload/fonts.gui:87`). Question: are these built into the engine?
- Event:

  ```paradox
  zz_tiger_test2.34 = {
   type = character_event
   title = zz_tiger_test2.34.t
   desc = zz_tiger_test2.34.desc
   theme = default
   option = { name = zz_tiger_test2.34.a }
  }
  ```

- Loc (in `zz_tiger_test2_l_english.yml`):

  ```paradox
   zz_tiger_test2.34.t: "Tiger text formats"
   zz_tiger_test2.34.a: "OK"
   zz_tiger_test2.34.desc: "#W 1 W#! #e 2 e#! #l 3 l#! #v 4 v#! #dark 5 dark#! #white 6 white#! #game_concept 7 game_concept#! #value_red 8 value_red#! #value_green 9 value_green#! #negative_value_toast 10 negative_value_toast#! #zz_nope 11 control#!"
  ```

- Check: `event zz_tiger_test2.34`, then `error.log` for `zz_nope` and the other names.
  - The control is logged as unknown and the others aren't: they are built in. Tiger should know them, and the reports are FPs. Names logged like the control are real (low: plain text in that GUI).
  - Nothing is logged, not even the control: the test is inconclusive. Leave the rule as unclear, low.
- **Result (1.20.1.2):** `#e`, `#l` and `#v` give no error; `#W` (logged as `w`), `#dark`, `#white`, `#game_concept`, `#value_red`, `#value_green` and `#negative_value_toast` give "Unknown formatting tag", like the control. Follow-up F63; the others are a low vanilla issue in the Unop conf.

### T35. Lowercase `list { }` in `default.map` (optional, low)

- `map_data/default.map:311` has `impassable_mountains = list { 12830 12846 12881 }`, where every other entry uses `LIST` or `RANGE`. Tiger reports it as an unexpected block. 12830 and 12846 are `papua_impassable`, and 12881, despite the `#Papua` comment, is `east_hokkaido_mountains` (`definition.csv`).
- No files.
- Check: search `error.log` after loading for `12830`, `12846`, `12881` and `default.map`. On the map, look at east Hokkaido's mountains (12881) and at the Annamite Range (12900, from a `RANGE` entry, as the control): does 12881 look and behave like impassable terrain (no borders, can't be selected, armies path around it)?
  - Impassable, no errors: lowercase `list` works. Tiger should accept it, and the report is a FP.
  - Passable, or errors about these provinces: the entry is ignored, and the 3 provinces aren't impassable (low).
- **Result (1.20.1.2):** province 12881 is impassable, no errors. Follow-up F64.

## CK3 1.20: review of the upstream PR (T36 to T39, T53, T61, T63, T64)

Questions from the review of the upstream PR (#324). The results of T36 to T39 were applied in `8c8a2f4d`. Their files go into new files named `zz_tiger_test3*` (UTF-8 BOM), with the namespace `zz_tiger_test3`: `events/zz_tiger_test3_events.txt` and `localization/english/zz_tiger_test3_l_english.yml`.

### T36. Scopes of `has_county_of_faith` and two fulfillment triggers

- Tiger follows the docs' "Supported Scopes": `has_county_of_faith` in a faith, `doctrine_fulfillment_value` and `expected_tenet_fulfillment_change` in a character. The docs contradict themselves: `has_county_of_faith` is described as "Does the ruler has at least one county following the target faith in their realm?" with the example `scope:ruler = { has_county_of_faith = scope:faith }`, and the two fulfillment triggers show a character example and a block example in `scope:rite`. Vanilla uses none of them.
- Event:

  ```paradox
  zz_tiger_test3.36 = {
   type = character_event
   hidden = yes
   immediate = {
    faith = { save_scope_as = zz_faith }
    rite = { save_scope_as = zz_rite }
    if = {
     limit = { has_county_of_faith = scope:zz_faith }
     debug_log = "zz_tiger_test3.36 A has_county_of_faith in character: true"
    }
    else = { debug_log = "zz_tiger_test3.36 A has_county_of_faith in character: false" }
    scope:zz_faith = {
     if = {
      limit = { has_county_of_faith = scope:zz_faith }
      debug_log = "zz_tiger_test3.36 B has_county_of_faith in faith: true"
     }
     else = { debug_log = "zz_tiger_test3.36 B has_county_of_faith in faith: false" }
    }
    if = {
     limit = { doctrine_fulfillment_value = { target = doctrine:doctrine_monogamy value > -1000 } }
     debug_log = "zz_tiger_test3.36 C doctrine_fulfillment_value in character: true"
    }
    else = { debug_log = "zz_tiger_test3.36 C doctrine_fulfillment_value in character: false" }
    scope:zz_rite = {
     if = {
      limit = { doctrine_fulfillment_value = { target = doctrine:doctrine_monogamy value > -1000 } }
      debug_log = "zz_tiger_test3.36 D doctrine_fulfillment_value in rite: true"
     }
     else = { debug_log = "zz_tiger_test3.36 D doctrine_fulfillment_value in rite: false" }
    }
    if = {
     limit = { expected_tenet_fulfillment_change = { target = tenet:tenet_communion value > -1000 } }
     debug_log = "zz_tiger_test3.36 E expected_tenet_fulfillment_change in character: true"
    }
    else = { debug_log = "zz_tiger_test3.36 E expected_tenet_fulfillment_change in character: false" }
    scope:zz_rite = {
     if = {
      limit = { expected_tenet_fulfillment_change = { target = tenet:tenet_communion value > -1000 } }
      debug_log = "zz_tiger_test3.36 F expected_tenet_fulfillment_change in rite: true"
     }
     else = { debug_log = "zz_tiger_test3.36 F expected_tenet_fulfillment_change in rite: false" }
    }
   }
  }
  ```

- Check: as a ruler holding counties of your own faith, search `error.log` after loading for `zz_tiger_test3`. Then run `event zz_tiger_test3.36` and look in `debug.log` for "zz_tiger_test3.36" and in `error.log` for errors at the event's lines (wrong scope, unknown trigger).
  - For each trigger, the scope that gives "true" without an error is a supported scope; one that logs a scope error is not. `value > -1000` is true whenever the trigger evaluates.
  - `has_county_of_faith`: A true without errors means the docs' "Supported Scopes: faith" is wrong; tiger should take a character. B without errors means faith works too.
  - The fulfillment triggers: D or F true without errors means a rite scope works as well.
- **Result (1.20.1.2):** `has_county_of_faith` is true in a character (A) and false in a faith (B), with no runtime error for either; when loading, the game logs "Inconsistent trigger scopes (character vs. faith)" for A. The fulfillment triggers are true in a character (C, E) and give "Wrong scope for trigger: rite, expected character" in a rite (D, F). Tiger now takes `has_county_of_faith` in a character, with a comment on the load error.

### T37. The remaining loc tooltip tags

- 1.20's `tooltip_structs_l_english.yml` uses six `#TOOLTIP:` tags that tiger's table (`COMPLEX_TOOLTIPS`) still lacks: `CONTRACT`, `FULFILLMENT_LEVEL`, `GLOSSARY` and `SITUATION_MODIFIER_NAMED_SET` (all with `$DESC_KEY$`), `GREAT_PROJECT` and `SITUATION` (with `$ID$`, which suggests a runtime id, as for `CHARACTER`). Without an entry, tiger checks the value as a loc key. T20 gave `CONTRACT,overdue_taxes` (a task contract type) "Unlocalized text"; this retest adds a subject contract key. The docs call a situation modifier set's key its loc key, and the fulfillment docs build level names as `<type>_level_<index>`.
- Event:

  ```paradox
  zz_tiger_test3.37 = {
   type = character_event
   title = zz_tiger_test3.37.t
   desc = zz_tiger_test3.37.desc
   theme = default
   option = { name = zz_tiger_test3.37.a }
  }
  ```

- Loc (in `zz_tiger_test3_l_english.yml`):

  ```paradox
   zz_tiger_test3.37.t: "Tiger tooltip tags 2"
   zz_tiger_test3.37.a: "OK"
   zz_tiger_test3.37.desc: "#TOOLTIP:FAITH,catholic 1 FAITH (control)#!\n#TOOLTIP:CONTRACT,overdue_taxes 2 CONTRACT task contract#!\n#TOOLTIP:CONTRACT,administrative_obligations 3 CONTRACT subject contract#!\n#TOOLTIP:FULFILLMENT_LEVEL,christian_fulfillment 4 FULFILLMENT_LEVEL type#!\n#TOOLTIP:FULFILLMENT_LEVEL,christian_fulfillment_level_0 5 FULFILLMENT_LEVEL level#!\n#TOOLTIP:GLOSSARY,POT_METAL_GLOSS 6 GLOSSARY#!\n#TOOLTIP:GREAT_PROJECT,grand_campaign_project 7 GREAT_PROJECT#!\n#TOOLTIP:SITUATION,silk_road_situation 8 SITUATION#!\n#TOOLTIP:SITUATION_MODIFIER_NAMED_SET,silk_road_phase_effects 9 SITUATION_MODIFIER_NAMED_SET#!"
  ```

- Check: `event zz_tiger_test3.37`, then hover each numbered line: does the tooltip describe the item, show nothing, or show a raw key? Also `error.log` for `Unlocalized text` and `zz_tiger_test3`.
  - A tooltip that describes the item: the tag takes that key; add it to the table with the item type (or keep tiger's loc check if the key is a loc key, as for GLOSSARY).
  - Nothing, or "Unlocalized text": the value is not that kind of key. Add the tag as a runtime id (`None`), like `CHARACTER`, when the struct passes `$ID$`.
  - When every tag is known, update the table's marker to `LAST UPDATED CK3 VERSION 1.20.1`.
- **Result (1.20.1.2):** lines 1 (FAITH) and 6 (GLOSSARY) show their tooltips; 4, 5, 7 and 8 (FULFILLMENT_LEVEL, GREAT_PROJECT, SITUATION) show nothing; 2, 3 and 9 show raw text with "Unlocalized text 'CONTRACT,overdue_taxes'", "... 'CONTRACT,administrative_obligations'" and "... 'SITUATION_MODIFIER_NAMED_SET,silk_road_phase_effects'". FULFILLMENT_LEVEL, GREAT_PROJECT and SITUATION are runtime ids; GLOSSARY, CONTRACT and SITUATION_MODIFIER_NAMED_SET stay out of the table (their value is checked as a loc key); the table's marker is now 1.20.1.

### T38. `scope:activity` in an activity phase's `is_valid`

- The docs give a phase's `is_valid` root = the activity, `scope:host` and `scope:province`, but not `scope:activity`, which every other activity block gets. Vanilla's feast reads it there: `feast_phase_meal` `is_valid` evaluates `scope:activity = { has_activity_option = ... }` in a `trigger_if` limit on every check (`common/activities/activity_types/feast.txt:4349`). Tiger provides it (`activities.rs`, "The docs leave out scope:activity here"). The first batch saw no errors in 20 years of observer mode; this test hosts a feast directly.
- No files.
- Check: plan and host a feast (any type). When it has started and the meal phase is current or about to start, search `error.log` for `feast.txt` with `line: 43`.
  - No errors from those lines: `scope:activity` is set in phase `is_valid`. Mark it `// undocumented` and drop the docs comment.
  - "Event target link 'scope' returned an unset scope" at `feast.txt:4349`: it is not set; tiger should drop it, and the vanilla feast check is a bug.
- **Result (1.20.1.2):** a hosted feast logged no errors for `feast.txt`, so `scope:activity` is set in phase `is_valid`; marked `// undocumented`.

### T39. Several `history` blocks in `set_owner` and `set_holy_site_owner`

- Tiger accepts several `history` blocks in the artifact effects `set_owner` (upstream) and `set_holy_site_owner` (added for 1.20, mirroring `set_owner`). The docs for both say `history = artifact history entry` (one), and vanilla never repeats it.
- Event:

  ```paradox
  zz_tiger_test3.39 = {
   type = character_event
   hidden = yes
   immediate = {
    random_courtier = { save_scope_as = zz_courtier }
    faith = { random_holy_site = { save_scope_as = zz_site } }
    random_character_artifact = { save_scope_as = zz_art1 }
    random_character_artifact = {
     limit = { this != scope:zz_art1 }
     save_scope_as = zz_art2
    }
    scope:zz_art1 = {
     set_owner = {
      target = scope:zz_courtier
      history = { type = given actor = root recipient = scope:zz_courtier }
      history = { type = purchased actor = root recipient = scope:zz_courtier }
      generate_history = no
     }
    }
    scope:zz_art2 ?= {
     set_holy_site_owner = {
      target = scope:zz_site
      history = { type = given actor = root recipient = root }
      history = { type = purchased actor = root recipient = root }
      generate_history = no
     }
    }
    debug_log = "zz_tiger_test3.39 done"
    debug_log_scopes = yes
   }
  }
  ```

- Check: as a ruler with at least two artifacts and a courtier, `event zz_tiger_test3.39`. Open the first artifact (now the courtier's) and look at its history: does it list both "given" and "purchased" entries, only one (which?), or none? Do the same for the second artifact, now in the holy site (`debug.log` names both). Also `error.log` for `zz_tiger_test3` and `history`.
  - Both entries listed: several `history` blocks work in both effects; tiger is right.
  - Only one: the effects take one entry; tiger should accept `history` once in both (`field_validated_block_sc`).
- **Result (1.20.1.2):** both artifacts list only the "purchased" entry, the last one: one `history` takes effect. `set_owner` and `set_holy_site_owner` now accept it once; `create_artifact` is follow-up F65.

### T53. `tier` in accolade types

- The 1.20 update accepts `tier = common/skilled/eminent` in accolade types (`accolades.rs:147`), following `_accolade_type.info:22-24` ("denotes the "rarity" of a accolade type, will correspond to an ingame enum"). Vanilla never sets it and puts `skilled`/`eminent` into `accolade_categories` instead, so the reviewer (PR #324, `accolades.rs:147`) asks for proof that the game reads it. Supporting evidence: the trigger `accolade_type_tier` ("Is the scoped Accolade Type's tier equal to?", `triggers.log:42`, unused in vanilla) and the standalone binary string `skilled`. Tiger's (upstream) table takes a scope for that trigger: `accolade_type_tier = <accolade type>` compares two types' tiers.
- Files, with the events in `events/zz_tiger_test3_events.txt`:

  ```paradox
  # common/accolade_types/zz_tiger_test3_accolade_types.txt
  zz_tiger_test3_eminent_a = {
   adjective = accolade_politicking
   noun = accolade_politicker
   icon = "gfx/interface/icons/knight_badge/icons/accolade_trait_courtly_blue.dds"
   tier = eminent
   accolade_categories = { courtly }
   squire_perfect_fit = { always = no }
   squire_acceptable_fit = { always = no }
   weight = { value = 0 }
   ranks = { 1 = { liege_modifier = { diplomacy = 1 } } }
  }
  # zz_tiger_test3_eminent_b: the same with tier = eminent
  # zz_tiger_test3_skilled: the same with tier = skilled

  # Scope form, as tiger expects
  zz_tiger_test3.53 = {
   type = character_event
   hidden = yes
   immediate = {
    if = { limit = { accolade_type:zz_tiger_test3_eminent_a = { accolade_type_tier = accolade_type:zz_tiger_test3_eminent_b } } debug_log = "T53 eminent vs eminent: true" }
    else = { debug_log = "T53 eminent vs eminent: false" }
    if = { limit = { accolade_type:zz_tiger_test3_eminent_a = { accolade_type_tier = accolade_type:zz_tiger_test3_skilled } } debug_log = "T53 eminent vs skilled: true" }
    else = { debug_log = "T53 eminent vs skilled: false" }
    if = { limit = { accolade_type:zz_tiger_test3_skilled = { accolade_type_tier = accolade_type:politicker_attribute } } debug_log = "T53 skilled vs vanilla skilled-category: true" }
    else = { debug_log = "T53 skilled vs vanilla skilled-category: false" }
    if = { limit = { accolade_type:politicker_attribute = { accolade_type_tier = accolade_type:tactician_attribute } } debug_log = "T53 control vanilla vs vanilla: true" }
    else = { debug_log = "T53 control vanilla vs vanilla: false" }
   }
  }

  # Literal form, in its own event in case it doesn't parse
  zz_tiger_test3.153 = {
   type = character_event
   hidden = yes
   immediate = {
    if = { limit = { accolade_type:zz_tiger_test3_eminent_a = { accolade_type_tier = eminent } } debug_log = "T53 literal eminent = eminent: true" }
    else = { debug_log = "T53 literal eminent = eminent: false" }
    if = { limit = { accolade_type:politicker_attribute = { accolade_type_tier = common } } debug_log = "T53 literal vanilla = common: true" }
    else = { debug_log = "T53 literal vanilla = common: false" }
   }
  }
  ```

- Check: `event zz_tiger_test3.53` and `event zz_tiger_test3.153`, then the `T53` lines in `debug.log`, and `error.log` for `zz_tiger_test3`, `tier` and `accolade`. Ignore messages about the test types' ranks or loc, if any.
  - "eminent vs skilled: false" with "eminent vs eminent: true", the control true, and no "unknown field" error for `tier`: the game reads `tier`. Keep `field_choice("tier", ...)` and reply "Tested in game (1.20.1.2)".
  - "eminent vs skilled: true", or an error naming `tier`: the field is ignored (every type is the default tier). Drop it from `accolades.rs` as the reviewer suggested.
  - "skilled vs vanilla skilled-category: true" while "eminent vs skilled" is false: vanilla's `skilled` category sets the tier too, i.e. both mechanisms exist; worth a line in the reply.
  - The literal form: if `.153` logs no parse error and gives "true" twice, the trigger also takes a tier name, and tiger's upstream entry (`Scope(Scopes::AccoladeType)`) is incomplete; that is a separate follow-up.

- **Result (1.20.1.2):** no error for `tier` or for either form. Literal form (`.153`): eminent_a = eminent true, vanilla = common true. Retest with mismatches (`zz_tiger_test3.253` in the test mod's literal file): eminent_a = common false, eminent_a = skilled false, skilled = skilled true, vanilla = eminent false. Scope form (`.53`): only vanilla vs vanilla true, as if the right side were `common`. So the game reads `tier` (the default is `common`), and `accolade_type_tier` takes a tier name, not an accolade type: tiger's entry `Scope(Scopes::AccoladeType)` is wrong.

### T61. Lengths and values of `subsequent_playback_chance`

- The docs show `subsequent_playback_chance = { 1.0 0.5 ... }`, "Chance (0-1) for each subsequent track from this group to be played in a row". Tiger used to require exactly 3 numbers; since 1.20 it accepts any number of numbers (`src/data/music.rs`), following the docs and vanilla, which has 5 lists of 2, 3 of 3 and 1 of 4. The reviewer (PR #324, `music.rs:170`) asks whether that is verified and suggests allowing only 2 to 4. It replaces T7, which was never run.
- File `music/in_game/zz_tiger_test3_music.txt`:

  ```paradox
  group_zz_tiger_test3_three = { subsequent_playback_chance = { 0.9 0.6 0.3 } } # control: must work
  group_zz_tiger_test3_text = { subsequent_playback_chance = { 0.9 abc } } # control: must fail
  group_zz_tiger_test3_empty = { subsequent_playback_chance = { } }
  group_zz_tiger_test3_one = { subsequent_playback_chance = { 0.9 } }
  group_zz_tiger_test3_six = { subsequent_playback_chance = { 0.9 0.8 0.7 0.6 0.5 0.4 } }
  group_zz_tiger_test3_range = { subsequent_playback_chance = { 1.5 -0.5 } }
  ```

- Check: `error.log` after loading for `zz_tiger_test3_music`, `group_zz_tiger_test3` and `subsequent_playback_chance`. The test only shows what the game accepts at load, not whether it uses every value.
  - An error for `text` only: there is no length limit at load, so tiger leaves the length open and answers the reviewer with this result. The 0 to 1 range stays a docs-based check.
  - No error for `text` either: the game doesn't report problems here, so the test says nothing about the others, and the length stays open with a `TODO: verify`.
  - An error for `one`, `six` or `empty`: enforce the limit that the errors show.
  - An error for `range`: the game enforces the range too.

**Result (1.20.1.2):** the only error after loading is "Malformed token: abc, near line: 3" for the text control; nothing for the empty list, the lists of 1, 3 and 6 chances, or the out-of-range 1.5 and -0.5. The game sets no length limit at load, so tiger keeps the length open. Whether it uses every value, and how it treats values outside 0 to 1, a load test can't show; the range stays a docs-based check.

### T63. Bare names in culture trait `parameters`

- Since 1.20, `validate_parameters` in `src/ck3/data/cultures.rs` accepts bare names next to `name = yes`: the 1.20 `_cultural_traits.info` changed its example from `param_name = yes/no` and `param_name = 0.5` to a bare `param_name` ("list of flags"). All 742 vanilla culture parameters still use `name = yes`. Tenets (315) and holy sites (6) use bare names in 1.20 vanilla, and the holy site docs say the `= yes` form is legacy. The reviewer (PR #324, review set 7, `cultures.rs`) asks whether bare names work in cultures, with an example.
- File `common/culture/traditions/zz_tiger_test3_traditions.txt`, a tradition that is never shown or picked:

  ```paradox
  zz_tiger_test3_params_tradition = {
  	category = ritual
  	layers = { 0 = learning 1 = western 4 = knight.dds }
  	is_shown = { always = no }
  	can_pick = { always = no }
  	parameters = {
  		zz_tiger_test3_bare_param
  		zz_tiger_test3_yes_param = yes # control
  	}
  	ai_will_do = { value = 0 }
  }
  ```

- Event `zz_tiger_test3.63` adds the tradition to the player's culture if it's missing, then logs `T63 tradition: present/missing`, `T63 bare name: set/not set` and `T63 control name = yes: set/not set` (`has_cultural_parameter`).
- Check: `event zz_tiger_test3.63`, then the `T63` lines in `debug.log`, and `error.log` for `zz_tiger_test3_traditions` and `zz_tiger_test3_bare_param`. If the control is "not set" too, the parameters may update later: run the event again a day later.
  - "bare name: set" with the control set: bare names work in cultures. Keep them and reply "Tested in game".
  - "bare name: not set" with the control set, or a parse error at the bare name: the culture docs are wrong. Accept only `name = yes` in culture parameters, and add a "Known docs issue".

**Result (1.20.1.2):** no errors; on the first run the tradition was added and present, and both "bare name: set" and "control name = yes: set". Bare names work in culture trait parameters and take effect at once, so tiger keeps accepting both forms.

### T64. Doctrine parameters without `doctrine_parameter_<name>` loc

- The doctrine docs say "Effects are localized based on the parameter name. "doctrine_parameter_" plus the name.", and the tenet docs refer to the doctrine docs. The reviewer (PR #324, review set 8, `doctrines.rs`) asks tiger to check that loc. `verify_parameter_loca` in `src/ck3/data/doctrines.rs` now does, for doctrine and tenet parameters (`TODO: verify`, `a316bc9a`). 27 of the 399 vanilla parameter names have no such loc (30 places, 22 in doctrines and 8 in tenets), among them `doctrine_monogamy` and `doctrine_polygamy`; 9 others have an empty `""` loc.
- No files: vanilla only.
- Start as a Catholic ruler and open the faith view. Hover the Monogamous doctrine (Marriage): the control line "Rulers get a significant bonus when they marry for the first time" (`marriage_event`) must be among its effects. Look for a raw key like `doctrine_parameter_doctrine_monogamy`. If possible, also check an Islamic faith's Polygamous doctrine: control "Having too few spouses will incur a piety penalty" (`spouse_piety_loss`), check `doctrine_parameter_doctrine_polygamy`.
  - Raw key shown: the loc is required. Remove the `TODO`; the vanilla reports are real.
  - Nothing shown for the parameter, with the control shown: the game skips parameters without loc. Replace the check with `mark_used`, and reply to the reviewer why.

## CK3 1.20: review of the Unop's vanilla-issue rules (T40 to T52)

Each settles one "Unclear" rule in the Unop conf.

### T40. Interaction root outside `cost`

Tiger roots interactions in none ("You're expected to use scope:actor"); only `cost` was tested. File `common/character_interactions/zz_tiger_test3_interactions.txt`, with the loc key `zz_tiger_test3_interaction`:

```paradox
zz_tiger_test3_interaction = {
 category = interaction_category_friendly
 auto_accept = yes
 is_shown = { is_ai = no }
 on_accept = {
  debug_log = "T40 on_accept"
  debug_log_scopes = yes
  add_gold = 1
 }
}
```

Check: open the interaction menu of an AI courtier. Shown there: `is_shown` sees the player, so root is the actor. Not shown: root is the recipient or none. If shown, use it: whose gold goes up, what root does `debug.log` give, and what does `error.log` say? Medium if root is none: `populate_actor_list` in `break_betrothal_interaction`, the adoptionism `is_shown` in `00_adoption.txt`, a diarch `send_option`, the catalyst in `10_tgp_interactions.txt:11530`.

**Result (1.20.1.2):** not shown on an AI courtier, and `error.log` gives "is_ai trigger [ Wrong scope for trigger: none, expected character ]" for `is_shown`. Interaction blocks have no root unless documented, as tiger says. The vanilla uses are real bugs (medium): `populate_actor_list` in `break_betrothal_interaction`, the adoption `is_shown`, a diarch `send_option` and the catalyst in `10_tgp_interactions.txt:11530`.

### T41. `if` without `limit`

`effect if = { add_gold = 10 }` in the console: does the gold change? Decides the struggle holy war trigger (`00_generic_struggle_scripted_triggers.txt:386`) and the relic-removal opinions (`inventory_on_actions.txt:462`).

**Result (1.20.1.2):** the gold changes and nothing is logged: an effect `if` without `limit` runs its effects, as if the limit were true. `trigger_if` and `else_if` weren't tested. Tiger keeps reporting the missing `limit`, since it is almost always a lost condition: in `are_holy_wars_disabled_in_struggle_phase_trigger` the county check is the body of the `trigger_if`, so duchy and kingdom targets never reach `trigger_else` (if `trigger_if` behaves the same); the `else_if` in `inventory_on_actions.txt` simply acts as an `else`.

### T42. Two `save_scope_as` in `create_character`

Event `zz_tiger_test3.42` in `events/zz_tiger_test3_events.txt` creates a character with `save_scope_as = zz_a save_scope_as = zz_b` and logs whether each is set, then `debug_log_scopes`. Check: `event zz_tiger_test3.42` and the `T42` lines in `debug.log`: are both set? Decides `tgp_mandala_task_contract_events.txt:665`.

**Result (1.20.1.2):** only the last `save_scope_as` applies ("zz_a: not set", "zz_b: set"). Tiger's duplicate-field warning is right, and `scope:local_character` is unset in `tgp_mandala_task_contract_events.txt` .1000 (medium).

### T43. `death = Yes` in character history

Start 1178 and check whether `character:33410` and `character:30218` are dead.

**Result (1.20.1.2):** both are dead (`set_relation_friend` gives "target character is dead"). Both have `death = Yes` (`bavarian.txt:5856`, `saxon.txt:3182`); the game reads it as a death, either as `yes` or as an unknown death reason. `death = yrd` (`italian.txt:19610`, character 167903) and `birth = ye` (`cisalpine.txt:5276`, character 70789) weren't tested.

### T44. A `desc = { }` block in `first_valid` with no matching `triggered_desc`

Hover a county in the court languages map mode whose holder has no royal court in the chain: does the fallback text show (`map_modes.txt:584`)?

**Result (1.20.1.2):** the tooltip shows "Court Language: None", the `COURT_LANGUAGES_MAP_MODE_TOOLTIP_FALLBACK` text. A `desc = { }` block whose `triggered_desc`s all fail is skipped, and `first_valid` moves on. Tiger's "multiple unconditional desc in one first_valid" is a false positive for such a block.

### T45. Named `attach` in unit assets

Look at a Mongol travel or turko-mongol infantry unit on the map: does the horse show and animate, and is `error.log` quiet (`attach = { name = mount root = ... }`)?

**Result (1.20.1.2):** the horse is shown and animated, and nothing is logged. These assets are in `gfx/models/units/infantry/` because that folder holds the army's foot-slot models; the Mongol ones are mounted. `attach = { name = mount root = ... }` names the attached entity, so tiger's "asset mount not defined" is a false positive.

### T46. `list_size` of a missing list (F74)

Event `zz_tiger_test3.46` in `events/zz_tiger_test3_events.txt` tests `"list_size(zz_none)" <= 0` and logs the result. Check: `event zz_tiger_test3.46`: is the `T46` line in `debug.log` true, and is `error.log` quiet?

**Result (1.20.1.2):** true, with no error. `list_size` of a list that was never built is 0, so `list_size(x) > 0` works as a guard (F74).

### T47. A saved scope named like a game concept (F33)

Tiger says the game concept takes precedence over a saved scope of the same name in loc, and then reports "`friend` returns CString but a scope type is needed here" (vanilla: `ecumenical_council_events.0108.desc`, `hostage_oath_of_friendship_desc_third_perspective`). Event in `events/zz_tiger_test3_events.txt` (`namespace = zz_tiger_test3`):

```paradox
zz_tiger_test3.47 = {
 type = character_event
 title = zz_tiger_test3.47.t
 desc = zz_tiger_test3.47.desc
 theme = friendly
 immediate = { random_courtier = { save_scope_as = friend } }
 option = { name = zz_tiger_test3.47.a }
}
```

Loc (`localization/english/zz_tiger_test3_l_english.yml`): `zz_tiger_test3.47.desc:0 "A: [friend.GetFirstName] B: [friend.Self.GetFirstName]"`, plus a title and an option. Check: `event zz_tiger_test3.47`. If A shows the courtier's name like B, the saved scope wins and the reports are tiger false positives; if A is empty or wrong, or `error.log` mentions `friend`, the concept wins and the vanilla loc is broken (`friend.Self` would be the fix). Tiger also reports B ("Self must be last in the chain"); if B shows the name, that check is wrong too.

**Result (1.20.1.2):** "A: Narin B: ERROR:[friend.Self.GetFirstName]", with "Could not find promote for 'Self' in 'friend.Self.GetFirstName'". The saved scope wins over the game concept, so tiger's reports on the vanilla loc are false positives (F33). `Self` in the middle of a chain fails, so tiger's "Self must be last in the chain" is right.

### T48. The same trait portrait modifier entry in several groups (F83)

Vanilla repeats `no_visuals_when_beautified` (trigger `portrait_beautified_trigger = yes`) as the first entry of 17 groups in `gfx/portraits/trait_portrait_modifiers/00_trait_modifiers.txt`, e.g. `disfigured` and `blind`. No files needed. Check `error.log` after loading for duplicate entry messages. Then give a character `disfigured` and `blind` (`add_trait`), check that the portrait shows both, make them a saint or kill them with the Beautify Deceased game rule on, and check that neither shows. Both hidden and no errors: entries belong to their group, and the 18 reports are tiger false positives.

**Result (1.20.1.2):** both traits showed on the living character. After `effect root.faith = { add_saint = { character = character:<id> burial_province = root.location } }` (the first try used `target`, which the game rejects: the docs are wrong), the dead saint shows neither: no blindness, no disfigurement, a halo. So `no_visuals_when_beautified` works as the first entry of both groups: entries belong to their group, and the 18 reports are tiger false positives (F83, #335).

### T49. Artifact visuals where no asset or icon matches (F84)

Tiger wants one `asset` and one `icon` without a trigger in each artifact visual ("needs at least one asset without a trigger", 69 vanilla reports); the `.info` doesn't say so. Vanilla `chest` (`00_court_artifacts.txt:1`) keys every asset on building gfx. Question: what does the game do when no entry matches? Files, with the event in `events/zz_tiger_test3_events.txt`:

```paradox
# common/artifacts/visuals/zz_tiger_test3_visuals.txt
zz_tiger_test3_none = {
 icon = { trigger = { always = no } reference = "artifact_chest.dds" }
 asset = { trigger = { always = no } reference = ep1_mena_chest_fancy_01_a_entity }
}
zz_tiger_test3_control = {
 icon = "artifact_chest.dds"
 asset = ep1_mena_chest_fancy_01_a_entity
}

zz_tiger_test3.49 = {
 type = character_event
 hidden = yes
 immediate = {
  create_artifact = { name = artifact_chest_name description = placeholder type = chest visuals = zz_tiger_test3_none modifier = artifact_placeholder_modifier }
  create_artifact = { name = artifact_chest_name description = placeholder type = chest visuals = zz_tiger_test3_control modifier = artifact_placeholder_modifier }
 }
}
```

Check: `event zz_tiger_test3.49`, then compare the two chests in the inventory and the court scene, and search `error.log` for `zz_tiger_test3`. If the first has no icon or model, or the game logs an error, the check is right and vanilla visuals whose triggers don't cover every case are real (low) issues; then only the complementary `NOR` fallback (longbow, armillary sphere) needs recognising. If the game falls back to something sensible without an error, lower the check to advice.

**Result (1.20.1.2):** "create_artifact effect [ Couldn't determine 'asset' visual type; all triggers returned false for zz_tiger_test3_none ]", and the same for `icon`; no artifact was created (the control wasn't either, probably because the error stopped the event). The game has no fallback, so tiger's check is right. Vanilla visuals keyed on building or unit gfx work only for cultures that have that gfx; the complementary `NOR` fallbacks are the only false positives (F84).

### T50. Which light type reads the cone angles (F34)

The portrait environment template (`gfx/portraits/environments/portrait_environments_pam.txt:20-40`) says "For point lights" above `outer_cone_angle` and `inner_cone_angle`, so tiger reports them on every other light: 705 reports each, nearly all on spot lights. Cone angles normally belong to spot lights. Test: the test mod's copy of `gfx/portraits/environments/portrait_environments.txt` narrows the key `spot_light` of `environment_standard` (which the head, shoulders, torso and body environments share) from `outer_cone_angle = 25` / `inner_cone_angle = 5` to 5 and 2. Compare a portrait in the character window with and without the test mod. If the lit area shrinks to a small spot, the angles are for spot lights and the 1,410 reports are tiger false positives. If nothing changes, repeat with a `point_light` (e.g. in `portrait_environments_pam.txt`); if that changes, the reports are template leftovers (vanilla, low).

**Result (1.20.1.2):** with the narrower angles on the key `spot_light`, the lit area of the character window portrait shrinks visibly (only the near side of the face is lit; `2026_11_01_1.png` in the game's screenshots). The cone angles are read by spot lights, so the 1,410 reports are tiger false positives (F34).

### T51. Comparing a variable that holds another type (F27, tiger#329)

Vanilla stores either a flag or a doctrine in `current_debate_proposal` and compares it with `?=` against flags (`pam_ecumenical_council_effects.txt:413,522`). Question: is a compare against the other type just false, or does it log an error, and does `=` behave differently from `?=`? Event in `events/zz_tiger_test3_events.txt`:

```paradox
zz_tiger_test3.51 = {
 type = character_event
 hidden = yes
 immediate = {
  set_variable = { name = zz_mixed value = doctrine:doctrine_clerical_succession_conditional }
  if = { limit = { var:zz_mixed ?= flag:permit } debug_log = "T51 doctrine ?= flag: true" }
  else = { debug_log = "T51 doctrine ?= flag: false" }
  if = { limit = { var:zz_mixed = flag:permit } debug_log = "T51 doctrine = flag: true" }
  else = { debug_log = "T51 doctrine = flag: false" }
  set_variable = { name = zz_mixed value = flag:permit }
  if = { limit = { var:zz_mixed ?= doctrine:doctrine_clerical_succession_conditional } debug_log = "T51 flag ?= doctrine: true" }
  else = { debug_log = "T51 flag ?= doctrine: false" }
  if = { limit = { var:zz_mixed = flag:permit } debug_log = "T51 control flag = flag: true" }
  else = { debug_log = "T51 control flag = flag: false" }
 }
}
```

Check: `event zz_tiger_test3.51`, then the `T51` lines in `debug.log` and anything new in `error.log`. Expected if mixed types work: the three cross-type compares are false, the control is true, and `error.log` is quiet; then tiger should allow a set of types per variable. If only `=` logs an error, tiger could accept a mixed-type variable only in `?=` compares.

**Result (1.20.1.2):** every cross-type compare logs "Left side and right side during comparison were of different types", with `?=` as well as `=`, and is false; the flag control is true. Storing several types in one variable is fine, but comparing it with another type is an error, so tiger's report at the compare is right and the vanilla compares in `pam_ecumenical_council_effects.txt:413,522` log errors when the variable holds a doctrine (low). This changes the premise of F27 (#329).

### T52. `exists` and `?=` on a link of an unset scope (F70, tiger#332)

Tiger should treat `exists = scope:x.link` and `scope:x.link ?= { }` as guards for `scope:x`, which assumes both are false without an error when `scope:x` is unset. Event in `events/zz_tiger_test3_events.txt`:

```paradox
zz_tiger_test3.52 = {
 type = character_event
 hidden = yes
 immediate = {
  if = { limit = { exists = scope:zz_none.capital_province } debug_log = "T52 exists: true" }
  else = { debug_log = "T52 exists: false" }
  if = { limit = { scope:zz_none.capital_province ?= { always = yes } } debug_log = "T52 ?=: true" }
  else = { debug_log = "T52 ?=: false" }
  if = { limit = { scope:zz_none.capital_province = { always = yes } } debug_log = "T52 control =: true" }
  else = { debug_log = "T52 control =: false" }
 }
}
```

Check: `event zz_tiger_test3.52`, then the `T52` lines in `debug.log` and `error.log`. Expected: "exists: false" and "?=: false" with no error for those lines, and an "Undefined event target" (or similar) error only for the control. If `exists` on the chain also logs an error, only the `?=` form counts as a guard, and #332 should say so.

**Result (1.20.1.2):** "exists: false" and "?=: false" with nothing logged; only the control logs "Undefined event target 'zz_none'" and "Failed context switch". Both `exists = scope:x.link` and `scope:x.link ?= { }` are safe guards, as F70 (#332) assumes.

## CK3 1.20: remaining follow-ups (T54 to T60)

Each settles a question in [follow-up.md](follow-up.md). Files go with the others in `zz_tiger_test3*` (UTF-8 BOM, namespace `zz_tiger_test3`). Missing loc for the test keys is expected and can be ignored.

### T54. `create_character`: `random_faith_in_religion` and the template forms of `ethnicity` (F10)

The effect docs say `random_faith_in_religion = religion tag OR a faith scope`; in templates the game rejects both keys (T4, applied in `4a0c557c`), and tiger accepts a religion key in the effect. The template `.info` also gives `ethnicity = default/template_character` and `ethnicities = { <ethnicity> = { trigger weight } }`, while the effect docs say `ethnicities = { <key> = { some_triggers } }`. Play a ruler who is neither Muslim nor of an African culture. Event in `events/zz_tiger_test3_events.txt`:

```paradox
zz_tiger_test3.54 = {
 type = character_event
 hidden = yes
 immediate = {
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture random_faith_in_religion = islam_religion save_scope_as = zz_a }
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture random_faith_in_religion = faith:sunni save_scope_as = zz_b }
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture faith = root.faith ethnicities = { african = { trigger = { always = yes } weight = { base = 1 } } } save_scope_as = zz_c }
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture faith = root.faith ethnicities = { african = { always = yes } } save_scope_as = zz_d }
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture faith = root.faith ethnicity = default save_scope_as = zz_e }
  create_character = { location = root.capital_province gender = male age = 30 culture = root.culture faith = root.faith template_character = scope:zz_c ethnicity = template_character save_scope_as = zz_f }
  if = { limit = { scope:zz_a.faith.religion = religion:islam_religion } debug_log = "T54 A religion key: Muslim" }
  else = { debug_log = "T54 A religion key: not Muslim" }
  if = { limit = { scope:zz_b.faith.religion = religion:islam_religion } debug_log = "T54 B control faith scope: Muslim" }
  else = { debug_log = "T54 B control faith scope: not Muslim" }
  if = { limit = { scope:zz_c = { has_ethnicity = african } } debug_log = "T54 C template form: african" }
  else = { debug_log = "T54 C template form: not african" }
  if = { limit = { scope:zz_d = { has_ethnicity = african } } debug_log = "T54 D effect docs form: african" }
  else = { debug_log = "T54 D effect docs form: not african" }
  if = { limit = { scope:zz_f = { has_same_ethnicity_as = scope:zz_c } } debug_log = "T54 F template_character: same as C" }
  else = { debug_log = "T54 F template_character: differs from C" }
 }
}
```

Check: `event zz_tiger_test3.54`, then the `T54` lines in `debug.log`, and `error.log` for `zz_tiger_test3`, `random_faith_in_religion`, `ethnicit` and `islam_religion`.

- A Muslim without an error: the effect takes a religion key, unlike templates; tiger is right. "Not Muslim" or an "event target link" error: it rejects the key like templates; validate it as a faith target only, as for templates.
- C african and D not (or an error for D's `always`): the effect uses the template syntax; apply the template validation. D african and C not: the effect docs form is right; validate `ethnicities` as `{ <ethnicity> = { triggers } }`.
- No error for E and F (and F the same as C): `default` and `template_character` work in the effect too.

**Result (1.20.1.2, from `error.log` at load):** `random_faith_in_religion = islam_religion` gives "Failed to find a valid event target link", so the effect takes only a faith, like templates; tiger's religion key is wrong. D (`african = { always = yes }`) gives "Unexpected token: always", so the effect uses the template syntax for `ethnicities`. No errors for C, E and F. The event itself wasn't run (whether C gets the ethnicity was left out as not needed): the load errors settle what tiger checks.

### T55. Several `history` blocks in `create_artifact` (F65)

The same question as T39, for the effect that creates the artifact. Event:

```paradox
zz_tiger_test3.55 = {
 type = character_event
 hidden = yes
 immediate = {
  create_artifact = {
   name = artifact_chest_name
   description = placeholder
   type = chest
   visuals = chest
   modifier = artifact_placeholder_modifier
   history = { type = given actor = root recipient = root }
   history = { type = purchased actor = root recipient = root }
   generate_history = no
  }
 }
}
```

Check: `event zz_tiger_test3.55`, open the new chest in the inventory and look at its history, and `error.log` for `zz_tiger_test3` and `history`. Only "purchased" (as in T39): one block takes effect; accept `history` once, as in `set_owner`. Both entries: several work in this effect, and tiger is right.

**Result (1.20.1.2):** no errors, and the chest's history shows only "Purchased": as in T39, only the last `history` block takes effect. Tiger should accept `history` once in `create_artifact`, as in `set_owner`.

### T56. Flavorization `target_character = yes` without `top_liege = no` (F24)

No files. The docs give `top_liege` the default yes and make it exclusive with `target_character`; tiger reports only an explicit `top_liege = yes`. Vanilla `monastery_christian` (`common/flavorization/20_pam_flavorization.txt:398-410`) sets `target_character = yes` alone. Check `error.log` after loading for `flavo`, `monastery_christian`, `target_character` and `top_liege`. An error there: the default conflicts; tiger should report it (1 vanilla report). Nothing: leave the check as it is and drop the question.

**Result (1.20.1.2):** no errors. The default doesn't conflict, so the check stays as it is.

### T57. Faiths: the unreformed doctrine from the main rite, and faith-level icon fields (F24)

Tiger computes "unreformed" from the faith's own `doctrines` only and errors on `reformed_icon` otherwise, while the 1.20 docs say the main rite's doctrines "become the faith's effective properties". It also rejects `piety_icon_group` and `tenet_background_icon` in faiths, which the religion and family docs still list with the precedence "faith > religion > family". Vanilla's `baltic_pagan` rite carries `unreformed_faith_doctrine`. File `common/religion/faith_types/zz_tiger_test3_faith_types.txt`:

```paradox
# Unreformed only through its main rite
zz_tiger_test3_rite_pagan = {
 main_rite = baltic_pagan
 faith_details = {
  religion = baltic_religion
  color = { 0.2 0.8 0.2 }
  icon = baltic
  reformed_icon = baltic_reformed
 }
 eminent_holy_sites = { pokaini torun }
 holy_sites = { hiiumaa braslau rugen }
}

# Faith-level icon overrides
zz_tiger_test3_icons = {
 main_rite = baltic_pagan
 faith_details = {
  religion = baltic_religion
  color = { 0.8 0.2 0.2 }
  icon = baltic
  reformed_icon = baltic_reformed
  piety_icon_group = christian
  tenet_background_icon = tenet_banner_unknown_heretical
 }
 eminent_holy_sites = { pokaini torun }
 holy_sites = { hiiumaa braslau rugen }
 doctrines = { unreformed_faith_doctrine }
}
```

Event:

```paradox
zz_tiger_test3.57 = {
 type = character_event
 hidden = yes
 immediate = {
  if = { limit = { faith:zz_tiger_test3_rite_pagan = { has_doctrine = unreformed_faith_doctrine } } debug_log = "T57 rite-only faith: unreformed" }
  else = { debug_log = "T57 rite-only faith: reformed" }
  if = { limit = { faith:baltic_pagan = { has_doctrine = unreformed_faith_doctrine } } debug_log = "T57 control baltic_pagan: unreformed" }
  else = { debug_log = "T57 control baltic_pagan: reformed" }
 }
}
```

Check: first `error.log` after loading for `zz_tiger_test3`, `reformed_icon`, `piety_icon_group` and `tenet_background_icon`. Then `event zz_tiger_test3.57` and the `T57` lines in `debug.log`. Then `effect set_character_faith = faith:zz_tiger_test3_rite_pagan` and open the faith window: does it offer the reformation (an unreformed faith) or not? Finally `effect set_character_faith = faith:zz_tiger_test3_icons`: is the piety icon in the top bar the Christian cross or the pagan one, and are the tenet banners in the faith window red (heretical) or the normal ones?

- Unreformed: "rite-only faith: unreformed" and the reformation is offered: tiger should also read the main rite's doctrines when it computes "unreformed" (no error for `reformed_icon`, and the `_old` loc checks apply). "Reformed": tiger is right.
- Icon fields: a cross and red banners without an "unknown field" error: faiths still read the fields; accept them in `faith_details`. Unknown-field errors and the pagan defaults: the religion docs are stale; tiger is right. An earlier test (2026-10-18, a copy of a vanilla faith with a bogus-key control) already gave "Unexpected token" for both fields at the faith level; if that copy had them in `faith_details` too, this part only repeats it.

**Result (1.20.1.2, partial):** `piety_icon_group` and `tenet_background_icon` in `faith_details` give "Unexpected token": faiths reject them, and tiger is right. The unreformed part is still open.

**Result (1.20.1.2, unreformed half, first run):**
- The test faith shows no tenets, so its main rite wasn't applied. The `baltic_pagan` rite belongs to the faith `baltic_pagan` (its `faith` field). The rite type docs say a faith without scripted rites gets a dynamic rite of its own name.
- The log has "rite-only faith: reformed" and "control baltic_pagan: unreformed". The faith window's rite button reads "Create Branched ... Rite". `CREATE_REFORM_RITE_BUTTON` shows that when `GetFaith.IsUnreformed` is false, and "Reform the ... Rite" otherwise.
- The control has `unreformed_faith_doctrine` in both its faith and its rite, so it only shows that the trigger works.
- The run doesn't settle the question.

Rerun with the faith's own rite: `common/religion/rite_types/zz_tiger_test3_rite_types.txt`, with the faith's `main_rite = zz_tiger_test3_pagan_rite` (the icon faith is gone):

```paradox
zz_tiger_test3_pagan_rite = {
 faith = zz_tiger_test3_rite_pagan
 color = { 0.2 0.8 0.2 }
 tenets = { tenet_sacred_childbirth tenet_ancestor_worship tenet_sanctity_of_nature }
 doctrines = { unreformed_faith_doctrine }
}
```

Check:
1. `error.log` after loading, for `zz_tiger_test3`.
2. `event zz_tiger_test3.57`, then the `T57` lines in `debug.log`.
3. `effect set_character_faith = faith:zz_tiger_test3_rite_pagan`, then the faith window:
   - It must show the three Baltic tenets; if it doesn't, the rite still wasn't applied.
   - "Reform the ... Rite" and "unreformed": the main rite makes the faith unreformed, so tiger should read it (see above).
   - "Create Branched ... Rite" and "reformed": tiger is right.

**Result (1.20.1.2, rerun):** no errors. The log has "rite-only faith: unreformed" and "control baltic_pagan: unreformed". The faith window shows the three Baltic tenets and "Reform the ... Rite". The main rite's doctrines make the faith unreformed, so tiger now reads the main rite's doctrines and tenets too. Together with the first run, the main rite applies only to a faith that owns it. The rite's `faith` field gives the owner, and faith history can assign the rite to another faith from a date on: vanilla `catholic` and `orthodox` use rites of `christian_faith` from 1054. Tiger therefore doesn't filter on the `faith` field.

### T58. Loc of portrait animations with `barbershop = no` (F24)

No files. Tiger requires `<key>` loc for every portrait animation; the five that lack it (`grand_marshal`, `thinking_all`, `prayer_muslim`, `spiritual_fulfillment`, `menacing_wooden_sword` in `gfx/portraits/portrait_animations/animations.txt`) all have `barbershop = no`. Open the barbershop (Change appearance) and the ruler designer, and look through their animation lists: do any of the five appear, under their raw key? None: the name loc is only shown there, so tiger should require it only when `barbershop` isn't `no` (marked `TODO: verify` until another place turns up). Any of them listed: the loc is needed; these are vanilla issues.

**Result (1.20.1.2):** the barbershop's pose list shows no raw keys, so none of the five is listed. The ruler designer has no animation list of its own (`gui/window_ruler_designer.gui`). The only other place found is the character tooltip's animation paragraph (`PORTRAIT_ANIMATION_TITLE`, `[Character.GetAnimationName]`). It is behind `ShouldShowAnimationInfo`, probably a debug option. Tiger should require `<key>` loc only when `barbershop` isn't `no`, with `TODO: verify`.

### T59. `create_holy_site` with a holy site type scope or a holy site scope (F24)

The docs say `type = <holy_site_type_key>/<holy_site_scope_object>`; tiger accepts a key or a holy site type scope (`TODO: verify` in `validate_create_holy_site`), and vanilla never uses the effect. Play a Catholic ruler with at least three provinces, none of them a holy site. Event:

```paradox
zz_tiger_test3.59 = {
 type = character_event
 hidden = yes
 immediate = {
  random_realm_province = {
   limit = { NOT = { barony = { any_barony_holy_site = { always = yes } } } }
   barony = { save_scope_as = zz_b1 }
  }
  random_realm_province = {
   limit = { NOT = { barony = { any_barony_holy_site = { always = yes } } } NOT = { barony = scope:zz_b1 } }
   barony = { save_scope_as = zz_b2 }
  }
  random_realm_province = {
   limit = { NOT = { barony = { any_barony_holy_site = { always = yes } } } NOT = { barony = scope:zz_b1 } NOT = { barony = scope:zz_b2 } }
   barony = { save_scope_as = zz_b3 }
  }
  root.faith = {
   create_holy_site = { type = site_of_valor barony = scope:zz_b1 }
   create_holy_site = { type = holy_site_type:site_of_martyrs barony = scope:zz_b2 }
   create_holy_site = { type = holy_site:uppsala barony = scope:zz_b3 }
  }
  debug_log = "T59 done"
  debug_log_scopes = yes
 }
}
```

Check: `event zz_tiger_test3.59`, then the faith window's holy sites (`debug.log` names the three baronies), and `error.log` for `zz_tiger_test3` and `create_holy_site`. The key form (valor) is the control. A holy site at the second barony: the holy site type scope works. A holy site of the Uppsala type at the third barony: a holy site scope works too. Tiger should accept exactly the forms that worked and drop the TODO.

**Result (1.20.1.2, from `error.log` at load):** `type = holy_site:uppsala` gives "Not found in database HolySiteTypeDatabase", and the effect fails PostValidate: a holy site scope doesn't work. No errors for the key and the holy site type scope, which pass the effect's checks at load. Whether they create holy sites was left out as not needed: tiger accepts exactly these two forms.

### T60. Holy order type `type = mendicant` (F24)

`_holy_orders.info` lists `monastic` and `military` for `type`, but `create_holy_order` takes `random_holy_order_type = mendicant` and the binary has `mendicant` next to `monastic`. File `common/holy_orders/zz_tiger_test3_holy_orders.txt`:

```paradox
zz_tiger_test3_mendicant = {
 type = mendicant
 trigger = { always = no }
 weight = { value = 0 }
 name = "zz_tiger_test3_mendicant"
}
zz_tiger_test3_bogus = {
 type = zz_bogus
 trigger = { always = no }
 weight = { value = 0 }
 name = "zz_tiger_test3_bogus"
}
```

Check `error.log` after loading for `zz_tiger_test3` and `mendicant`. The bogus type is the control and should log an error. No error for the mendicant type: it is a valid value; accept it with `// undocumented`. The same error as the bogus type: only `monastic` and `military` exist, and `random_holy_order_type = mendicant` deserves a warning.

**Result (1.20.1.2):** both test types also lacked `government` ("Field 'government' not set"), and both logged "missing type, falling back to 'military'", the message the `.info` gives for a missing type. `mendicant` fails like the bogus control: only `monastic` and `military` exist, and tiger is right. `random_holy_order_type = mendicant` in `create_holy_order` deserves a warning.

## CK3 1.20: false positives claimed by Unop fixes (T62)

### T62. Scalar `weight` in `random_traits_list`

Vanilla's `ce1_hooded_figure_template` writes `weight = 30` in its `random_traits_list` entries, and tiger reports "expected block, found value": `validate_random_traits_list` accepts only a block, and the docs (`weight ( mtth )`) show only `weight = { base = 1 }`. A Unop repro (Unop issue #808) found tier-5 educations at most 25 in 200 with weights 30 and 5, against about 50 if the weights were ignored, but it lost the exact count, and a low count could have other causes. This test reverses the weights, so a high count can only come from them. File `common/scripted_character_templates/zz_tiger_test3_templates.txt`:

```paradox
zz_tiger_test3_scalar_weight = {
	age = 30
	gender = male
	culture = root.culture
	faith = root.faith
	dynasty = none
	random_traits = no
	random_traits_list = {
		count = 1
		education_martial_2 = { weight = 1 }
		education_martial_3 = { weight = 1 }
		education_martial_4 = { weight = 1 }
		education_martial_5 = { weight = 100 }
	}
}
```

Plus two controls that differ only in the weights: `zz_tiger_test3_block_weight` with `weight = { base = 1 }` and `weight = { base = 100 }`, and `zz_tiger_test3_no_weight` with empty entries `{}`. Event, with the same loop for each template:

```paradox
zz_tiger_test3.62 = {
	type = character_event
	hidden = yes
	immediate = {
		while = {
			count = 100
			create_character = { location = root.capital_province template = zz_tiger_test3_scalar_weight save_scope_as = zz_c }
			scope:zz_c = {
				if = { limit = { has_trait = education_martial_5 } debug_log = "T62 scalar: tier 5" }
				else_if = { limit = { OR = { has_trait = education_martial_2 has_trait = education_martial_3 has_trait = education_martial_4 } } debug_log = "T62 scalar: tier 2-4" }
				else = { debug_log = "T62 scalar: no list education" }
				silent_disappearance_effect = yes
			}
		}
		# The same loop for zz_tiger_test3_block_weight ("T62 block") and zz_tiger_test3_no_weight ("T62 none")
		debug_log = "T62 done"
	}
}
```

Check: `event zz_tiger_test3.62`, then count each line in `debug.log` (`grep -c "T62 scalar: tier 5"`, and so on), and search `error.log` for `zz_tiger_test3_templates`. The block control should give about 97 tier 5 in 100 (100 of 103), and the no-weight control about 25. Scalar near 97: the scalar weight works, tiger should accept it (likely also in `random_culture` and `random_faith`, whose docs give the same mtth weight), and the report is a false positive. Scalar near 25, or "no list education": the scalar is ignored, and the vanilla template's education weights are a real bug.

**Result (1.20.1.2):** no errors. Tier 5 came up 98 times in 100 for the scalar weights, 96 for the block control and 24 for the no-weight control, with no "no list education". The scalar weight works, so tiger's report is a false positive, as PR #840 says.

