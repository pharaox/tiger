use crate::block::{BV, Block};
use crate::ck3::modif::ModifKinds;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::desc::validate_desc;
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::helpers::TigerHashMap;
use crate::item::{Item, ItemLoader};
use crate::modif::validate_modifs;
use crate::scopes::Scopes;
use crate::script_value::validate_non_dynamic_script_value;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validate::validate_modifiers_with_base;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct Building {
    is_upgrade: bool,
    is_duchy_capital: bool,
    is_great: bool,
}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::Building, Building::add)
}

impl Building {
    pub fn new() -> Self {
        Self { is_upgrade: false, is_duchy_capital: false, is_great: false }
    }

    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Building, key, block, Box::new(Self::new()));
    }

    pub fn finalize(db: &mut Db) {
        let mut next: TigerHashMap<String, String> = TigerHashMap::default();
        for (key, block) in db.iter_key_block(Item::Building) {
            if let Some(token) = block.get_field_value("next_building") {
                next.insert(key.to_string(), token.to_string());
            }
        }

        // Since 1.21, the type of the district a building line belongs to decides whether its
        // buildings are great buildings or duchy capital buildings.
        let mut properties = Vec::new();
        for (_, block) in db.iter_key_block(Item::District) {
            let property = if block.field_value_is("type", "great_district") {
                "is_great"
            } else if block.field_value_is("type", "duchy_capital") {
                "is_duchy_capital"
            } else {
                continue;
            };
            let mut lines: Vec<String> = block
                .get_field_values("primary_building")
                .iter()
                .map(ToString::to_string)
                .collect();
            if let Some(list) = block.get_field_list("buildings") {
                lines.extend(list.iter().map(ToString::to_string));
            }
            for mut building in lines {
                // Bounded, to guard against loops in the upgrade chain
                for _ in 0..=next.len() {
                    let upgrade = next.get(&building).cloned();
                    properties.push((building, property));
                    let Some(upgrade) = upgrade else { break };
                    building = upgrade;
                }
            }
        }

        for upgrade in next.into_values() {
            db.set_property(Item::Building, &upgrade, "is_upgrade");
        }
        for (building, property) in properties {
            db.set_property(Item::Building, &building, property);
        }
    }
}

impl DbKind for Building {
    fn add_subitems(&self, _key: &Token, block: &Block, db: &mut Db) {
        for token in block.get_field_values("flag") {
            db.add_flag(Item::BuildingFlag, token.clone());
        }
    }

    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        fn sc_builder(key: &Token) -> ScopeContext {
            let mut sc = ScopeContext::new(Scopes::Province, key);
            sc.define_name("holder", Scopes::Character, key);
            sc.define_name("county", Scopes::LandedTitle, key);
            sc
        }

        let mut vd = Validator::new(block, data);

        let graphical_only = block.get_field_bool("is_graphical_background").unwrap_or(false);
        if !graphical_only {
            data.verify_exists(Item::Localization, key);
            // The docs say the desc is `<key>_desc`, else `building_type_<key>_desc` of the
            // first building of the line, else blank.
            let loca = format!("{key}_desc");
            data.mark_used(Item::Localization, &loca);
            if !self.is_upgrade {
                let loca = format!("building_type_{key}");
                data.verify_exists_implied(Item::Localization, &loca, key);
                let loca = format!("building_type_{key}_desc");
                data.mark_used(Item::Localization, &loca);
            }
        }

        vd.multi_field_validated_block("triggered_name", validate_triggered_name);

        vd.field_icon("type_icon", "NGameIcons|BUILDING_TYPE_ICON_PATH", "");

        // TODO: int
        vd.field_validated("levy", validate_non_dynamic_script_value);
        // TODO: int
        vd.field_validated("max_garrison", validate_non_dynamic_script_value);
        // TODO: 0..1
        vd.field_validated("garrison_reinforcement_factor", validate_non_dynamic_script_value);
        // TODO: int
        vd.field_validated("construction_time", validate_non_dynamic_script_value);
        vd.field_script_value_builder("district_level_requirement", sc_builder);
        vd.advice_field("type", "removed in 1.21");

        vd.multi_field_validated_block("assets", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.multi_field_validated_block("asset", validate_asset);
        });
        vd.multi_field_validated_block("asset", validate_asset);

        vd.field_trigger_builder(
            "is_enabled",
            if graphical_only { Tooltipped::No } else { Tooltipped::FailuresOnly },
            sc_builder,
        );

        vd.advice_field("can_rebuild", "removed in 1.21");
        vd.advice_field("rebuild_cost", "removed in 1.21");

        // For buildings that are upgrades, can_construct_potential is added to can_construct_showing_failures_only so it will be tooltipped
        vd.field_trigger_builder(
            "can_construct_potential",
            if block.get_field_bool("show_disabled").unwrap_or(false) || self.is_upgrade {
                Tooltipped::FailuresOnly
            } else {
                Tooltipped::No
            },
            sc_builder,
        );

        vd.field_trigger_builder(
            "can_construct_showing_failures_only",
            Tooltipped::FailuresOnly,
            sc_builder,
        );

        vd.field_trigger_builder("can_construct", Tooltipped::Yes, sc_builder);
        vd.field_bool("show_disabled");

        vd.field_script_value_rooted("cost_gold", Scopes::Character);
        vd.field_script_value_rooted("cost_piety", Scopes::Character);
        vd.field_script_value_rooted("cost_prestige", Scopes::Character);
        // The docs say this is a fixed cost, and that great buildings don't support it
        if self.is_great {
            vd.ban_field("cost", || "buildings other than great buildings");
        } else {
            vd.field_validated_block("cost", validate_fixed_cost);
        }

        vd.field_item("next_building", Item::Building);
        vd.field_validated_rooted("effect_desc", Scopes::None, validate_desc);

        let is_duchy_capital = self.is_duchy_capital;
        validate_modifiers(&mut vd, is_duchy_capital);
        // TODO: fallback requires an `is_enabled` trigger to be configured as well
        vd.field_validated_block("fallback", |block, data| {
            let mut vd = Validator::new(block, data);
            validate_modifiers(&mut vd, is_duchy_capital);
        });
        vd.multi_field_validated_block("override_modifiers", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.field_item("requires_dlc_flag", Item::DlcFeature);
            validate_modifiers(&mut vd, is_duchy_capital);
        });
        vd.multi_field_validated_block("trade_good_need", |block, data| {
            validate_trade_good_need(block, data, is_duchy_capital);
        });

        vd.multi_field_value("flag");

        vd.field_validated_key("ai_value", |key, bv, data| match bv {
            BV::Value(token) => {
                token.expect_integer();
            }
            BV::Block(block) => {
                let mut sc = sc_builder(key);
                validate_modifiers_with_base(block, data, &mut sc);
            }
        });

        vd.field_bool("is_graphical_background");

        for field in ["on_start", "on_cancelled", "on_complete"] {
            vd.field_effect_builder(field, Tooltipped::No, |key| {
                let mut sc = ScopeContext::new(Scopes::Province, key);
                sc.define_name("character", Scopes::Character, key);
                sc.define_name("holding", Scopes::HoldingType, key);
                sc
            });
        }

        if self.is_great {
            vd.field_item("great_project_type", Item::GreatProjectType);
            vd.field_bool("is_mandala_capital");
        } else {
            vd.ban_field("great_project_type", || "great buildings");
            vd.ban_field("is_mandala_capital", || "great buildings");
        }
    }

    fn set_property(&mut self, _key: &Token, _block: &Block, property: &str) {
        match property {
            "is_upgrade" => self.is_upgrade = true,
            "is_duchy_capital" => self.is_duchy_capital = true,
            "is_great" => self.is_great = true,
            _ => (),
        }
    }
}

/// Validate a `triggered_name` block of a building or a district.
pub fn validate_triggered_name(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.field_trigger_rooted("trigger", Tooltipped::No, Scopes::Province);
    vd.req_field("reference");
    vd.field_item("reference", Item::Localization);
}

fn validate_trade_good_need(block: &Block, data: &Everything, is_duchy_capital: bool) {
    let mut vd = Validator::new(block, data);
    vd.req_field_one_of(&["trade_good", "trade_good_category"]);
    vd.field_item("trade_good", Item::TradeGoodType);
    vd.field_item("trade_good_category", Item::TradeGoodCategory);
    vd.field_validated("amount", validate_non_dynamic_script_value);
    vd.field_validated_block("modifiers", |block, data| {
        let mut vd = Validator::new(block, data);
        validate_modifiers(&mut vd, is_duchy_capital);
    });
    vd.field_bool("skip_internal_fulfillment");
    vd.advice_field(
        "skip_internal_fullfilment",
        "docs say `skip_internal_fullfilment` but it's `skip_internal_fulfillment`",
    );
    // undocumented
    vd.field_integer("can_be_filled_multiple_times");

    vd.field_item("culture_parameter", Item::CultureParameter);
    vd.field_item("not_culture_parameter", Item::CultureParameter);
    vd.field_item("rite_parameter", Item::DoctrineParameter);
    vd.field_item("not_rite_parameter", Item::DoctrineParameter);
    vd.field_item("government_flag", Item::GovernmentFlag);
    vd.field_item("situation_parameter", Item::SituationPhaseParameter);
    vd.field_item("dynasty_perk", Item::DynastyPerk);
    vd.field_integer("province_size");
    vd.field_item("terrain", Item::Terrain);
    vd.field_bool("is_coastal");
    vd.field_bool("is_riverside");
    vd.field_item("potential_trade_good", Item::TradeGoodType);
}

/// A cost made of numbers or named script values, not of dynamic script values
fn validate_fixed_cost(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    for field in [
        "gold",
        "prestige",
        "piety",
        "renown",
        "influence",
        "herd",
        "treasury",
        "treasury_or_gold",
        "merit",
        "barter_goods",
    ] {
        vd.field_validated_value(field, |_, mut vd| {
            if vd.value().get_number().is_some() {
                vd.numeric();
            } else {
                vd.item(Item::ScriptValue);
            }
        });
    }
}

pub fn validate_asset(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("type");
    vd.field_choice("type", &["pdxmesh", "entity"]);

    let meshes = block.field_value_is("type", "pdxmesh");
    let itype = if meshes { Item::Pdxmesh } else { Item::Entity };

    vd.multi_field_item("name", itype);
    vd.field_list_items("names", itype);

    vd.field_item("domicile_building", Item::DomicileBuilding);

    vd.field_item("illustration", Item::File);

    vd.multi_field_validated("soundeffect", |bv, data| {
        match bv {
            BV::Value(token) => data.verify_exists(Item::Sound, token),
            BV::Block(block) => {
                let mut vd = Validator::new(block, data);
                vd.req_field("soundeffect");
                vd.field_item("soundeffect", Item::Sound);
                vd.field_block("soundparameter"); // TODO
            }
        }
    });

    vd.field_list_items("graphical_cultures", Item::BuildingGfx);
    vd.field_list_items("graphical_faiths", Item::GraphicalFaith);
    vd.field_list_items("graphical_regions", Item::Region); // TODO check that it's a graphical region
    vd.field_list_items("provinces", Item::Province);
    vd.field_list_items("governments", Item::GovernmentType);

    vd.field_item("requires_dlc_flag", Item::DlcFeature);
}

/// A modifier block that applies when a parameter is present, or absent with `not_parameter`.
pub fn validate_param_modifier(block: &Block, data: &Everything, itype: Item, kinds: ModifKinds) {
    let mut vd = Validator::new(block, data);
    vd.req_field_one_of(&["parameter", "not_parameter"]);
    vd.field_item("parameter", itype);
    vd.field_item("not_parameter", itype);
    validate_modifs(block, data, kinds, vd);
}

/// A modifier block that applies to provinces that match all of its criteria.
fn validate_terrain_modifier(block: &Block, data: &Everything, kinds: ModifKinds) {
    let mut vd = Validator::new(block, data);
    vd.exclusive_fields(&["parameter", "not_parameter"]);
    vd.field_item("parameter", Item::CultureParameter);
    vd.field_item("not_parameter", Item::CultureParameter);
    vd.field_item("terrain", Item::Terrain);
    vd.field_bool("is_coastal");
    vd.field_bool("is_riverside");
    validate_modifs(block, data, kinds, vd);
}

fn validate_dynasty_modifier(block: &Block, data: &Everything, kinds: ModifKinds) {
    let mut vd = Validator::new(block, data);
    vd.req_field("county_holder_dynasty_perk");
    vd.field_item("county_holder_dynasty_perk", Item::DynastyPerk);
    validate_modifs(block, data, kinds, vd);
}

fn validate_potential_trade_good_modifier(block: &Block, data: &Everything, kinds: ModifKinds) {
    let mut vd = Validator::new(block, data);
    vd.req_field("trade_good");
    vd.field_item("trade_good", Item::TradeGoodType);
    validate_modifs(block, data, kinds, vd);
}

fn validate_plain_modifier(block: &Block, data: &Everything, kinds: ModifKinds) {
    let vd = Validator::new(block, data);
    validate_modifs(block, data, kinds, vd);
}

fn validate_modifiers(vd: &mut Validator, is_duchy_capital: bool) {
    let character = ModifKinds::Character;
    let province = ModifKinds::Province;
    let county = ModifKinds::County;
    let market = ModifKinds::TradeMarket;

    vd.multi_field_validated_block("character_modifier", |block, data| {
        validate_plain_modifier(block, data, character);
    });
    vd.multi_field_validated_block("character_culture_modifier", |block, data| {
        validate_param_modifier(block, data, Item::CultureParameter, character);
    });
    vd.multi_field_validated_block("character_rite_modifier", |block, data| {
        validate_param_modifier(block, data, Item::DoctrineParameter, character);
    });
    vd.multi_field_validated_block("character_dynasty_modifier", |block, data| {
        validate_dynasty_modifier(block, data, character);
    });
    vd.multi_field_validated_block("character_situation_modifier", |block, data| {
        validate_param_modifier(block, data, Item::SituationPhaseParameter, character);
    });
    vd.multi_field_validated_block("character_government_modifier", |block, data| {
        validate_param_modifier(block, data, Item::GovernmentFlag, character);
    });
    vd.multi_field_validated_block("character_potential_trade_good_modifier", |block, data| {
        validate_potential_trade_good_modifier(block, data, character);
    });
    vd.advice_field("character_faith_modifier", "removed in 1.20");

    vd.multi_field_validated_block("province_modifier", |block, data| {
        validate_plain_modifier(block, data, province);
    });
    vd.multi_field_validated_block("province_culture_modifier", |block, data| {
        validate_param_modifier(block, data, Item::CultureParameter, province);
    });
    vd.replaced_field("province_faith_modifier", "province_rite_modifier");
    vd.multi_field_validated_block("province_rite_modifier", |block, data| {
        validate_param_modifier(block, data, Item::DoctrineParameter, province);
    });
    vd.multi_field_validated_block("province_terrain_modifier", |block, data| {
        validate_terrain_modifier(block, data, province);
    });
    vd.multi_field_validated_block("province_dynasty_modifier", |block, data| {
        validate_dynasty_modifier(block, data, province);
    });
    vd.multi_field_validated_block("province_situation_modifier", |block, data| {
        validate_param_modifier(block, data, Item::SituationPhaseParameter, province);
    });
    vd.multi_field_validated_block("province_government_modifier", |block, data| {
        validate_param_modifier(block, data, Item::GovernmentFlag, province);
    });
    vd.multi_field_validated_block("province_potential_trade_good_modifier", |block, data| {
        validate_potential_trade_good_modifier(block, data, province);
    });

    vd.multi_field_validated_block("county_modifier", |block, data| {
        validate_plain_modifier(block, data, county);
    });
    vd.multi_field_validated_block("county_culture_modifier", |block, data| {
        validate_param_modifier(block, data, Item::CultureParameter, county);
    });
    vd.replaced_field("county_faith_modifier", "county_rite_modifier");
    vd.multi_field_validated_block("county_rite_modifier", |block, data| {
        validate_param_modifier(block, data, Item::DoctrineParameter, county);
    });
    vd.multi_field_validated_block("county_dynasty_modifier", |block, data| {
        validate_dynasty_modifier(block, data, county);
    });
    vd.multi_field_validated_block("county_situation_modifier", |block, data| {
        validate_param_modifier(block, data, Item::SituationPhaseParameter, county);
    });
    vd.multi_field_validated_block("county_potential_trade_good_modifier", |block, data| {
        validate_potential_trade_good_modifier(block, data, county);
    });

    if is_duchy_capital {
        vd.multi_field_validated_block("duchy_capital_county_modifier", |block, data| {
            validate_plain_modifier(block, data, county);
        });
        vd.multi_field_validated_block("duchy_capital_county_culture_modifier", |block, data| {
            validate_param_modifier(block, data, Item::CultureParameter, county);
        });
        vd.replaced_field(
            "duchy_capital_county_faith_modifier",
            "duchy_capital_county_rite_modifier",
        );
        vd.multi_field_validated_block("duchy_capital_county_rite_modifier", |block, data| {
            validate_param_modifier(block, data, Item::DoctrineParameter, county);
        });
        vd.multi_field_validated_block("duchy_capital_county_dynasty_modifier", |block, data| {
            validate_dynasty_modifier(block, data, county);
        });
        vd.multi_field_validated_block("duchy_capital_county_situation_modifier", |block, data| {
            validate_param_modifier(block, data, Item::SituationPhaseParameter, county);
        });
    } else {
        vd.ban_field("duchy_capital_county_modifier", || "duchy_capital buildings");
        vd.ban_field("duchy_capital_county_culture_modifier", || "duchy_capital buildings");
        vd.ban_field("duchy_capital_county_situation_modifier", || "duchy_capital buildings");
        vd.ban_field("duchy_capital_county_rite_modifier", || "duchy_capital buildings");
        vd.ban_field("duchy_capital_county_dynasty_modifier", || "duchy_capital buildings");
    }

    vd.multi_field_validated_block("market_modifier", |block, data| {
        validate_plain_modifier(block, data, market);
    });
    vd.multi_field_validated_block("market_culture_modifier", |block, data| {
        validate_param_modifier(block, data, Item::CultureParameter, market);
    });
    vd.advice_field(
        "market_faith_modifier",
        "docs say `market_faith_modifier` but it's `market_rite_modifier`",
    );
    vd.multi_field_validated_block("market_rite_modifier", |block, data| {
        validate_param_modifier(block, data, Item::DoctrineParameter, market);
    });
    vd.multi_field_validated_block("market_terrain_modifier", |block, data| {
        validate_terrain_modifier(block, data, market);
    });
    vd.multi_field_validated_block("market_size_modifier", |block, data| {
        let mut vd = Validator::new(block, data);
        vd.req_field("province_size");
        vd.field_integer("province_size");
        validate_modifs(block, data, market, vd);
    });
    vd.multi_field_validated_block("market_dynasty_modifier", |block, data| {
        validate_dynasty_modifier(block, data, market);
    });
    vd.multi_field_validated_block("market_situation_modifier", |block, data| {
        validate_param_modifier(block, data, Item::SituationPhaseParameter, market);
    });
    vd.multi_field_validated_block("market_government_modifier", |block, data| {
        validate_param_modifier(block, data, Item::GovernmentFlag, market);
    });

    vd.multi_field_validated_block("county_holding_modifier", |block, data| {
        let mut vd = Validator::new(block, data);
        vd.req_field("holding");
        vd.field_item("holding", Item::HoldingType);
        validate_modifs(block, data, county, vd);
    });
    vd.multi_field_validated_block("county_holder_character_modifier", |block, data| {
        validate_plain_modifier(block, data, character);
    });
}
