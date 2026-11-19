use crate::block::{BV, Block};
use crate::ck3::modif::ModifKinds;
use crate::ck3::validate::validate_cost;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::modif::validate_modifs;
use crate::report::{ErrorKey, warn};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validate::validate_duration;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct DomicileType {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::DomicileType, DomicileType::add)
}

impl DomicileType {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::DomicileType, key, block, Box::new(Self {}));
    }
}

impl DbKind for DomicileType {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        let mut sc = ScopeContext::new(Scopes::Domicile, key);

        let loca = format!("domicile_{key}");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_trigger_rooted("allowed_for_character", Tooltipped::Yes, Scopes::Character);
        vd.field_list_items("initial_buildings", Item::DomicileBuilding);
        vd.field_choice("rename_window", &["primary_title", "house"]);
        // TODO: verify the scopes of the header and frame triggers
        vd.multi_field_validated("header", validate_triggered_texture);
        vd.multi_field_validated("frame", validate_triggered_texture);
        vd.multi_field_validated("illustration", validate_triggered_texture);
        vd.field_item("icon", Item::File);
        vd.field_item("map_pin_texture", Item::File);
        vd.field_choice("map_pin_anchor", &["up", "down", "left", "right"]);
        vd.field_bool("map_pin_lobby");
        vd.field_bool("provisions");
        vd.field_bool("travel");
        vd.field_bool("herd");
        vd.field_bool("culture_and_faith");
        vd.field_bool("move_with_realm_capital");
        vd.field_bool("can_move_manually");
        vd.field_bool("move_instantly");
        // undocumented
        vd.field_bool("acumen");
        vd.field_validated_block_sc("move_cooldown", &mut sc, validate_duration);
        vd.field_validated_block_sc("move_cost", &mut sc, validate_cost);
        vd.field_effect_rooted("on_move_start", Tooltipped::No, Scopes::Domicile);
        vd.field_effect_rooted("on_move_end", Tooltipped::No, Scopes::Domicile);
        vd.multi_field_validated_key_block(
            "domicile_temperament_low_modifier",
            |key, block, data| {
                let mut sc = ScopeContext::new(Scopes::Character, key);
                let mut vd = Validator::new(block, data);
                vd.field_item("name", Item::Localization);
                vd.field_script_value("scale", &mut sc);
                validate_modifs(block, data, ModifKinds::Character, vd);
            },
        );
        vd.multi_field_validated_key_block(
            "domicile_temperament_high_modifier",
            |key, block, data| {
                let mut sc = ScopeContext::new(Scopes::Character, key);
                let mut vd = Validator::new(block, data);
                vd.field_item("name", Item::Localization);
                vd.field_script_value("scale", &mut sc);
                validate_modifs(block, data, ModifKinds::Character, vd);
            },
        );
        vd.field_item("construction_soundeffect", Item::Sound);
        vd.field_integer("base_external_slots");
        vd.field_validated_block("domicile_building_slots", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.unknown_block_fields(|key, block| {
                validate_building_slot(key, block, data);
            });
        });
        // undocumented
        vd.field_list_items("artifact_slots", Item::ArtifactSlot);
        vd.multi_field_validated_block("domicile_asset", validate_domicile_asset);

        vd.multi_field_validated("map_entity", validate_map_entity);
    }
}

fn validate_building_slot(_key: &Token, block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.field_choice("slot_type", &["main", "external"]);
    vd.field_block("position"); // TODO
    vd.field_block("size"); // TODO
    vd.field_integer("depth");
    vd.multi_field_validated_block("empty_slot_asset", validate_slot_asset);
    vd.multi_field_validated_block("construction_slot_asset", validate_slot_asset);
}

fn validate_slot_asset(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.field_trigger_rooted("trigger", Tooltipped::No, Scopes::Domicile);
    vd.field_item("icon", Item::File);
    vd.field_item("texture", Item::File);
    vd.field_item("intersectionmask_texture", Item::File);
}

fn validate_domicile_asset(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.field_trigger_builder("trigger", Tooltipped::No, sc_domicile_owner);
    vd.field_item("background", Item::File);
    vd.field_item("foreground", Item::File);
    vd.multi_field_validated_block("layer", |block, data| {
        let mut vd = Validator::new(block, data);
        vd.field_item("texture", Item::File);
        vd.field_integer("depth");
    });
    vd.field_item("ambience", Item::Sound);
}

fn validate_triggered_texture(bv: &BV, data: &Everything) {
    match bv {
        BV::Value(value) => data.verify_exists(Item::File, value),
        BV::Block(block) => {
            let mut vd = Validator::new(block, data);
            vd.field_trigger_builder("trigger", Tooltipped::No, sc_domicile_owner);
            vd.req_field("reference");
            vd.field_item("reference", Item::File);
        }
    }
}

fn validate_map_entity(bv: &BV, data: &Everything) {
    match bv {
        BV::Value(value) => data.verify_exists(Item::Entity, value),
        BV::Block(block) => {
            let mut vd = Validator::new(block, data);
            vd.field_trigger_builder("trigger", Tooltipped::No, sc_domicile_owner);
            vd.field_item("reference", Item::Entity);
        }
    }
}

#[derive(Clone, Debug)]
pub struct DomicileBuilding {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::DomicileBuilding, DomicileBuilding::add)
}

impl DomicileBuilding {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::DomicileBuilding, key, block, Box::new(Self {}));
    }
}

impl DbKind for DomicileBuilding {
    fn add_subitems(&self, _key: &Token, block: &Block, db: &mut Db) {
        for parameters in block.get_field_blocks("parameters") {
            for (key, _) in parameters.iter_assignments() {
                db.add_flag(Item::DomicileParameter, key.clone());
            }
        }
    }

    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        // TODO: verify scope type
        fn sc_ai_value(key: &Token) -> ScopeContext {
            let mut sc = ScopeContext::new(Scopes::Domicile, key);
            sc.define_name("owner", Scopes::Character, key);
            sc
        }

        let mut vd = Validator::new(block, data);

        let loca = format!("{key}_domicile_building_desc");
        data.localization.suggest(&loca, key);

        vd.field_trigger_rooted("can_construct", Tooltipped::Yes, Scopes::Character);
        vd.field_trigger_rooted("can_construct_potential", Tooltipped::No, Scopes::Character);

        for field in &["on_start", "on_cancelled", "on_complete"] {
            vd.field_effect_builder(field, Tooltipped::No, sc_domicile_owner);
        }

        vd.field_integer("construction_time");

        vd.multi_field_validated_block("parameters", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.unknown_value_fields(|key, value| {
                let loca = format!("domicile_building_parameter_{key}");
                data.verify_exists_implied(Item::Localization, &loca, key);
                if !value.is("yes") {
                    let msg = "only `yes` currently makes sense here";
                    warn(ErrorKey::Validation).msg(msg).loc(value).push();
                }
            });
        });

        vd.field_choice("slot_type", &["main", "external", "internal"]);
        vd.field_integer("internal_slots");
        vd.field_list_items("allowed_domicile_types", Item::DomicileType);
        vd.field_item("previous_building", Item::DomicileBuilding);

        // TODO: verify scope type
        let mut sc = ScopeContext::new(Scopes::Character, key);
        vd.field_validated_block_sc("cost", &mut sc, validate_cost);
        vd.field_validated_block_sc("refund", &mut sc, validate_cost);

        vd.field_validated_block("character_modifier", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Character, vd);
        });

        vd.field_validated_block("province_modifier", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Province, vd);
        });

        vd.field_script_value_no_breakdown_builder("ai_value", sc_ai_value);

        vd.multi_field_validated_block("asset", validate_building_asset);

        vd.field_validated_block_sc("refund", &mut sc, validate_cost);
    }
}

fn validate_building_asset(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.field_trigger_rooted("trigger", Tooltipped::No, Scopes::Domicile);
    vd.field_item("icon", Item::File);
    vd.field_item("texture", Item::File);
    vd.field_item("intersectionmask_texture", Item::File);
    vd.field_item("soundeffect", Item::Sound);
}

fn sc_domicile_owner(key: &Token) -> ScopeContext {
    let mut sc = ScopeContext::new(Scopes::Domicile, key);
    sc.define_name("owner", Scopes::Character, key);
    sc
}
