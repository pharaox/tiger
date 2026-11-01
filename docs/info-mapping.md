# `.info` files and the tiger code that validates them

Every `.info` file in the CK3 game folder (203 in 1.20.1.1) and the tiger code that validates its folder. When an update changes an `.info` file, this says which validator to compare it with; when it adds one, add a row. Paths are under `src/ck3/data/` unless given in full. "not loaded" means tiger doesn't read the folder at all (see F6 in [follow-up.md](follow-up.md)).

| .info | Tiger code |
| --- | --- |
| _commandline_options.info | not loaded |
| common/accolade_icons/_accolade_icon.info | accolades.rs |
| common/accolade_names/_accolade_name.info | accolades.rs |
| common/accolade_types/_accolade_type.info | accolades.rs |
| common/achievements/_achievements.info | src/data/achievements.rs |
| common/activities/activity_group_types/_activity_group_types.info | activities.rs |
| common/activities/activity_locales/_activity_locales.info | activities.rs |
| common/activities/activity_types/_activity_type.info | activities.rs, src/ck3/tables/misc.rs |
| common/activities/guest_invite_rules/_invite_rules.info | activities.rs |
| common/activities/intents/_intents.info | activities.rs |
| common/activities/pulse_actions/_pulse_actions.info | activities.rs |
| common/ai_war_stances/_ai_war_stances.info | ai_war_stances.rs |
| common/artifacts/blueprints/_blueprints.info | artifacts.rs |
| common/artifacts/feature_groups/_feature_groups.info | artifacts.rs |
| common/artifacts/features/_features.info | artifacts.rs |
| common/artifacts/slots/_slots.info | artifacts.rs |
| common/artifacts/templates/_templates.info | artifacts.rs |
| common/artifacts/types/_types.info | artifacts.rs |
| common/artifacts/visuals/_visuals.info | artifacts.rs |
| common/bookmark_portraits/_bookmark_portraits.info | bookmarks.rs |
| common/bookmarks/bookmarks/_bookmarks.info | bookmarks.rs |
| common/bookmarks/challenge_characters/_challenge_characters.info | bookmarks.rs |
| common/bookmarks/groups/_bookmark_groups.info | bookmarks.rs |
| common/buildings/_buildings.info | buildings.rs |
| common/casus_belli_groups/_casus_belli_groups.info | casusbelli.rs |
| common/casus_belli_types/_casus_belli.info | casusbelli.rs |
| common/character_backgrounds/_character_backgrounds.info | pool.rs |
| common/character_interaction_categories/_character_interaction_categories.info | interaction_cats.rs |
| common/character_interactions/_character_interactions.info | interactions.rs |
| common/character_memory_types/_character_memories.info | memories.rs |
| common/coat_of_arms/dynamic_definitions/_dynamic_definitions.info | src/data/coa.rs |
| common/combat_effects/_combat_effects.info | combat_effects.rs |
| common/combat_phase_events/_combat_phase_events.info | combat.rs |
| common/confederation_types/_confederation_types.info | confederations.rs |
| common/connection_arrows/_connection_arrows.info | connection_arrows.rs |
| common/console_groups/_console_groups.info | not loaded |
| common/council_positions/_council_positions.info | council.rs |
| common/council_tasks/_council_tasks.info | council.rs |
| common/court_amenities/_court_amenities.info | amenities.rs |
| common/court_positions/tasks/_court_position_tasks.info | courtpos.rs |
| common/court_positions/types/_court_positions.info | courtpos.rs |
| common/court_types/_court_types.info | court_type.rs |
| common/courtier_guest_management/_courtier_guest_management.info | courtier_guest_mgmt.rs |
| common/culture/_cultural_traits.info | cultures.rs (validate_culture_trait) |
| common/culture/aesthetics_bundles/_aesthetics_bundles.info | cultures.rs |
| common/culture/creation_names/_creation_names.info | cultures.rs |
| common/culture/cultures/_cultures.info | cultures.rs |
| common/culture/eras/_culture_eras.info | cultures.rs, innovations.rs, src/ck3/validate.rs |
| common/culture/innovations/_culture_innovations.info | innovations.rs |
| common/culture/name_equivalency/_info.info | cultures.rs, src/everything.rs |
| common/culture/name_lists/_example.info | namelists.rs |
| common/culture/name_lists/_name_lists.info | namelists.rs |
| common/culture/pillars/_pillars.info | cultures.rs |
| common/culture/traditions/_traditions.info | cultures.rs |
| common/customizable_localization/_custom_loc.info | src/data/customloca.rs |
| common/deathreasons/_death_reasons.info | deathreasons.rs |
| common/decision_group_types/_decision_group_types.info | decisions.rs |
| common/decisions/_decisions.info | decisions.rs, src/ck3/validate.rs |
| common/diarchies/diarchy_mandates/_mandates.info | diarchies.rs |
| common/diarchies/diarchy_types/_diarchies.info | diarchies.rs |
| common/dna_data/_dna_data.info | src/data/dna.rs |
| common/domiciles/buildings/_domicile_buildings.info | domiciles.rs |
| common/domiciles/types/_domicile_types.info | domiciles.rs |
| common/dynasties/_dynasties.info | dynasties.rs |
| common/dynasty_house_motto_inserts/_inserts.info | mottos.rs |
| common/dynasty_house_mottos/_mottos.info | mottos.rs |
| common/dynasty_houses/_dynasty_houses.info | houses.rs |
| common/dynasty_legacies/_dynasty_legacies.info | dynasty_legacies.rs |
| common/dynasty_perks/_dynasty_perks.info | dynasty_legacies.rs |
| common/effect_localization/_effect_localization.info | src/data/effect_localization.rs |
| common/epidemics/_epidemics.info | epidemics.rs |
| common/event_2d_effects/_event_2d_effects.info | event_effect_2d.rs |
| common/event_backgrounds/_event_backgrounds.info | event_themes.rs |
| common/event_themes/_event_themes.info | event_themes.rs, src/ck3/validate.rs |
| common/event_transitions/_event_transitions.info | event_themes.rs |
| common/factions/_factions.info | factions.rs |
| common/flavorization/_flavourization.info | flavorization.rs |
| common/focuses/_focuses.info | focus.rs |
| common/game_concepts/_game_concepts.info | gameconcepts.rs |
| common/game_rules/_game_rules.info | gamerules.rs |
| common/genes/_genes.info | src/data/genes.rs |
| common/governments/_governments.info | government.rs, src/ck3/tables/misc.rs |
| common/graphical_unit_types/_graphical_unit_types.info | graphical_unit_types.rs |
| common/great_projects/types/_great_project_types.info | greatprojects.rs |
| common/guest_system/_guest_system.info | courtier_guest_mgmt.rs |
| common/holdings/_holdings.info | holdings.rs |
| common/holy_orders/_holy_orders.info | holyorders.rs |
| common/hook_types/_hooks.info | hooks.rs |
| common/house_aspirations/_house_aspiration.info | houses.rs |
| common/house_relation_types/_house_relation.info | houses.rs |
| common/house_unities/_house_unities.info | house_unities.rs |
| common/important_actions/_important_actions.info | important_actions.rs |
| common/inspirations/_inspirations.info | inspirations.rs |
| common/landed_titles/_landed_titles.info | titles.rs, src/ck3/tables/misc.rs |
| common/law_groups/_law_groups.info | laws.rs |
| common/laws/_laws.info | laws.rs |
| common/lease_contracts/_lease_contracts.info | lease_contracts.rs |
| common/legends/chronicles/_chronicles.info | legends.rs |
| common/legends/legend_seeds/_legend_seeds.info | legends.rs |
| common/legends/legend_types/_legends.info | legends.rs |
| common/legitimacy/_legitimacy.info | legitimacy.rs |
| common/lifestyle_perks/_lifestyle_perks.info | perks.rs |
| common/lifestyles/_lifestyles.info | lifestyles.rs |
| common/men_at_arms_types/_men_at_arms_types.info | maa.rs |
| common/menu_scenes/_default_menu_scenes.info | menu_scenes.rs |
| common/message_filter_types/_message_filter_types.info | messages.rs |
| common/message_group_types/_message_group_types.info | messages.rs |
| common/messages/_messages.info | messages.rs |
| common/modifier_definition_formats/_definitions.info | modif.rs |
| common/modifiers/_modifiers.info | modifiers.rs |
| common/morpheme_strip_rules/00_morpheme_strip_rules.info | morphemes.rs |
| common/nicknames/_nicknames.info | nickname.rs, src/ck3/tables |
| common/on_action/_on_actions.info | src/data/on_actions.rs |
| common/opinion_modifiers/_opinions.info | opinions.rs |
| common/playable_difficulty_infos/_playable_difficulty_infos.info | difficulty.rs |
| common/pool_character_selectors/_pool_character_selectors.info | pool.rs |
| common/province_terrain/_province_terrain.info | prov_terrain.rs |
| common/puppets/actions/_puppet_actions.info | puppets.rs |
| common/puppets/types/_puppet_types.info | puppets.rs |
| common/raids/intents/raid_intents.info | raids.rs |
| common/religion/doctrine_category_types/_doctrine_category_types.info | doctrines.rs |
| common/religion/doctrine_group_types/_doctrine_group_types.info | doctrines.rs |
| common/religion/doctrine_types/_doctrine_types.info | doctrines.rs |
| common/religion/faith_types/_faith_types.info | religions.rs |
| common/religion/great_holy_wars.info | prose about the system; its triggers and effects are in the tables |
| common/religion/holy_site_types/_holy_site_types.info | holysites.rs |
| common/religion/religion_family_types/_religion_family_types.info | religions.rs |
| common/religion/religion_types/_religion_types.info | religions.rs |
| common/religion/rite_icons/_rite_icons.info | rites.rs |
| common/religion/rite_names/_rite_names.info | rites.rs |
| common/religion/rite_types/_rite_types.info | rites.rs |
| common/religion/tenet_types/_tenet_types.info | tenets.rs |
| common/ruler_objective_advice_types/_ruler_objective_advice_types.info | ruler_objectives.rs |
| common/schemes/agent_types/_agent_types.info | schemes.rs |
| common/schemes/pulse_actions/_pulse_actions.info | schemes.rs |
| common/schemes/scheme_countermeasures/_scheme_countermeasures.info | schemes.rs |
| common/schemes/scheme_types/_schemes.info | schemes.rs |
| common/script_values/_script_values.info | src/script_value.rs |
| common/scripted_animations/_scripted_animations.info | scripted_animations.rs |
| common/scripted_character_templates/_scripted_character_templates.info | character_templates.rs |
| common/scripted_costs/_scripted_costs.info | scripted_costs.rs |
| common/scripted_modifiers/_scripted_modifiers.info | src/data/scripted_modifiers.rs |
| common/scripted_relations/_scripted_relations.info | relations.rs |
| common/secret_types/_secret_types.info | secrets.rs |
| common/situation/catalysts/_catalysts.info | situations.rs |
| common/situation/situation_group_types/_situation_group_types.info | situations.rs |
| common/situation/situations/_situations.info | situations.rs |
| common/spiritual_fulfillment/_spiritual_fulfillment_type.info | fulfillment.rs |
| common/story_cycles/_story_cycles.info | stories.rs |
| common/struggle/catalysts/_catalysts.info | struggle.rs |
| common/struggle/struggles/_struggles.info | struggle.rs |
| common/subject_contracts/contracts/_subject_contracts.info | subjectcontract.rs |
| common/subject_contracts/groups/_subject_contract_groups.info | subjectcontract.rs |
| common/succession_appointment/_succession_appointment.info | succession.rs |
| common/succession_election/_succession_election.info | election.rs |
| common/suggestions/_suggestions.info | suggestions.rs |
| common/task_contracts/_task_contracts.info | task_contracts.rs |
| common/tax_slots/obligations/_tax_obligation.info | tax_slots.rs |
| common/tax_slots/types/_tax_slot_type.info | tax_slots.rs |
| common/terrain_types/_terrains.info | terrain.rs, src/ck3/tables/modifs.rs |
| common/traits/_traits.info | traits.rs |
| common/travel/point_of_interest_types/_travel_point_of_interest_types.info | points_of_interest.rs |
| common/travel/travel_options/_travel_options.info | travel.rs |
| common/trigger_localization/_trigger_localization.info | src/data/trigger_localization.rs |
| common/tutorial_lesson_chains/_tutorial_lesson_chains.info | src/data/tutorials.rs |
| common/tutorial_lessons/_tutorial_lesson.info | src/data/tutorials.rs |
| common/vassal_stances/_vassal_stances.info | vassalstance.rs, src/ck3/tables/modifs.rs |
| dlc_metadata/_dlc_metadata.info | src/data/dlc.rs |
| events/_events.info | src/ck3/events.rs |
| gfx/coat_of_arms/color_palettes/_color_palettes.info | src/data/coadesigner.rs |
| gfx/coat_of_arms/colored_emblems/_colored_emblems.info | src/data/coadesigner.rs |
| gfx/coat_of_arms/emblem_layouts/_emblem_layouts.info | src/data/coadesigner.rs, src/data/coa.rs |
| gfx/coat_of_arms/patterns/_patterns.info | src/data/coadesigner.rs |
| gfx/court_scene/character_groups/_groups.info | court_scene.rs |
| gfx/court_scene/character_roles/_roles.info | court_scene.rs |
| gfx/court_scene/scene_cultures/_cultures.info | court_scene.rs |
| gfx/interface/illustrations/loading_screens/_loadscreen.info | not loaded |
| gfx/interface/illustrations/scripted_illustrations/_scripted_illustrations.info | scripted_illustrations.rs |
| gfx/map/environment/environment.info | map_environment.rs |
| gfx/map/flat_map_styles/_flat_map_styles.info | flatmap.rs |
| gfx/map/province_effects/_province_effects.info | prov_effects.rs |
| gfx/map/table_styles/_table_styles.info | table_styles.rs |
| gfx/models/units/_unit_states.info | not loaded |
| gfx/models/units/entity_links/_entity_links.info | not loaded |
| gfx/portraits/accessory_variations/_second_color_mask.info | src/data/accessory.rs |
| gfx/portraits/customization_rules/_customization_rules.info | not loaded |
| gfx/portraits/portrait_animations/_animations.info | src/data/portrait.rs |
| gfx/portraits/portrait_modifiers/_portrait_modifiers.info | src/data/portrait.rs |
| gfx/portraits/trait_portrait_modifiers/_trait_modifier.info | trait_portrait_modifiers.rs |
| gfx/skins/hud_skins/_hud_skins.info | not loaded |
| gui/scripted_widgets/_scripted_widgets.info | not loaded |
| gui/shared/animation.info | - |
| history/_characters.info | characters.rs |
| history/_history.info | the history loaders (general format) |
| history/_provinces.info | prov_history.rs |
| history/cultures/_culture.info | culture_history.rs |
| history/faiths/_faith_history.info | faith_history.rs |
| history/situations/_situations.info | situations.rs |
| history/struggles/_struggles.info | struggle.rs |
| map_data/geographical_regions/_geographical_regions.info | regions.rs |
| music/_music.info | src/data/music.rs |
| reader_export/_reader_export.info | src/everything.rs, src/parse/pdxfile/ |
| tests/_tests.info | not loaded |
