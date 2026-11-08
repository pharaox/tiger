# Follow-up work for tiger (CK3)

Known gaps in tiger's CK3 support, found during game updates but out of their scope: problems that predate an update, or work that was postponed. Done items are removed; the numbers stay, so references in issues and commits remain valid. Items filed upstream link their issue. Game tests (T numbers) are in [game-tests.md](game-tests.md); a fix that changes an existing check gets a row in [changed-checks.md](changed-checks.md).

- **Effort:** S = an hour or two, M = half a day to a day, L = several days.
- **Value:** H = removes many false positives or catches common real bugs, M = useful, L = marginal.
- **Vanilla:** what a run on vanilla shows (CK3 1.20 unless noted).

## F1. Scope type deductions across runtime type checks

**Effort:** L. **Value:** H. **Issue:** [amtep/tiger#351](https://github.com/amtep/tiger/issues/351).

Tiger deduces the type of a named scope from how shared scripted items use it (scripted triggers, effects, values and modifiers), and checks every caller against that deduction. It doesn't understand the runtime checks that let one item handle several types, so callers of such items get false positives. This is a major source of tiger false positives, well beyond the schemes where we saw it.

Example from schemes, whose `scope:target` is a character, title, culture or faith: `agent_groups_target_character_perspective_value` first checks `exists = scope:target.holder` and only then uses the target as a title; `ai_agent_join_chance_basic_suite_modifier` uses character triggers on the target only inside `scope:scheme = { is_scheme_target_type = character }`. About 25 vanilla "expects scope:target to be X" reports in scheme types come from this, and the deductions depend on which caller is validated first, which also makes tiger's output nondeterministic (F8).

Narrowing `scope:target` to the type that `target_type` names (tried on 2026-10-24) made it worse: about 20 new vanilla false positives, 1 gone. Fixing it means recognizing such checks (existence checks on type-specific links, `is_scheme_target_type`, type triggers in `limit`s) and not carrying deductions out of the guarded branches, or validating shared items per caller. Complex; needs care not to lose real reports.

## F3. Event `soundeffect`: only `none` has an effect

**Effort:** S. **Value:** L. **Issue:** [amtep/tiger#383](https://github.com/amtep/tiger/issues/383).

The 1.20 event docs say `soundeffect = none` mutes the event, and other values are read but not used. An untidy warning for other values. Vanilla: 19 new untidy reports (`in_the_dungeon_event`, `feast_event`), which the Unop would have to ignore. Relies on one docs sentence.

## F4. Rules that a key must exist

**Effort:** M. **Value:** L.

Graphical unit type `default`, the two guest system keys, court scene roles `ruler` and `spouse`, the color palette `coa_designer_background_colors`, every named scripted cost, a customization rule per portrait type group, the activity group types `activities`, `grand`, `unavailable`, `joinable` and `invitations`, and the casus belli groups `religious` and `de_jure`. Needs a location convention for a report about something missing (tiger reports need a token; `prov_terrain.rs` already has a TODO about this) and a cross-item check. Only matters for mods that replace or remove the vanilla files. Vanilla: 0 reports.

## F5. Geographical region loops and nesting depth

**Effort:** M. **Value:** L to M. **Issue:** [amtep/tiger#382](https://github.com/amtep/tiger/issues/382).

The new 1.20 docs say nesting deeper than 20 levels is an error ("probable loop"). A cross-item depth-first check over regions, like `NameEquivalency::validate_all`. Tiger already reports a missing sub-region as a crash. Vanilla: 0 reports. Catches crash-class mistakes in map mods.

## F6. Load the folders tiger doesn't read

**Value:** M overall (coverage). **Issue:** [amtep/tiger#357](https://github.com/amtep/tiger/issues/357). Each its own PR, with a test mod and vanilla runs; new checks with no vanilla signal can add false positives.

- a. `gfx/models/units/entity_links`, effort S: entity names, types, cultures, gender.
- b. `gfx/models/units` unit states, effort S to M: unclear what to validate beyond the state names.
- c. `gfx/portraits/customization_rules` (new 1.20 `.info`), effort M: genes, modifier groups, cameras, loc keys.
- d. `gfx/skins/hud_skins`, effort S: `is_valid` and `is_shown` triggers, `.skin` name match.
- e. `tests/` (scripted tests, new 1.20 `.info`), effort M: triggers and effects with history targets.
- f. `gfx/interface/illustrations/loading_screens` (not in #357 yet), effort S: the `<key>.dds` next to the file, `priority` an integer above 0, `show_first` a bool (`_loadscreen.info:1-3`). Vanilla and DLCs have 16 entries; big mods use it. `src/fileset.rs:802` already exempts its `.dds` from the unused-file check.
- g. `gui/scripted_widgets` (not in #357 yet), effort S to M: `<gui file> = <widget name>` (`_scripted_widgets.info:4`); check the file exists and defines the widget, and allow duplicate entries (`:7`). No vanilla use, but mods use it (artifact_trade).

## F7. End-to-end tests per `.info` file

**Effort:** L. **Value:** H.

One test mod per area under `tests/files/`, run by `tests/test.rs`: deliberate mistakes asserted by file, line and message, valid and legacy forms alongside. Expect them to find existing bugs. On each later game update, update them first. Agreed earlier: after 1.20 ships.

## F8. Deterministic output

**Effort:** M. **Value:** M (for maintainers and update work, not for mod authors). **Issue:** [amtep/tiger#325](https://github.com/amtep/tiger/issues/325).

The local hasher patch (`skills/update-ck3/scripts/deterministic-hasher.patch`, a fixed-seed `TigerBuildHasher`) plus `RAYON_NUM_THREADS=1` gives byte-identical runs. Multi-threaded runs still differ (about 1.7%) because of the scripted-item validation cache race and the global variable-scope registry. xTiger fixes both, the latter by weakening checks; ours shouldn't. Check upstream first. Related to F1.

## F9. Audit shared ScopeContexts across fields

**Effort:** M to L. **Value:** M. **Issue:** [amtep/tiger#352](https://github.com/amtep/tiger/issues/352).

Many validators pass one `ScopeContext` to several fields (e.g. subject contracts, the remaining scheme fields). Tiger reports a missing scope once per context, so one field's report hides the others', and saved scopes leak between fields. 1.20 examples: the casus belli descs hid 3 `scope:war` reports; splitting the scheme contexts surfaced a vanilla `scope:actor` bug. Give each field its own builder where the docs give per-field scopes. Vanilla: some new (real) reports.

## F10. `create_character`: `ethnicity` and `ethnicities` from the template docs

**Effort:** S. **Value:** M. Game test T54 (T4 settled the template side). **Issue:** [amtep/tiger#375](https://github.com/amtep/tiger/issues/375).

The new 1.20 scripted character templates `.info` documents the shared parser: `ethnicity` also takes `default` and `template_character`, and `ethnicities` is `{ <ethnicity> = { trigger weight } }`. Templates now use that; the `create_character` effect still has "TODO: Find out the syntax of this" and accepts any block. Apply the same validation there (`effect_validation.rs`).

Also `random_faith_in_religion`: the docs of both say it takes a key (a religion key for the effect, a faith key for templates) or a faith scope, but in templates the game rejects both keys as event targets (game test T4). T54 checks whether the effect behaves the same, and which `ethnicities` form it takes.

Tested in game (T54, from `error.log` at load): the effect rejects `random_faith_in_religion = islam_religion` ("Failed to find a valid event target link"), like templates, so it takes only a faith target. The effect-docs form `ethnicities = { african = { always = yes } }` gives "Unexpected token: always", so the effect uses the template syntax. `ethnicity = default` and `template_character` load without errors. Apply the template validation to the effect.

## F11. Messages: default loc keys

**Effort:** S. **Value:** M. **Issue:** [amtep/tiger#356](https://github.com/amtep/tiger/issues/356).

Both messages `.info` versions say `title` defaults to the message key; 1.20 adds that `desc` defaults to `<key>_desc`. Tiger checks neither when the field is absent. Needs a vanilla run to see how many vanilla messages rely on missing defaults.

## F12. Province terrain: read every file, mixing terrains and properties

**Effort:** M. **Value:** L to M. **Issue:** [amtep/tiger#358](https://github.com/amtep/tiger/issues/358). Game test T3 confirmed the docs: the game reads both from any file.

Rewrite `ProvinceTerrains` and `ProvinceProperties` (`prov_terrain.rs`) to read every file in `common/province_terrain` and take assignments and blocks from any of them, instead of picking the files by name. Fixes ignored mod files and false "unexpected key" reports for mixed files.

## F13. Modifiers with no loc at all

**Effort:** M. **Value:** M.

89 modifiers in the tables have no loc in vanilla; 80 of the reported keys are the same as in V19. Investigate which really need loc (`SPECIAL_MODIF_LOC_TABLE`, generated ones) and which tiger reports wrongly.

## F16. Reorder validators to `.info` order

**Effort:** M. **Value:** L. You chose to skip it for now (2026-10-24).

Easier side-by-side review of validators against the docs. Mostly cosmetic, but field order can affect shared ScopeContext state (see F9) and report order, and it bloats diffs.

## F17. Move `CharacterBackground` out of `pool.rs`

**Effort:** S. **Value:** L. Raised 2026-10-24; I suggested keeping it, you didn't push.

It's a documented item type of its own since 1.20 (`character_backgrounds` `.info`).

## F18. Governments must be listed in `GOVERNMENT_TYPES`

**Effort:** S. **Value:** M. **Issue:** [amtep/tiger#354](https://github.com/amtep/tiger/issues/354).

A government type that isn't in the define `NGovernment|GOVERNMENT_TYPES` gets no generated modifiers: the game logs "Could not find the preregistered modifier type '<key>_opinion' ... Are you sure the government type '<key>' is included in the define GOVERNMENT_TYPES?" for each of them (game test T15). Tiger knows the define as a list of government types but doesn't check that every government is in it. Vanilla: probably 0 reports.

## F20. Trigger localization fallback

**Effort:** M. **Value:** M. **Issue:** [amtep/tiger#355](https://github.com/amtep/tiger/issues/355).

The 1.20 trigger localization docs give the fallback keys `<trigger>_<rule>` / `not_<trigger>_<rule>` and `<trigger>` / `NOT_<trigger>`; tiger checks `<key>_<rule>_not`. The check never runs for `custom_description`, because tooltipped is set to no before it (`src/trigger.rs`). Turning it on enables a check that never fired, so it needs its own vanilla classification.

## F24. Open questions for game tests

**Effort:** S each. **Value:** L to M.

- Holy orders: warn about `random_holy_order_type = mendicant` in `create_holy_order`, since `mendicant` is not a holy order type (game test T60). **Issue:** [amtep/tiger#380](https://github.com/amtep/tiger/issues/380).

Settled: `portrait_group` after a modifier is now a warning (`f3f9879f`, closes #365 on merge); domicile `move_cost` is game test T29, filed as #334; `scope:new_rite` in `on_rite_created` is not set, so tiger dropped it (game test T11); `create_holy_site` takes a holy site type key or scope, not a holy site scope (game test T59, `ae7660b3`); faiths reject `piety_icon_group` and `tenet_background_icon` (game test T57), and holy order types reject `mendicant` (game test T60), as tiger says; a main rite with `unreformed_faith_doctrine` makes its faith unreformed, so tiger now reads the main rite (game test T57); flavorization `target_character = yes` without `top_liege = no` is no error, so tiger is right not to report it (game test T56); the loc of portrait animations with `barbershop = no` is F96 (game test T58).

## F25. Validate scripted animations at their call sites

**Effort:** M. **Value:** M. **Issue:** [amtep/tiger#359](https://github.com/amtep/tiger/issues/359).

The 1.20 docs say a scripted animation's trigger scopes "come from the context using the animation" (event portraits, court scene roles, activity windows). Tiger therefore validates named scripted animations with non-strict scopes and a character or activity root, so a misspelled `scope:` in their triggers is never reported. Validating each named animation with the scopes of each place that refers to it, as tiger does for scripted triggers, would bring strictness back.

## F27. Variables that hold several scope types

**Effort:** M. **Value:** H. Game test T51. **Issue:** [amtep/tiger#329](https://github.com/amtep/tiger/issues/329).

Tiger deduces one type per variable or variable list from all its uses, and warns when a use doesn't match. 1.20 vanilla deliberately stores either a flag or a doctrine in `current_debate_proposal` (`pam_ecumenical_council_effects.txt:391-413`, with the comment "tenet-status compares are type-unsafe vs a doctrine"), and the other `*_debate_proposal`, `pilgrim_badge` and regency variables mix types too. Vanilla 1.20: about 970 reports, 822 from the debate proposals alone. Investigate whether mixed-type variables work in game (a `?=` compare against the other type: false, or an error in `error.log`?), and whether tiger should allow a set of types per variable and warn only about uses that match none of them. Related to F1.

Tested in game (T51): storing several types in one variable works, but every compare against another type logs "Left side and right side during comparison were of different types" and is false, with `?=` as well as `=`. So tiger should allow a set of types per variable but keep reporting compares that can meet another type. Vanilla avoids most of them on purpose (`current_debate_proposal` is kept a flag when the proposal is a doctrine). Posted on #329.

## F28. Code-supplied `$PARAM$` placeholders in loc

**Effort:** M. **Value:** H. **Issue:** [amtep/tiger#328](https://github.com/amtep/tiger/issues/328).

Tiger reports "The substitution parameter $X$ is not defined anywhere as a key" unless `X` is a loc key or in `BUILTIN_MACROS` (`src/ck3/tables/localization.rs`, "LAST UPDATED CK3 VERSION 1.12.3", "collected by analyzing tiger's own output"). The engine fills many placeholders from code, so most reports are false positives: 2,763 on vanilla 1.20 (2,317 in V19), +446 from new 1.20 GUI loc (`$HOSTILITY_NAME$`, `$OLD_TENET$`, `$NEW_TENET$`, `$IS_LAST_ITEM$`, ...). A few are real (`$holy_order_poor_clares$`, the Japanese `$cathedral_royal_nave_01_domicilebuilding$`). Refresh the table for 1.20, checking that each candidate is only used where code supplies it (GUI-only keys, for example), or find a better rule.

## F29. Icons with arguments in loc (`@icon:frame:color!`)

**Effort:** S. **Value:** M. **Issue:** [amtep/tiger#327](https://github.com/amtep/tiger/issues/327).

Vanilla uses `@aptitude:1:inherit_color!` and similar, an icon with frame and color arguments. Tiger's loc parser expects `!` right after the icon name and reports "Unexpected character `:`, expected `!`": 467 on vanilla 1.20 (350 in V19). Confirm the syntax (binary strings, other Jomini games), then parse the arguments, checking the frame against the icon's frame count where tiger knows it.

## F31. GUI type instances that set their own datacontext

**Effort:** M. **Value:** L. **Issue:** [amtep/tiger#360](https://github.com/amtep/tiger/issues/360).

The type `icon_tenet_popularity` (`gui/window_faith.gui:2670`) uses `ScriptedGui.IsValid( GuiScope.SetRoot( TenetType.MakeScope ) ... )` and notes "requires scripted gui datacontext". Each instance sets `datacontext = "[GetScriptedGui( 'tenet_popularity_gui_*' )]"` (scope = tenet), but tiger evaluates the type's body with the parent's datacontext (`using_tenet_popularity_gui`, scope = faith) and reports "SetRoot scope does not match scripted gui scope". 1 vanilla report. Check how tiger merges instance and type properties, and apply an instance's datacontext before the type's other properties.

## F32. Event descs guarded by `exists = scope:x` when called without that scope

**Effort:** M. **Value:** M. **Issue:** [amtep/tiger#346](https://github.com/amtep/tiger/issues/346).

`religious_interaction.1` and `.6` are triggered from the divorce interaction, without `barony_to_lease`, `lease_order` and `joining_order_title`, and from `pam_interactions_events.0060`, with them. The descs that use those scopes are guarded by `exists = scope:price_lease`, which only the second path sets, but tiger validates them for the first caller too: 52 loc reports. If a `triggered_desc`'s trigger requires `exists = scope:x` and the caller's context can't have `scope:x`, the desc is unreachable from that caller and could be skipped. How much of vanilla's 1,272 strict-scope reports this explains is unknown. Related to F1.

## F33. Named scopes that are also game concepts

**Effort:** S. **Value:** M. Game test T47. **Issue:** [amtep/tiger#370](https://github.com/amtep/tiger/issues/370).

When a loc chain starts with a name that is both a saved scope and a game concept (`friend`, `rival`, `court_chaplain`, `family`), tiger warns that the game concept takes precedence, then reports "`friend` returns CString but a scope type is needed here". 168 + 161 vanilla reports, mostly 1.20 PAM loc. If the precedence claim holds, these are vanilla bugs (the text shows the concept instead of the character); if not, they are false positives. A game test settles it: an event that saves `scope:friend` and shows `[friend.GetFirstName]`.

Tested in game (T47): at the start of a chain the saved scope wins, so the reports are false positives. `friend.Self.GetFirstName` fails ("Could not find promote for 'Self'"), so tiger's `.Self` advice is wrong too.

## F34. Portrait light cone angles

**Effort:** S. **Value:** M. Game test T50. **Issue:** [amtep/tiger#371](https://github.com/amtep/tiger/issues/371).

`src/data/environment.rs` accepts `inner_cone_angle` and `outer_cone_angle` only for `point_light` and reports them as untidy elsewhere, following the vanilla comment `# For point lights` in `portrait_environments.txt`. Cone angles normally belong to spot lights, and vanilla sets them on spot lights: 705 reports each. Check the binary strings and the other Jomini games for which light type reads them; if it's spot lights, the vanilla comment is wrong and the check should follow the engine.

Tested in game (T50): narrowing the cone angles of a spot light shrinks its lit area, so spot lights read them and the 1,410 reports are false positives. Whether point lights read them is untested.

## F35. Advice-level reports that 1.20 content adds

**Effort:** S. **Value:** L.

`AND with only one item` (+114), `suggest-localization` (+340) and `OR is a too complex trigger to be tooltipped` (+30) are pre-existing tips and warnings that more vanilla content trips. Sample them for false positives; no problem is known.

## F38. `needs_prefix` for the remaining global links

**Effort:** S. **Value:** L. **Issue:** [amtep/tiger#361](https://github.com/amtep/tiger/issues/361).

`needs_prefix` (`src/ck3/scopes.rs`, marked 1.16.0) tells the user which `prefix:` a bare key needs when a scope is expected. It lacks 9 of the 1.20 global links that take data, all of them already present in 1.19: `casus_belli_type`, `confederation_type`, `contract_type`, `court_position_type`, `culture_innovation`, `great_project_type`, `situation`, `task_contract_type` and `trait`. Only the hint is missing; the bare key is still reported. Add them (with the matching `Scopes` and `Item` checks) and update the marker.

## F40. Report `this` and `prev` passed as a parameter

**Effort:** M. **Value:** M. **Issue:** [amtep/tiger#317](https://github.com/amtep/tiger/issues/317) (we filed #353, closed as its duplicate).

A parameter value is substituted as text, so `this` or `prev` passed to a scripted trigger or effect (`can_be_warrior_trigger = { ARMY_OWNER = this }`, `00_compliment_custom_loc.txt:283`) means the scope at each use inside it, not the caller's. It works only while every use is at the top level of the scripted item, and breaks silently when a use moves into a nested scope. Tiger doesn't report this. (Its "target `this` makes no sense here" is a different check: a literal `this` as a target, i.e. comparing or targeting the current scope itself; after the substitution above it fires at `is_spouse_of = $ARMY_OWNER$`, 32 harmless self-comparisons.) Report `this` and `prev` as parameter values at the call site, with a message that explains the footgun, even where the scripted item uses them safely (decided in C2).

## F43. GUI `selectedindex` with an int32 getter

**Effort:** S. **Value:** L. **Vanilla:** 4 reports. **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

Tiger types `selectedindex` as CVector2i (`src/gui/properties.rs`). Vanilla binds int32 getters (`RulerDesignerLoadWindow.CurrentRulerLoadOption`, `DataModelView.GetActiveSortIndex`, `LobbyView.GetTransferSpeedSettingIndex`, ...) to it. Accept both, or find which widget types take which.

## F44. Scripted GUIs without a scope: `GuiScope.End` and `AddScope` without `SetRoot`

**Effort:** S. **Value:** L. **Vanilla:** 8 reports. **Issue:** [amtep/tiger#339](https://github.com/amtep/tiger/issues/339).

`src/data/scripted_guis.rs` demands `GuiScope.SetRoot(...)` as the first call. Vanilla calls scripted GUIs that declare no `scope` with `GuiScope.AddScope( 'owner', ... ).End` (`tgp_data_bindings.txt:88`) or a bare `GuiScope.End` (`window_pam_christian_church.gui:52`). Require `SetRoot` only when the scripted GUI has a `scope`. The other way round, scripted GUIs without a `scope` get their root from the caller's `SetRoot`: `window_knights.gui` passes `GetPlayer.GetCulture.MakeScope` to `kp_culture_*` (`knight_permissions_sguis.txt`), and `coronation_macros.txt` passes a character to `coronation_trigger_gui`; tiger roots them in none (4 reports). Take the root from the callers, or accept any root when there is no `scope`. Also, `scope = innovation_type` is not a scope type (`tgp_scripted_guis.txt`: vanilla bug or a name tiger doesn't know).

## F45. Data binding macros applied to method calls

**Effort:** S. **Value:** L. **Vanilla:** 10 reports. **Issue:** [amtep/tiger#340](https://github.com/amtep/tiger/issues/340).

`data_binding/00_script_value_bindings.txt` defines the macro `GetScriptValueBreakdown(Target,ScriptValueName)`. Vanilla GUI calls `GuiScope.SetRoot( Activity.MakeScope ).GetScriptValueBreakdown( 'debate_outcome_value' )`, the native one-argument `TopScope.GetScriptValueBreakdown( Arg0 )` from the datatype dump, and tiger expands the macro there: "wrong number of arguments for macro". The same happens with `IsNomad(TargetCharacter)` (`mpo_macros.txt`) on `TitleItem.GetTitle.IsNomad` (Carnalitas, xxx_compatch). Expand macros only for a chain's first part (or when no promote/function of that name fits).

## F46. Iterator parameters evaluated in the iterated scope

**Effort:** M. **Value:** M. **Vanilla:** 1 report seen (`00_ep1_inspiration_effects_sean.txt:543`). **Issue:** [amtep/tiger#348](https://github.com/amtep/tiger/issues/348).

Tiger validates an iterator's own parameters after `open_scope` (`src/effect.rs:466`, `src/trigger.rs`), so in `title_province = { every_pool_character = { province = this } }` the `this` is the pool character. The game builds the list in the enclosing scope, so `this` there is the province. The same goes for `faith = ...`, `culture = ...` and the `*_filter` targets of the ruler iterators. Related open question: in the ruler iterators, the plain `culture`/`faith` filters (still accepted, the 1.20 docs name `culture_filter`/`faith_filter`) clash with `culture = { ... }` used as a link inside the iterator (`learning_scholarship_events.txt:2714`) and with `culture != liege.culture` (`00_societal_traditions.txt:421`); a game test settles which one the game reads (T25). The docs have said the same since at least 1.19: the filter list names `*_filter`, the usage example below it writes the plain `government_type` and `tier`.

Game test T25 (1.20.1.2): in `random_ruler = { limit = { is_ai = yes } culture = { save_scope_as = x } }`, `culture` is the link, so the plain names are not filters. Tiger should accept only the `*_filter` forms as filters in the ruler iterators, and treat plain `culture`/`faith` as ordinary links. That fixes the 3 vanilla reports (`learning_scholarship_events.txt:2714`, `00_societal_traditions.txt:421`).

## F47. Event loc doesn't see scopes passed in by the caller

**Effort:** M. **Value:** M. **Vanilla:** about 20 reports seen. **Issue:** [amtep/tiger#330](https://github.com/amtep/tiger/issues/330).

`bastard_interaction.0003` is fired by `legitimize_bastard_interaction_opinions_effect`, called only from the legitimize interaction's `on_accept`, which has `scope:secondary_recipient`; the event's loc uses `[secondary_recipient.GetFirstName]` and tiger reports "unknown datafunction secondary_recipient". Saved scopes travel into events fired with `trigger_event`, so the loc is fine. Check how tiger builds the loc context of event descs, and whether it uses the callers' named scopes (it does for strict-scope checks). Others of the same kind: `recipient`/`actor` in `interactions_l_english.yml`, `raid_events_l_english.yml`, `pam_decisions_l_english.yml`.

## F48. Duplicate `switch` cases and duplicate scripted-effect parameters

**Effort:** S. **Value:** L. **Issue:** [amtep/tiger#362](https://github.com/amtep/tiger/issues/362). **Vanilla:** none known.

Tiger reports neither a duplicate `switch` case nor a parameter passed twice to a scripted effect. The vanilla example noted earlier (`petition_type_outcome_effect`) was a misreading: it has one `flag:council_seat_family` case in every version. Add both checks (a duplicate case is an error; a repeated parameter is at least a warning, as the game takes one of them).

## F49. Loc shown by GUI windows has the window's datacontext

**Effort:** M. **Value:** M. **Vanilla:** 35 reports. **Issue:** [amtep/tiger#333](https://github.com/amtep/tiger/issues/333).

Loc keys that only GUI windows show use the window's datacontext: `RAISE_TIME_DAYS:0 "[BannersView.GetSelectedBannerTimeToRaise] days"` (`banners_view_l_english.yml`), `[CourtWindow.GetCharacter...]`, `[SchemeItem.GetProgressDescription]`, `[CourtPositionsWindow.NumHiredPositions]`, `[CombatWindow.GetPhaseTooltip]` and others. Tiger validates them with no context: "unknown datafunction BannersView", "GetCharacter cannot follow a CourtWindow promote". Either learn the context from the GUI file that shows the key, or don't report window promotes in loc that no script uses.

## F51. Scripted modifier parameters inside quoted values

**Effort:** S. **Value:** L. **Vanilla:** 1 report. **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

`court_position_boon_aptitude_modifier` (`07_ep3_scripted_modifiers.txt:136`) has `value = "scope:position_temp.aptitude($POS$_court_position)"`; tiger checks the literal `$POS$_court_position` as a court position. Skip items that contain a `$PARAM$` in the definition of a scripted item (they are checked at the call sites).

## F53. Title history liege entries that apply at a start date

**Effort:** M. **Value:** H. **Vanilla:** 483 of the 1,044 liege and suzerain reports. **Issue:** [amtep/tiger#326](https://github.com/amtep/tiger/issues/326).

Tiger reports `liege = X` (and `suzerain`) in `history/titles` when X has no living holder on the entry's date ("setting the liege will not have effect here"). Game test T21 (1.20.1.2): `c_angouleme`'s last liege entry is `806.1.1 = { liege = "d_aquitaine" }`, when d_aquitaine's holder was dead, yet at the 1066 start Foulques I is the Duke of Aquitaine's vassal. The game applies the title's latest liege entry at the start date, so the report is wrong whenever the entry is still the latest one at a start date where both titles are held (483 entries, `histeff.py` in the C2 scratchpad). Report only entries that are superseded before every start date at which they would take effect, or rephrase the report for the others.

## F54. `$key|q$` escaping and literal casts in loc

**Effort:** S to M. **Value:** M. **Vanilla:** 16 reports for `|q` (PAM scripture glosses), 5 for casts. **Issue:** [amtep/tiger#331](https://github.com/amtep/tiger/issues/331).

Game test T22 (1.20.1.2): in `[Glossary('$key|q$','gloss')]` the `|q` formatter escapes the substituted text, so an apostrophe in it works; without `|q` the game logs "Expected ','" and fails the call. A literal cast `['(int32)7']` shows 7. Tiger substitutes the text unescaped and so reports "Unexpected character" for `|q`, and rejects literal casts (`gui/window_ledger.gui`, `data_binding/tgp_data_bindings.txt`). Apply `|q` escaping when substituting, report the unescaped form instead, and accept `'(type)value'` casts. The other loc forms in the same Unop rule (`\'` and `""` in strings, a space before `(`, `'50%'|P`) are still untested.

## F55. `exists` guards inside a negated block

**Effort:** S. **Value:** M. **Vanilla:** 15 reports (`councillor_spouse_learning_2603_scope_save_effect`). **Issue:** [amtep/tiger#336](https://github.com/amtep/tiger/issues/336).

Tiger ignores `exists = scope:x` anywhere inside `NOT`/`NOR` (`trigger.rs`, `if !negated`). Game test T23 (1.20.1.2): `NOR = { AND = { exists = scope:zz_nope this = scope:zz_nope } }` passes with no error, while the unguarded `NOR = { this = scope:zz_nope }` logs "Undefined event target". So inside an `AND` (or any block evaluated as a conjunction), an earlier `exists` guards the later uses even when the block is negated. Honour such guards. The same goes for `OR = { NOT = { exists = scope:x } <use of scope:x> }` (`06_fp3_scripted_triggers.txt:444`, 1 report): the use is reached only when `scope:x` exists.

## F56. `filter` in any list iterator

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`defenders_of_highgod_decision`), plus 1 in `any_held_title`. **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

Tiger accepts `filter = { }` only in `any_in_de_facto_hierarchy` and `any_in_de_jure_hierarchy`. Game test T24 (1.20.1.2): `any_vassal = { filter = { always = no } count = all always = no }` is true, so `filter` restricts the counted items in `any_vassal` too. Accept it in every `any_` iterator (`// undocumented`; the docs mention it for none).

## F57. `max` in a `random` effect

**Effort:** S. **Value:** L. **Vanilla:** 2 reports (`07_dlc_ep3_scripted_effects.txt:2489`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

Tiger accepts `max` only in `ordered_` iterators, `random_list` and `duel`. Game test T26 (1.20.1.2): `random = { chance = 0 modifier = { add = 100 } max = 0 ... }` fired 0 times in 20 with no error, while the control `random = { chance = 100 }` fired. So `max` caps the chance in `random` (`// undocumented`; the docs mention only `chance` and `modifier`).

## F58. `target_titles` in the grant-titles interface

**Effort:** S. **Value:** L. **Vanilla:** 2 reports (`grant_title_holy_order_patronage_warning_effect` in `grant_titles_interaction` and `grant_governorship_interaction`). **Issue:** [amtep/tiger#349](https://github.com/amtep/tiger/issues/349).

Game test T28 (1.20.1.2): an interaction with `special_interaction = grant_titles_interaction` and `interface = grant_titles` gets `list target_titles` (the chosen titles) and `scope:landed_title`; a plain `target_type = title` interaction gets neither. Tiger should provide both in the interactions that use that interface (`// undocumented`), and nowhere else.

## F59. Root of a domicile type's `move_cost`

**Effort:** S. **Value:** L. **Vanilla:** 9 reports (estate `move_cost`, `00_domicile_types.txt`). **Issue:** [amtep/tiger#334](https://github.com/amtep/tiger/issues/334).

Tiger roots `move_cost` in the domicile (`domiciles.rs`, undocumented). Game test T29 (1.20.1.2): an estate with main building level 2 costs 40 gold to move, the base 20 plus the level-2 bonus from `domicile ?= { has_domicile_building_or_higher = estate_main_02 }`. So the root is the owner character. Root `move_cost` in the character (`// undocumented`).

## F60. Comparators in `recent_history`

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`war_on_actions.txt:1837`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

Tiger rejects `recent_history = { days <= 1 }` (the docs show `days = Y`). Game test T32 (1.20.1.2): `days <= 1` and `days = 1` matched the same two counties on the day they changed hands, with no error. Accept the comparators (`// undocumented`).

## F61. Scripted GUI `scope = innovation_type`

**Effort:** S. **Value:** L. **Vanilla:** 2 reports (`tgp_scripted_guis.txt`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

Tiger knows only `culture_innovation` as the scope type name (as do the docs and the binary strings). Game test T33 (1.20.1.2): `on_silk_road_innovation_gui` with `scope = innovation_type` works; the culture window shows the silk road icon on Fire Medicine for the Viet culture, the innovation of the Tibet and Central Asia sub-regions. Accept `innovation_type` as a scripted GUI scope (`// undocumented`).

## F63. Built-in text formats `e`, `l` and `v`

**Effort:** S. **Value:** L. **Vanilla:** about 5 of the 19 "text format X not defined in gui/" reports. **Issue:** [amtep/tiger#341](https://github.com/amtep/tiger/issues/341).

Tiger reports `format_override` names that no `textformatting` block in `gui/` defines. Game test T34 (1.20.1.2): in loc, `#e`, `#l` and `#v` work without an error, while `#W`, `#dark`, `#white`, `#game_concept`, `#value_red`, `#value_green` and `#negative_value_toast` give "Unknown formatting tag", like the control `#zz_nope`. So `e`, `l` and `v` are built into the engine; treat them as defined. The other names are real (vanilla, low); `value_yellow`, `color:gray` and `color:red` were not tested.

## F64. Lowercase `list { }` in `default.map`

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`map_data/default.map:311`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

`impassable_mountains = list { 12830 12846 12881 }` is the only lowercase `list` in `default.map`; tiger reports it as an unexpected block. Game test T35 (1.20.1.2): province 12881 is impassable, with no errors, so the lowercase form works. Accept `list` like `LIST`.

## F65. Several `history` blocks in `create_artifact`

**Effort:** S. **Value:** L. **Vanilla:** 0 reports expected. Game test T55. **Issue:** [amtep/tiger#376](https://github.com/amtep/tiger/issues/376).

`create_artifact` accepts several `history` blocks (`src/ck3/effect_validation.rs:544`, upstream). Game test T39 (1.20.1.2) showed that `set_owner` and `set_holy_site_owner` use only the last `history` block, so tiger now accepts one there. Game test T55 (1.20.1.2) shows the same for `create_artifact`: with `given` and then `purchased`, the chest's history has only "Purchased". Accept `history` once, as in `set_owner`.

## F66. Loc keys validated without their scope context

**Effort:** S to M. **Value:** M. **Vanilla:** 90 reports (`ThisLocation` 72, `SAISE_location` 18). **Issue:** [amtep/tiger#342](https://github.com/amtep/tiger/issues/342).

Some loc keys are only checked for existence, so their text is validated generically, in a non-strict context: an event option's `name = key` (`validate_event_option`, `src/ck3/events.rs`) and a custom loc's `localization_key` (`src/data/customloca.rs`). There tiger accepts an unknown first name only if it starts with a lowercase letter, so a saved scope whose name starts with an uppercase letter gets "unknown datafunction X" although the context sets it. Vanilla: `00_roaming_loc.txt` saves `ThisLocation` in `setup_scope`, and `no_holding_roaming` uses `[ThisLocation.GetTerrain...]`; `SAISE_location` is saved in `00_ep1_artifact_effects.txt`. RICE: `aachen.0032` saves `RICE_aachen_spa_goer` in `immediate`, and only the option's text is reported, not the title or desc; the same event with a lowercase name gives no report. Validate those keys with the event's or custom loc's scope context (`validate_localization_sc`). The Unop conf files the vanilla reports under "loc datafunction slips".

## F67. Prison types are any static modifier

**Effort:** S. **Value:** L. **Vanilla:** none. **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

Tiger accepts only `dungeon` and `house_arrest` as prison types (`PRISON_TYPES`, `src/ck3/tables/misc.rs`). The docs of `change_prison_type`, `is_in_prison_type` and `imprison` say the type is any static modifier (1.19 and 1.20). Travelers defines `travl_travel_prison_type` and `prisoner_of_war` in `common/modifiers` and gets "unknown prison type" (7 reports). Check `Item::PrisonType` against `Item::Modifier`.

## F68. Domicile type names from flavorization

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`domicile_camp`, `00_domicile_types.txt`; also in the 1.19 baseline). **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

Tiger requires the loc key `domicile_<type>` for each domicile type (`src/ck3/data/domiciles.rs`). Vanilla has no `domicile_camp`: the camp's name comes from the flavorization entry `domicile_adventurer` (`common/flavorization/01_domicile.txt`, `type = domicile`, `domicile_type = camp`), whose loc is `[CHARACTER.GetPrimaryTitle.GetNameNoTooltip]`. Accept a domicile type that a flavorization entry names, or require the loc key only when none does. Pyrates overrides `camp` and gets the same report.

## F69. `exists = root` guards in root type deduction

**Effort:** M. **Value:** L. **Vanilla:** 1 report (`accolade_to_mercenary_effect` in `00_accolade_interactions.txt`). **Issue:** [amtep/tiger#351](https://github.com/amtep/tiger/issues/351).

Scripted effects that use root only inside `if = { limit = { exists = root } ... }` get their root type deduced from the guarded branch, and callers whose root is none (interactions, global on_actions) get "expects root to be character but root seems to be none". Vanilla `accolade_to_mercenary_effect`; Carnalitas Prostitution's `cpr_spawn_prostitutes_effect`, called from `yearly_global_pulse`. Don't carry deductions about root out of a branch guarded by `exists = root` (compare F1, which is the same problem for other runtime checks).

## F70. Guards on a scope's links: `exists = scope:x.link` and `scope:x.link ?= { }`

**Effort:** S to M. **Value:** M. Game test T52. **Vanilla:** 33 reports (`pam_church_event_background_is_grand_*_trigger`, "expects scope:activity to be set"). **Issue:** [amtep/tiger#332](https://github.com/amtep/tiger/issues/332).

Tiger honours `exists = scope:x` in a `limit` as a guard for `scope:x`, but not `exists = scope:x.link` or `scope:x.link ?= { ... }`, which are false (not errors) when `scope:x` is unset. Test mod (1.20.1.2 tiger): a scripted trigger with `trigger_if = { limit = { exists = scope:zz_c } scope:zz_c = { ... } }` gives no report, while the same with `exists = scope:zz_b.capital_province` or `scope:zz_a.capital_province ?= { ... }` gives "expects scope:zz_b to be set". Vanilla `pam_church_event_background_is_grand_western_trigger` uses `scope:activity.activity_location ?= { ... }` before `scope:activity.activity_location.rite`. Vanilla `artifact_should_use_gfx_type_trigger` (`00_artifact_triggers.txt:501`) guards with `exists = scope:artifact.creator.culture`, which gives "`create_artifact_weapon_effect` expects scope:artifact to be set" for vanilla callers (seen with artifact_trade and travelers). Treat both forms as guards for the scope they start from. That `exists` on a chain doesn't log an error when the scope is unset is still to be confirmed in game; `?=` is the null-safe form. The Unop conf calls these "Intended".

Tested in game (T52): both forms are false with nothing logged when `scope:x` is unset; only `scope:x.link = { }` logs errors. Posted on #332.

## F71. Scope types deduced from scope names

**Effort:** S. **Value:** L. **Vanilla:** none. **Issue:** [amtep/tiger#344](https://github.com/amtep/tiger/issues/344).

`scope_type_from_name` (`src/context.rs`) assumes that some named scopes have a fixed type, e.g. `scope:councillor` is a character, and limits that to "names so obvious that it's extremely unlikely that anyone would use them for a different type". Active Courtiers saves council position flags as `scope:councillor` (`accou_courtier_values.txt`, `accou_courtier_effects.txt`), and gets "`flag:chancellor` produces flag but expected character" (about 30 reports). Decide whether to keep the name list (mods rename their scope) or use the names only for messages, not as deductions.

## F72. `create_legend_seed` without `properties`

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`ep3_laamp_decision_events.txt`, `conqueror_chronicle`). **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

Tiger requires `properties` in `create_legend_seed` (`src/ck3/effect_validation.rs`). The docs only say that each key must match a property of the chronicle; `conqueror_chronicle` declares no `properties`, and its one vanilla call has none (the other 63 calls have them). Require `properties` only when the chronicle declares some, and check the keys against the chronicle.

## F73. `provisions = max_provisions` as a "full" check

**Effort:** S. **Value:** L. **Vanilla:** 9 uses (`ep3_laamp_decision_events.txt`, `laamp_extra_contracts.txt`, `ep3_interactions_events.txt`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

`provisions` is `CompareValueWarnEq`, so `provisions = max_provisions` gets "`provisions =` means exactly equal to that amount, which is usually not what you want". Provisions can't exceed the maximum, so equality with `max_provisions` is the natural "is full" check. Don't warn when the right side is `max_provisions`.

## F74. `list_size` as a list guard

**Effort:** S. **Value:** L. **Vanilla:** none. Game test T46. **Issue:** [amtep/tiger#372](https://github.com/amtep/tiger/issues/372).

Pyrates guards `laamp_base_2011_desired_surveyable_holdings_per_tier_value` with `"list_size(travellable_counties_list)" <= 0` before iterating the list, and gets "expects list travellable_counties_list to exist". A game test settles whether `list_size` of a list that doesn't exist is 0 without an error; if so, tiger could treat `"list_size(x)" > 0` (and `<= 0` in the other branch) as a guard for list `x`.

Tested in game (T46): `list_size` of a list that was never built is 0, with no error.

## F75. Several `apply_modifier` in a game rule setting

**Effort:** S. **Value:** L. **Vanilla:** 2 reports (`hard_difficulty`, `very_hard_difficulty` in `00_game_rules.txt`). **Issue:** [amtep/tiger#343](https://github.com/amtep/tiger/issues/343).

`src/ck3/data/gamerules.rs` reads `apply_modifier` with `field_value`, so a second one gets "`apply_modifier` is redefined in a following line". Vanilla's difficulty settings apply one modifier to the AI and another to the player (`apply_modifier = ai:hard_difficulty` and `apply_modifier = player:hard_difficulty_player`), and both work. Accept several (`multi_field_value`). Game rule mods hit it a lot (Interactive Game Rules: 955 reports in xxx_compatch).

## F76. Branches that are always false once the parameters are filled in

**Effort:** M. **Value:** L. **Issue:** [amtep/tiger#280](https://github.com/amtep/tiger/issues/280) (we filed #363, closed as its duplicate). **Vanilla:** none seen.

Tiger validates a scripted effect's body for each call with the arguments filled in, including branches that can never run with those arguments. A common debug helper in the user's mods: `trigger_if = { limit = { $TYPE$ = flag:boolean } scope:$NAME$ = yes }`, called with `NAME = artifact TYPE = flag:artifact`, becomes `flag:artifact = flag:boolean` in the limit, which is always false, yet tiger reports "`yes` produces bool but expected artifact" (artifact_trade 2, Active Courtiers 9, Carnalitas Prostitution 1). Same with Travelers' `if = { limit = { $KILLER$ != this } death = { killer = $KILLER$ } }` called with `KILLER = this`: the limit becomes `this != this`, and tiger reports "target `this` makes no sense here" (7). When a `limit` is a comparison whose two sides are identical (`this != this`) or two different literal flags (`flag:a = flag:b`), skip the reports in that branch.

## F77. `exists = scope:x` guards for scopes the caller never sets

**Effort:** M. **Value:** M. **Vanilla:** about 11 reports. **Issue:** [amtep/tiger#346](https://github.com/amtep/tiger/issues/346).

A use of `scope:y` (or list `y`) inside `trigger_if = { limit = { exists = scope:x } ... }` or after `exists = scope:x` in an `AND` is unreachable for a caller that never sets `scope:x`, but tiger still reports "expects scope:y to be set". Vanilla: `court_chaplain_endorse_override_trigger` (`00_councillor_triggers.txt:200`, guard `scope:ruler`, from `00_realm_actions.txt`), `dominion_fight` (`tgp_japan_decision_events.txt:3065`, guard `scope:main_defender`, from `war_on_actions.txt`), the county corruption effects (`00_county_corruption_effects.txt:1134`, guard `scope:marshal_removal`, from war, disaster, ministry and maintenance callers). Tiger knows which named scopes each caller sets (strict scopes), so it could treat such a branch as dead for that caller. Generalises F32.

## F78. A named scope re-saved with another type in the callee

**Effort:** M. **Value:** L. **Issue:** [amtep/tiger#364](https://github.com/amtep/tiger/issues/364). **Vanilla:** 4 reports (`diarch_changed_own_contract_effect`).

`00_diarchy_scripted_effects.txt:264-277` tests the bool `scope:diarch` passed by the send option, then re-saves `scope:diarch` as a character before using it. Tiger deduces the callee expects a character and reports "expects scope:diarch to be character but scope:diarch seems to be bool" at the callers. Don't carry deductions about a named scope past a `save_scope_as` of the same name in the callee. Related to F1.

## F80. `remove_geographical_region` scope

**Effort:** S. **Value:** L. **Vanilla:** 1 report (`test_decision.txt:365`, debug only). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

`effects.log` gives `remove_geographical_region` "Supported Scopes: situation_sub_region", like `add_geographical_region`. Tiger has `Scopes::Situation` and `Scope(GeographicalRegion)` (`src/ck3/tables/effects.rs:791`, also on main). Make it match `add_geographical_region`.

## F81. `?=` in character history date entries

**Effort:** S. **Value:** L. **Vanilla:** 4 reports (`greek.txt:2885,3092,3325,7306`). **Issue:** [amtep/tiger#345](https://github.com/amtep/tiger/issues/345).

`history/_characters.info`: "Any other key in a date entry is read as an effect statement, as if it was inside 'effect'". Vanilla writes `domicile ?= { move_domicile = root.top_liege.capital_province }` directly in date entries. Tiger's loop (`src/ck3/data/characters.rs:709`, `iter_assignments_and_definitions_warn`) rejects `?=` with "expected `domicile =`, found `?=`". Accept `?=` for the effect keys.

## F82. Tutorial lesson steps are per lesson

**Effort:** S. **Value:** L. **Vanilla:** 15 reports. **Issue:** [amtep/tiger#337](https://github.com/amtep/tiger/issues/337).

`_tutorial_lesson.info` names lesson steps "arbitrary name" inside each lesson (its own example uses `lesson_step_1`, `lesson_step_2`), and transitions target "Name of the step". Vanilla copies the war chain into the `reactive_advice_*` lessons with the same step names (`00_tutorial_lessons_reactive.txt:3161` vs `00_tutorial_lessons_war.txt:202`). Tiger registers steps as global `Item::TutorialLessonStep` (`src/data/tutorials.rs:34`) and reports them as redefined. Scope step names to their lesson.

## F83. Trait portrait modifier entries belong to their group

**Effort:** S. **Value:** L. **Vanilla:** 18 reports. Game test T48. **Issue:** [amtep/tiger#335](https://github.com/amtep/tiger/issues/335).

`gfx/portraits/trait_portrait_modifiers/_trait_modifier.info` nests `entry = { }` blocks in a group ("The first that is valid will be applied"). Vanilla repeats `no_visuals_when_beautified` as the first entry of 17 groups and `mild` in two. Tiger adds entries as global items (`src/ck3/data/trait_portrait_modifiers.rs:22`) and reports them as redefined. Scope entry names to their group (and check `base = <entry>` within it).

Tested in game (T48): a blind, disfigured character shows neither trait as a saint, so `no_visuals_when_beautified` applies in both groups. Posted on #335.

## F84. Artifact visuals: a complementary trigger as the fallback

**Effort:** S. **Value:** L. **Vanilla:** 8 of 69 reports. Game test T49. **Issue:** [amtep/tiger#379](https://github.com/amtep/tiger/issues/379).

Tiger wants one asset and one icon without a trigger in each artifact visual ("needs at least one asset without a trigger"). The `.info` doesn't require that. Some vanilla visuals cover all cases with an entry whose trigger is `NOR = { <the other triggers> }` (the icons of `axe`, `mace`, `axe_court`, `mace_court` and `crown`, the assets of `longbow`, `composite` and `fp2_armillary_sphere`); the rest key every entry on building or unit gfx, and some miss cultures: e.g. `book` has no entry for Indian, Southeast Asian or Tibetan building gfx, and the game logs lost `book` and `animal_trophy` artifacts. Lower the check to advice, or recognise the complementary form.

Tested in game (T49): with no matching entry the game logs "Couldn't determine 'asset' visual type; all triggers returned false" and creates no artifact, so the check is right. Only the complementary `NOR` fallback (longbow, armillary sphere) needs recognising.

## F85. Context names in tooltips the engine builds

**Effort:** M. **Value:** L. **Vanilla:** 27 reports. **Issue:** [amtep/tiger#347](https://github.com/amtep/tiger/issues/347).

Engine-built tooltips pass named context objects that are not global promotes in the data types dump: `KILLER` (`effects_l_english.yml:2608`), `TARGET_INNOVATION` (`triggers/culture_triggers_l_english.yml:95`; both near related code in the binary), and likely `COURTIER`, `EMPLOYER`, `HOST`, `SUB_REGION`, `TAX_SLOT`, `PHASE`, `TARGET`. Tiger reports "unknown datafunction X". `LandedTitle` (`situation_effects_l_english.yml:2-8`) is unclear. Accept these names in the loc keys that the engine uses for those tooltips (like F28 for `$PARAM$` placeholders).

## F86. Accessory variation `pattern_layout` with plain numbers

**Effort:** S. **Value:** L. **Vanilla:** 8 reports (`ccp7_emishi.txt`, `ccp7_nivkh.txt`). **Issue:** [amtep/tiger#338](https://github.com/amtep/tiger/issues/338).

`gfx/portraits/accessory_variations/example.txt:41-55` shows `scale = 0.1`, `rotation = 0.0` and `offset = { x = 0 y = 0 }` with plain numbers. Tiger (`src/data/accessory.rs:157-166`) requires `{ min max }` blocks and reports "expected block, found value". Accept both forms.

## F88. Graphical unit types: the `{key}` message and `western`

**Effort:** S. **Value:** L. **Vanilla:** 1 report. **Issue:** [amtep/tiger#350](https://github.com/amtep/tiger/issues/350).

`src/ck3/data/graphical_unit_types.rs:42` prints a literal `{key}` (the message isn't formatted). Also, `western` has no knight or levy illustrations of its own, but the `_default.dds` ones in `gfx/interface/illustrations/graphical_unit_types/*/` are evidently the western art; check whether `western` falls back to them and, if so, skip it like `default`.

## F89. Checks that the unchanged `.info` files document but tiger lacks

**Effort:** S each. **Value:** M for a to e, L for the rest. **Vanilla:** 0 new reports unless given. **Issue:** [amtep/tiger#366](https://github.com/amtep/tiger/issues/366).

From an audit of the 41 `.info` files that 1.20 didn't change. Each gap predates 1.20, so a fix gets a row in [changed-checks.md](changed-checks.md). Code is in `src/ck3/data/` unless given in full.

- a. Dynamic CoA definitions: "Name must match a landed title definition" (`_dynamic_definitions.info:1`). `src/data/coa.rs:440` never checks the key; add `Item::Title`. All 122 vanilla keys are titles.
- b. Event background and transition `ambience` refers to `sound/GUIDs.txt` (`_event_backgrounds.info:30-34`), but `event_themes.rs:129` and `:183` take any value. Use `Item::Sound`, as activity backgrounds do (`activities.rs:438`). Adds 11 vanilla reports, all `event:/SFX/Events/Backgrounds/FP4/fp4_magic_spring` (`01_event_backgrounds.txt:4178` and others), which is not in `GUIDs.txt`: a vanilla bug.
- c. Situation group types: loc `situation_group_type_<key>` (`_situation_group_types.info:1`) is not checked (`situations.rs:233`); message group types check theirs (`messages.rs:98`).
- d. Suggestions: `<key>_label` is listed with `_desc` and `_click` (`_suggestions.info:45-47`) and is the HUD button text (`hud_notification_templates.gui:2496`), but `suggestions.rs:31` only marks it used (`TODO: when is _label needed?`). Require it. Adds 1 vanilla report (`suggestion_sanctify_artifact`, which has no loc at all).
- e. Mercenary names: `coat_of_arms` (`_name_lists.info:34-36`) is `field_value` (`namelists.rs:79`); use `Item::Coa`. All vanilla values are in `01_mercenary_coas.txt`.
- f. Innovations: `skill` "Can be one of: learning, martial, stewardship, diplomacy, intrigue" (`_culture_innovations.info:23-30`), but `innovations.rs:55` takes any skill, including `prowess`.
- g. Innovation assets: `name` is a "base loc key" with `_desc` appended (`_culture_innovations.info:11`, `:63`); suggest `<name>_desc` as for the innovation itself (`innovations.rs:47`). All 9 vanilla asset names have it.
- h. Achievements: `happened` is the one trigger not marked optional (`_achievements.info:38`); add `req_field` (`src/data/achievements.rs:43`). All 198 vanilla achievements have it.
- i. Dynamic CoA definitions: require `item`, and `coat_of_arms` in each item (`_dynamic_definitions.info:3`, `:7`; `src/data/coa.rs:446`).
- j. Legend seeds: "Each property defined on the chronicle must be set here" (`_legend_seeds.info:39`), but the missing-property check in `legends.rs:231-266` runs only inside a `chronicle_properties` block, so a seed without one passes. The block form of a property (`:46-51`) doesn't require `target` (`legends.rs:246`).
- k. Holdings: `buildings` lists "First levels" and "doesn't include the primary building" (`_holdings.info:12-13`); `holdings.rs:66` checks existence only. Building already records `is_upgrade` (`buildings.rs:36-46`).
- l. Table styles: `default` "Only one can be set" (`_table_styles.info:35-37`), not enforced (`table_styles.rs:31`). The style key is also the settings dropdown's loc key (`settings_l_english.yml:115-118`, `settings_layout.txt:125`); check it, marked `// undocumented`.
- m. 2d event effects: the `trigger` "Receives the event scope" (`_event_2d_effects.info:14-16`), but `event_effect_2d.rs:29` only takes a block (`TODO: use validate_call`). Follow `EventBackground` in `event_themes.rs`.
- n. `divide_war_chest`: `effects.log:2886` says `faction`, vanilla (3 uses) and the binary say `fraction`, which tiger takes (`src/ck3/effect_validation.rs:815`). Add ``advice_field("faction", "docs say `faction` but it's `fraction`")``.
- o. Loc options: the "too many options" check (`src/data/localization.rs:621`) fires only on `$N$` without `$N|`, so `$N|U$` is never reported. Use `||`, as the "missing" check above it does.

## F90. DLC event decorations are optional

**Effort:** S. **Value:** M. **Vanilla:** 27 reports. **Issue:** [amtep/tiger#367](https://github.com/amtep/tiger/issues/367).

`src/data/dlc.rs:56` requires `gfx/interface/illustrations/dlc_event_decorations/<key>.dds` for every CK3 DLC, but `_dlc_metadata.info` names no files, and vanilla has only 3 of 30 (`dlc_006`, `dlc_009`, `dlc_015`), as in 1.13 to 1.19. The event windows test for it (`character_event.gui:1016`, `big_event_window.gui:972`: `GetContentSourceDlc.HasEventDecoration`). Drop that check; keep the `gfx/interface/icons/dlc` one (all 30 exist).

## F91. Report texts with mistakes

**Effort:** S. **Value:** L. **Issue:** [amtep/tiger#368](https://github.com/amtep/tiger/issues/368).

- `src/parse/pdxfile/parser.lalrpop:313` prints a literal "reader macro `{name}` is not defined" (a plain `&str`, not `format!`). Same kind as F88's `{key}`.
- `src/ck3/data/innovations.rs:68`: "asset must have `name` or `icon" lacks a closing backtick.

## F92. Reader export: `@@name` keys and the 1.13 "not supported" errors

**Effort:** M. **Value:** L. Needs a game test for b. **Issue:** [amtep/tiger#369](https://github.com/amtep/tiger/issues/369).

- a. `_reader_export.info:24`: "To use as the left hand side do @@<name>". Tiger's lexer reads `@@x` as `@` plus `@x` and the grammar has no `@@` key (`parser.lalrpop:198-219`), so `{ @@x = 3 }` gives "found loose value @", "reader variable  not defined" and "already defined as a reader variable". No vanilla use.
- b. `@:register_variable`, `@:load_variable` (`lexer.rs:309-326`) and inserting macros without `$` parameters (`parser.lalrpop:247`, `285`, `320`) are errors "as of CK3 1.13". The `.info` documents all of them (`:12`, `:20`, `:51`), unchanged since 1.13, so that is no proof; the 1.20 binary has `register_variable` and `load_variable` next to "Unknown reader action". Test: in `common/script_values/`, `@:register_variable t_a = 5`, `t_sv1 = @t_a`, `t_sv2 = @:load_variable t_a`, `@:define t_blk = { value = 3 }`, `t_sv3 = { @:insert t_blk }`; then `effect add_gold = t_sv1` (and 2, 3) and check `error.log`.
- c. Content in `reader_export` files other than reader actions is parsed and dropped silently (`src/parse/pdxfile.rs:97-100`); the `.info` says "it does not read anything itself". Warn.

## F93. Open questions from the audit of unchanged `.info` files

**Effort:** S each. **Value:** L. Game tests not yet written.

- a. Accolade icons: `texture = name_of_texture`, "only the filename" (`_accolade_icon.info:10-11`). Tiger wants the name with `.dds` (`accolades.rs:38-41`); vanilla defines none. Test both forms.
- b. Combat effects: `advantage = int` (`_combat_effects.info:4`), tiger takes any number (`combat_effects.rs:37`). Vanilla uses integers only. Test `advantage = 2.5`; switch to `field_integer` only if the game rejects it.
- c. Suggestions: `soundeffect` (`suggestions.rs:50`) is in neither the `.info` nor vanilla since 1.13, and the binary hit is only a tail of other strings. Test it, then remove it or mark it `TODO: verify`.
- d. History: `_history.info:3-9` puts "basic key-value pairs that denote the beginning of time" before the dated entries in all history files, but only character and province history accept undated keys. Test an undated `discover_innovation` in `history/cultures/`.
- e. Situation and struggle catalysts: no loc in the docs, but vanilla has `<key>` and `<key>_desc` for 267 of 269 and 122 of 127, and the GUI shows `GetCatalystType.GetName`. Check them, marked `// undocumented`, if a catalyst without loc shows its raw key. About 7 vanilla reports, mostly for unused catalysts.

## F94. `desc` blocks in `first_valid` that hold only triggered descs

**Effort:** S. **Value:** L. **Vanilla:** 2 reports (`map_modes.txt:584`, `bp1_yearly_events_nick.txt:2076`). Game test T44. **Issue:** [amtep/tiger#373](https://github.com/amtep/tiger/issues/373).

`src/desc.rs` counts any `desc = { ... }` block in a `first_valid` as unconditional and warns "multiple unconditional desc in one first_valid" for the entries after it. In game, a block whose `triggered_desc`s all fail is skipped, and `first_valid` moves on to the fallback. Count a block as unconditional only if it contains an unconditional desc. The other 6 vanilla reports of this message follow a plain `desc = key` and are right.

## F95. `name` in the `attach` blocks of asset entities

**Effort:** S. **Value:** L. **Vanilla:** 6 reports. Game test T45. **Issue:** [amtep/tiger#374](https://github.com/amtep/tiger/issues/374).

Unit entities attach their horse as `attach = { name = mount root = "unit_horse_4_entity" }`, and their states address it as `propagate_state = { mount = ... }`. `src/data/assets.rs` checks every value in an `attach` block as an asset and reports "asset mount not defined". In game the horse shows and animates. Treat `name` as the attachment's name.

## F96. Loc of portrait animations with `barbershop = no`

**Effort:** S. **Value:** L. **Vanilla:** 5 reports (`grand_marshal`, `thinking_all`, `prayer_muslim`, `spiritual_fulfillment`, `menacing_wooden_sword` in `animations.txt`). Game test T58. **Issue:** [amtep/tiger#377](https://github.com/amtep/tiger/issues/377).

`src/data/portrait.rs` requires `<key>` loc for every portrait animation. The CK3 `_animations.info` says `barbershop = no` keeps an animation out of the barbershop, which is where the name is shown. In game test T58 (1.20.1.2), none of the five is listed there. The only other use found is a character tooltip paragraph behind `ShouldShowAnimationInfo`, probably a debug option. Require the loc only when `barbershop` isn't `no`, with `TODO: verify`. The code is shared, so check the other games' `barbershop` docs first.

## F97. Scalar `weight` in `random_traits_list`

**Effort:** S. **Value:** L. **Vanilla:** 8 reports (`05_ce1_character_templates.txt:15-22`, unchanged since 1.19). Game test T62. **Issue:** [amtep/tiger#378](https://github.com/amtep/tiger/issues/378).

`validate_random_traits_list` (`src/ck3/validate.rs`) accepts `weight` only as a block, and the docs (`weight ( mtth )`) show only `weight = { base = 1 }`. In game test T62 (1.20.1.2), a scalar `weight = 100` against three `weight = 1` gave tier 5 98 times in 100. The block control gave 96 and the no-weight control 24, so the scalar works. Accept a value as well as a block. `random_culture` and `random_faith` have the same mtth docs, but weren't tested. Unop PR #840 files the vanilla reports as tiger false positives.

## F98. `main_rite` that names a rite of another faith

**Effort:** S to M. **Value:** L. **Vanilla:** 0 reports expected. Game test T57. **Issue:** [amtep/tiger#381](https://github.com/amtep/tiger/issues/381).

In the first run of game test T57 (1.20.1.2), a test faith with `main_rite = baltic_pagan` got none of the rite's tenets or doctrines, and nothing was logged. That rite's `faith` field names the vanilla faith `baltic_pagan`. Faith history can assign a rite to another faith from a date on: vanilla `catholic` and `orthodox` get `roman_rite` and `byzantine_rite`, whose `faith` is `christian_faith`, in `history/faiths/00_christianity.txt`. Warn when a faith's `main_rite` names a rite whose `faith` is another faith, and the faith's history never lists it as `main_rite` or in `rites`.
