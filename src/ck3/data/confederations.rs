use crate::block::Block;
use crate::ck3::modif::ModifKinds;
use crate::ck3::validate::validate_parameter_value;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::modif::validate_modifs;
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct ConfederationType {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::ConfederationType, ConfederationType::add)
}

impl ConfederationType {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::ConfederationType, key, block, Box::new(Self {}));
    }
}

impl DbKind for ConfederationType {
    fn add_subitems(&self, _key: &Token, block: &Block, db: &mut Db) {
        for block in block.get_field_blocks("cohesion_level") {
            if let Some(block) = block.get_field_block("parameters") {
                for (key, _) in block.iter_assignments() {
                    db.add_flag(Item::CohesionLevelParameter, key.clone());
                }
            }
        }
    }

    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let loca = format!("confederation_type_{key}_name");
        data.verify_exists_implied(Item::Localization, &loca, key);

        let mut vd = Validator::new(block, data);
        vd.field_choice("confederation_base", &["character", "house", "title"]);
        vd.replaced_field("house_based_confederation", "confederation_base");
        let base = block.get_field_value("confederation_base").map_or("character", Token::as_str);
        vd.field_integer("diplomatic_icon_frame_index");

        vd.field_script_value_builder("cohesion_base_change_monthly", |key| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            // TODO: figure out scope type
            sc.define_name("bloc", Scopes::all(), key);
            sc
        });
        vd.field_script_value_builder("cohesion_base_change_per_house_monthly", |key| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("house", Scopes::DynastyHouse, key);
            sc
        });
        // undocumented
        // TODO: verify that `title` is set, like `house` in the field above
        vd.field_script_value_builder("cohesion_base_change_per_title_monthly", |key| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("title", Scopes::LandedTitle, key);
            sc
        });
        vd.field_script_value_builder("cohesion_contribution", |key| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("base_value", Scopes::Value, key);
            sc.define_name("house", Scopes::DynastyHouse, key);
            sc.define_name("house_relation", Scopes::HouseRelation, key);
            sc.define_name("other_house", Scopes::DynastyHouse, key);
            sc.define_name("num_relations", Scopes::Value, key);
            sc
        });

        vd.field_script_value_rooted("cohesion_soft_cap", Scopes::Confederation);

        let mut levels = 0;
        vd.multi_field_validated_block("cohesion_level", |block, data| {
            levels += 1;
            let mut vd = Validator::new(block, data);
            vd.field_numeric("cohesion_threshold");
            vd.field_validated_block("parameters", |block, data| {
                let mut vd = Validator::new(block, data);
                vd.unknown_value_fields(|param, value| {
                    let loca = format!("{key}_level_parameter_{param}");
                    data.verify_exists_implied(Item::Localization, &loca, param);
                    validate_parameter_value(param, value);
                });
            });
            validate_confederation_modifiers(&mut vd, base);
        });

        if levels > 0 {
            for lvl in 0..=levels {
                let loca = format!("{key}_level_{lvl:02}");
                data.verify_exists_implied(Item::Localization, &loca, key);
            }
        }

        vd.field_trigger_rooted("is_valid_confederation", Tooltipped::No, Scopes::Confederation);
        vd.field_effect_rooted("on_confederation_destroyed", Tooltipped::No, Scopes::Confederation);

        vd.field_trigger_builder(
            "can_receive_modifiers_from_confederation",
            Tooltipped::No,
            |key| {
                let mut sc = ScopeContext::new(Scopes::Confederation, key);
                sc.define_name("character", Scopes::Character, key);
                sc.define_name("house", Scopes::DynastyHouse, key);
                sc
            },
        );
        vd.field_trigger_builder("member_would_join_war", Tooltipped::No, |key| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("enemy", Scopes::Character, key);
            sc.define_name("ally", Scopes::Character, key);
            sc
        });

        let sc_member_builder = |key: &Token| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("character", Scopes::Character, key);
            sc.define_name("house", Scopes::DynastyHouse, key);
            sc.define_name("confederation", Scopes::Confederation, key);
            sc
        };
        vd.field_trigger_builder("is_valid_member_character", Tooltipped::No, sc_member_builder);
        vd.field_effect_builder("on_member_character_joined", Tooltipped::No, sc_member_builder);
        vd.field_effect_builder("on_member_character_left", Tooltipped::No, sc_member_builder);

        let sc_house_builder = |key: &Token| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("house", Scopes::DynastyHouse, key);
            sc
        };
        vd.field_trigger_builder("is_valid_member_house", Tooltipped::No, sc_house_builder);
        vd.field_effect_builder("on_member_house_joined", Tooltipped::No, sc_house_builder);
        vd.field_effect_builder("on_member_house_left", Tooltipped::No, sc_house_builder);

        // undocumented
        let sc_title_builder = |key: &Token| {
            let mut sc = ScopeContext::new(Scopes::Confederation, key);
            sc.define_name("title", Scopes::LandedTitle, key);
            sc
        };
        vd.field_trigger_builder("is_valid_member_title", Tooltipped::No, sc_title_builder);
        vd.field_effect_builder("on_member_title_joined", Tooltipped::No, sc_title_builder);
        vd.field_effect_builder("on_member_title_left", Tooltipped::No, sc_title_builder);
    }
}

fn validate_confederation_modifiers(vd: &mut Validator, base: &str) {
    for field in &[
        "leading_house_head",
        "leading_house_member_not_head",
        "leading_house_member",
        "aligned_house_head",
        "aligned_house_member_not_head",
        "aligned_house_member",
        "divergent_house_head",
        "divergent_house_member_not_head",
        "divergent_house_member",
        "any_member_house_head",
        "any_member_house_member_not_head",
        "any_member_house_member",
    ] {
        if base == "house" {
            vd.field_validated_block(field, |block, data| {
                let vd = Validator::new(block, data);
                validate_modifs(block, data, ModifKinds::Character, vd);
            });
        } else {
            vd.ban_field(field, || "house based confederations");
        }
    }
    for field in &[
        "leading_title_holder",
        "leading_title_noble_family",
        "leading_title_landed_vassal",
        "member_title_holder",
        "member_title_noble_family",
        "member_title_landed_vassal",
    ] {
        if base == "title" {
            vd.field_validated_block(field, |block, data| {
                let vd = Validator::new(block, data);
                validate_modifs(block, data, ModifKinds::Character, vd);
            });
        } else {
            vd.ban_field(field, || "title based confederations");
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfederationAspiration {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::ConfederationAspiration, ConfederationAspiration::add)
}

impl ConfederationAspiration {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::ConfederationAspiration, key, block, Box::new(Self {}));
    }
}

impl DbKind for ConfederationAspiration {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.field_bool("show_in_main_hud");
        vd.field_bool("is_default");
        // undocumented
        // TODO: verify the scope type
        vd.field_trigger_rooted("is_shown", Tooltipped::No, Scopes::Confederation);

        vd.req_field("level");
        vd.multi_field_validated_block("level", |block, data| {
            let mut vd = Validator::new(block, data);
            for field in &["confederation_leader_modifier", "confederation_member_modifier"] {
                vd.field_validated_block(field, |block, data| {
                    let vd = Validator::new(block, data);
                    validate_modifs(block, data, ModifKinds::Character, vd);
                });
            }
            vd.field_script_value_no_breakdown_rooted("ai_score", Scopes::Character);
            vd.field_list("parameters");
            vd.field_bool("can_request_great_project_contributions_from_allies");
        });

        vd.field_item("illustration", Item::File);
        vd.field_validated_block("cooldown", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.field_integer("days");
            vd.field_integer("weeks");
            vd.field_integer("months");
            vd.field_integer("years");
        });
        vd.field_effect_builder("on_changed", Tooltipped::Yes, |key| {
            let mut sc = ScopeContext::new(Scopes::Character, key);
            sc.define_name("confederation", Scopes::Confederation, key);
            sc
        });
    }
}
