# Changes to existing checks

Checks that existed before a game update and behave differently after it: relaxed, tightened, or both. This is what reviewers of an update need most. Checks for content new in that version, refactors, and plain additions of documented entries are left out. Fixes of pre-existing gaps (see [follow-up.md](follow-up.md)) get a row too.

- **Direction:** relaxed (fewer reports), tightened (more reports), or both.
- **Evidence:** the strongest source: the docs of that version, the binary, an in-game test ([game-tests.md](game-tests.md)), vanilla usage, or a bug fix. "Vanilla agrees" means vanilla uses the form the docs give.
- **Commit:** the commit on the update branch.

## CK3 1.20

| Change | Direction | Evidence | Commit |
| --- | --- | --- | --- |
| Faiths no longer need a holy site | relaxed | in-game test | `59a0e7af` |
| Faiths no longer accept `holy_order_maa` | tightened | in-game test | `59a0e7af` |
| `duplicate doctrine` reported for any repeated doctrine, not only grouped ones | tightened | bug fix | `c97b8f97` |
| 7 stale defines removed (`NCoatOfArms\|ENTITY_*_COA_FRAME_*`, `NReligion\|DEFAULT_FERVOR`) | tightened | 1.20 defines | `bd1e02df` |
| Holy site `parameters` accept the `name = yes` form | relaxed | 1.20 docs | `7207feb3` |
| `holy_site_<key>_effects` loc no longer required | relaxed | 1.20 docs; in-game test | `7207feb3` |
| Interaction `target_filter` values checked against the documented list | tightened | 1.20 docs; vanilla agrees | `0518d374` |
| Interaction `needs_confirmation` gets a deprecated warning | tightened | 1.20 docs | `0518d374` |
| Bookmark character `religion` compared with the rite in history (the old check never fired); a history with only a faith compared through the faith's main rite | tightened | vanilla usage; 1.20 docs (faith `main_rite`, faith history) | `39af1650`, `b368b661` |
| Map environment `tonemap_function` accepts `Filmic_Hable` instead of `Filmic` | both | 1.20 docs; binary | `09c203f8`, `a25851f5` |
| Activity `guest_join_chance`/`max_guests`: option category keys are flag scopes | both | 1.20 docs; vanilla agrees | `2db65eab` |
| Law group `can_change_law_group` no longer validated as a law | relaxed | 1.20 docs | `77ba7a8a` |
| Casus belli `on_*_desc` get the effect scopes | relaxed | 1.20 docs; vanilla agrees | `9cce01a4` |
| Casus belli `should_invalidate` gets the effect scopes (TODO: verify) | relaxed | 1.20 docs; vanilla agrees | `9cce01a4` |
| Casus belli group `allowed_for_character` gets `scope:attacker`, `scope:defender`, `list target_titles` | relaxed | 1.20 docs | `9cce01a4` |
| Casus belli lists `attackers`/`defenders` hold characters (TODO: verify) | both | 1.20 docs; vanilla agrees | `9cce01a4` |
| Innovation and era `maa_upgrade` accept both `type` and `men_at_arms` (TODO: verify) | relaxed | vanilla usage | `1ee72102` |
| Decision `advice` accepts several `per_title_hint` | relaxed | 1.20 docs; vanilla agrees | `811fcd4d` |
| Decisions with the `create_holy_order` controller get `scope:barony` | relaxed | 1.20 docs; vanilla agrees | `811fcd4d` |
| Domicile buildings: `on_canceled` -> `on_cancelled` | both | 1.20 docs; binary | `17a4c98a` |
| Game concept `texture = legitimacy` not checked as a file | relaxed | 1.20 docs; vanilla agrees | `7c9fd653` |
| Factions accept only `desc`; `<key>_desc` required only without it | both | 1.20 docs; vanilla agrees | `e8317eb1` |
| Court position `sort_order` rooted in the liege | relaxed | 1.20 docs; vanilla agrees | `e8317eb1` |
| Governments with `vassal_contract_group` need `<key>_vassals_label` (the old check never fired) | tightened | 1.20 docs; vanilla agrees | `e8317eb1` |
| Government rules outside `government_rules` are an error | tightened | 1.20 docs | `e8317eb1` |
| Government `administrative_title_maa_setup`: `main_tier_and_top_liege` | both | 1.20 docs; binary | `e8317eb1` |
| Great project `group` checked against the documented values | tightened | 1.20 docs; vanilla agrees | `2db65eab` |
| Situation-level `on_join`/`on_leave` rooted in the character, with `scope:situation` | both | 1.20 docs; vanilla agrees | `75a775f7` |
| Situation `max_duration_next_phase` takes `weighted_non_takover` (sic) | both | 1.20 docs; binary | `75a775f7` |
| Scheme `base_success_chance` rooted in the scheme, with the scheme scopes | both | 1.20 docs; vanilla agrees | `af960214` |
| Court scene role `effect` gets `scope:ruler`, its `scripted_animation` `scope:character` | relaxed | 1.20 docs; vanilla agrees | `09c203f8` |
| Court scene settings accept `requires_dlc_flag` (undocumented, but consistent vanilla usage) | relaxed | vanilla usage; binary | `09c203f8` |
| Opinion modifier `obedient` no longer reported as removed | relaxed | 1.20 docs; vanilla agrees | `e688e8b6` |
| Decisions need `ai_check_interval` or `ai_check_interval_by_tier` unless `ai_goal = yes` | tightened | 1.20 docs; vanilla agrees | `811fcd4d` |
| Opinion modifiers: `growing` with `decaying`, `monthly_change` with a duration are errors | tightened | 1.20 docs; vanilla agrees | `6e3ae8f1` |
| `field_script_value_no_breakdown_rooted` no longer checks `desc` loc (all games) | relaxed | bug fix | `b0a7aaea` |
| Activity group types need `activity_group_type_<key>` loc | tightened | 1.20 docs; vanilla agrees | `2db65eab` |
| `is_manual_participant` is a situation trigger with a character target | both | 1.20 docs; vanilla agrees | `a25851f5` |
| `change_government` accepts a government type scope | relaxed | 1.20 docs; vanilla agrees | `a25851f5` |
| Scripted rules define their documented lists (CK3, Vic3) | both | bug fix | `a25851f5` |
| Music `subsequent_playback_chance` takes any number of chances | relaxed | 1.20 docs; vanilla agrees | `b9389eab` |
| Bookmark characters with `display = no` don't need a location image | relaxed | 1.20 docs; vanilla agrees | `b9389eab` |
| Culture era `invalid_for_government` can be repeated | relaxed | 1.20 docs; vanilla agrees | `b9389eab` |
| Tutorial lesson step `interface_effect` rooted in the player character | relaxed | 1.20 docs; vanilla agrees | `b9389eab` |
| Factions accept `on_declaration` next to `on_war_start` | relaxed | 1.20 docs | `b9389eab` |
| Scripted animation `triggered_animation` accepts `scripted_animation` | relaxed | 1.20 docs; in-game test | `4a0c557c` |
| On actions with several `events`/`on_actions` blocks no longer warned about | relaxed | in-game test | `4a0c557c` |
| Governments: `primary_cultures`, AI `imprison`, `start_murders` are unknown | tightened | in-game test | `4a0c557c` |
| Culture traditions and pillars accept a dynamic `desc` rooted in the culture | relaxed | 1.20 docs; in-game test | `4a0c557c` |
| `on_tradition_added`/`on_tradition_removed`: `scope:tradition` is a tradition | both | in-game test | `4a0c557c` |
| Character history: entries before birth or after death reported; one spouse per date entry | tightened | 1.20 docs | `abd6b31a` |
| Culture `dlc_fallback_pillar` accepted, once | relaxed | 1.20 docs; binary | `abd6b31a`, `3d86ed2e` |
| Era and innovation `maa_upgrade` accept `max_size`; docs-mismatch advice for `pursue` | relaxed | 1.20 docs | `abd6b31a` |
| Flavorization: `type = domicile` needs `domicile_type`; `holding` only for baronies | tightened | 1.20 docs; vanilla agrees | `abd6b31a` |
| Great project contribution name loc key corrected | both | 1.20 docs; vanilla agrees | `abd6b31a` |
| Building `province_terrain_modifier` accepts `is_riverside` | relaxed | 1.20 docs; binary | `abd6b31a` |
| Claimant factions get `scope:claimant`, `scope:title` in `ai_create_score` | relaxed | 1.20 docs | `abd6b31a` |
| Law `<key>_effects` loc is optional | relaxed | 1.20 docs | `abd6b31a` |
| Opinion modifiers: `decaying` needs `monthly_change` or a duration; no negative `monthly_change` | tightened | 1.20 docs | `abd6b31a` |
| Undocumented scripted costs reported | tightened | 1.20 docs; vanilla agrees | `abd6b31a` |
| Game concept aliases that clash with a concept or another concept's alias are errors; an alias repeated in its own concept is a warning | tightened | 1.20 docs | `abd6b31a`, `eb8b06e9` |
| Interactions with `target_type = court_position_type` get `scope:target` | relaxed | 1.20 docs; vanilla agrees | `abd6b31a` |
| Struggles: an ending phase counts as an ending | relaxed | 1.20 docs | `abd6b31a` |
| Effect loc fallbacks `_global_past`/`_first_past` (typo fixed) | relaxed | 1.20 docs; vanilla agrees | `abd6b31a` |
| Event court scene: `scripted_animation` per role, not at the `court_scene` level | both | 1.20 docs | `abd6b31a` |
| Portrait animation variants accept several `portrait_modifier` blocks | relaxed | 1.20 docs | `abd6b31a` |
| Music accepts `reset` and `dlc` | relaxed | 1.20 docs | `abd6b31a` |
| Situation future phases: `takeover_points` and `takeover_duration` exclusive, with a matching `takeover_type` | tightened | 1.20 docs; binary | `abd6b31a` |
| `set_coa` accepts organization and confederation targets | relaxed | 1.20 docs | `abd6b31a` |
| `create_title_and_vassal_change` rejects `type = destroyed` | tightened | 1.20 docs (the full list is in the binary) | `abd6b31a`, `3d86ed2e` |
| `remove_barter_goods` takes a script value (its effect loc shows `$VALUE$`) | relaxed | vanilla usage | `abd6b31a` |
| Duplicate table entries removed (`decline_invitation_for_character`, `levies_to_liege`, `is_riverside_province`) | tightened | 1.20 docs | `abd6b31a` |
| Script value `round_to = 0` is an error | tightened | 1.20 docs | `abd6b31a` |
| Unused interaction categories are errors | tightened | 1.20 docs | `abd6b31a` |
| Music group templates and their fields checked | tightened | 1.20 docs | `abd6b31a` |
| Activity pulse action icons use `path_overrides.txt` | both | vanilla usage | `b65ee582` |
| A doctrine without an icon shows its group's icon; group icons optional | relaxed | in-game test | `84307280` |
| Council task modifier `scale` gets the task's scopes | relaxed | 1.20 docs; vanilla agrees | `9d42bbc1` |
| `great_project_type` usable as a link in chains; `cancel_great_project`/`complete_great_project = this` accepted | relaxed | 1.20 docs; vanilla agrees | `579057fa` |
| `change_struggle_phase` block without `with_transition` accepted | relaxed | 1.20 docs; vanilla agrees | `f6992d20` |
| `squared_distance` takes a title or province; `vassal_contract_obligation_level_score` a contract type | relaxed | 1.20 docs; vanilla agrees | `c2c6d07d` |
| `Faith.Custom` also takes faith custom localization | both | 1.20 docs; vanilla agrees | `0e29c958` |
| `hex { }` colors (CK3); `relative_fog_color` is three numbers | relaxed | 1.20 docs; vanilla agrees | `8469802e` |
| Portrait modifier triggers in animations, sets and packs get the portrait scopes (TODO: verify) | relaxed | vanilla usage | `c62f5c56` |
| On actions with several `first_valid`, `random_on_action` or `first_valid_on_action` blocks no longer warned about | relaxed | in-game test | `f063dca0` |
| Province history `religion` is unknown | tightened | 1.20 docs; in-game test | `f063dca0` |
| `on_artifact_durability_low` gets `scope:artifact` | relaxed | in-game test | `f063dca0` |
| Loc `#TOOLTIP:` tags: 14 more tags known (database keys or runtime ids), `GAME_FAITH_DOCTRINE` dropped, `$PARENT_KEY$` built in | both | in-game test | `f063dca0` |
| `set_relation`: `<reason>_corresponding` loc counts as used | relaxed | in-game test; vanilla agrees | `3d86ed2e` |
| Artifact `set_owner` accepts `history` once | tightened | in-game test | `8c8a2f4d` |
| Loc `#TOOLTIP:` tags `FULFILLMENT_LEVEL`, `GREAT_PROJECT`, `SITUATION` take runtime ids | relaxed | in-game test | `8c8a2f4d` |
| `accolade_type_tier` takes a tier name (`common`, `skilled`, `eminent`), not an accolade type | both | in-game test | `6caf8fc1` |
| Custom localization keys are optional with `if_invalid_loc = return_empty` or `fallback_to_next_entry` (CK3 only) | relaxed | 1.20 docs | `abd6b31a`, `568695d1` |
| A live trigger takes the place of a removed link of the same name at a chain's end (`house_power`; all games) | relaxed | script docs; vanilla usage | `eb5b2093`, `09712614` |
| `maa_upgrade` `type` takes only a base men-at-arms type, no longer a men-at-arms type as well | tightened | docs; vanilla agrees | `25cf791c` |
| Decisions with the `create_holy_order` controller no longer get `scope:ruler` outside `title_valid` | tightened | 1.20 docs; vanilla agrees | `b971a3b9` |
| Diarchy `liege_modifier` and `diarch_modifier` need a `name` | tightened | 1.20 docs; binary; vanilla agrees | `77614975` |
| A faith is unreformed if a doctrine or core tenet of the faith, its religion or its main rite makes it so, not only its own doctrines (`reformed_icon`, the `_old` locs) | both | 1.20 docs; in-game test | `027ce778`, `a15f919b` |
| Doctrine `piety_cost` no longer gets `selected_doctrines` (tenet `piety_cost` keeps it, `TODO: verify`) | tightened | 1.20 docs; vanilla agrees | `a316bc9a` |
| Doctrine and tenet parameters need their `doctrine_parameter_<name>` loc (`TODO: verify`, T64), and `parameter_opinion_override` keys need `doctrine_parameter_<key>_opinion_active` | tightened | 1.20 docs | `a316bc9a` |
| Dynasty and house `forced_coa_religiongroup` gets "not used by the game" | tightened | 1.20 docs | `e3ff46d7` |
| A name in two name equivalency lists of the same gender is an error; a name listed twice in one list is untidy | tightened | 1.20 docs | `1ee72102`, `35155926` |
| Holy site `faith_character_modifier` no longer accepts `name` (it came from the old `character_modifier`) | tightened | 1.20 docs; vanilla agrees | `c00bb8fb` |
| A message without `title` or `desc` needs the `<key>` or `<key>_desc` loc | tightened | 1.20 docs; vanilla agrees | `7064d9e7` |
| Scheme `phases_per_agent_charge` is an integer, not a script value | tightened | 1.20 docs; vanilla agrees | `6aa9390f` |
