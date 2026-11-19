use crate::block::Block;
use crate::ck3::data::buildings::{validate_asset, validate_triggered_name};
use crate::ck3::validate::validate_cost;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validate::validate_possibly_named_color;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct District {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::District, District::add)
}

impl District {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::District, key, block, Box::new(Self {}));
    }
}

impl DbKind for District {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        fn sc_builder(key: &Token) -> ScopeContext {
            let mut sc = ScopeContext::new(Scopes::Province, key);
            sc.define_name("holder", Scopes::Character, key);
            sc.define_name("county", Scopes::LandedTitle, key);
            sc
        }

        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        // `DistrictType` has `HasFlavorDescription` and `HasGameplayDescription`
        let loca = format!("{key}_desc");
        data.mark_used(Item::Localization, &loca);
        let loca = format!("{key}_desc_flavor");
        data.mark_used(Item::Localization, &loca);

        vd.multi_field_validated_block("triggered_name", validate_triggered_name);

        vd.multi_field_validated_block("background", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.field_trigger_rooted("trigger", Tooltipped::No, Scopes::Province);
            vd.req_field("reference");
            vd.field_item("reference", Item::File);
        });

        vd.field_choice(
            "type",
            &[
                "regular",
                "special",
                "duchy_capital",
                "holding",
                "great_district",
                "trade_post",
                "entrepot",
                // documented in history/_provinces.info
                "resource",
            ],
        );
        vd.field_validated("color", validate_possibly_named_color);

        vd.req_field("primary_building");
        vd.field_item("primary_building", Item::Building);
        vd.field_bool("can_be_replaced");
        vd.field_list_items("buildings", Item::Building);
        // undocumented
        vd.field_item("unlocked_by", Item::District);

        vd.multi_field_validated_block("assets", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.multi_field_validated_block("asset", validate_asset);
        });
        vd.multi_field_validated_block("asset", validate_asset);

        vd.field_trigger_builder("is_enabled", Tooltipped::FailuresOnly, sc_builder);
        if block.field_value_is("type", "great_district") {
            vd.field_trigger_builder("can_rebuild", Tooltipped::Yes, sc_builder);
            vd.field_validated_block_rooted("rebuild_cost", Scopes::Character, validate_cost);
        } else {
            vd.ban_field("can_rebuild", || "great districts");
            vd.ban_field("rebuild_cost", || "great districts");
        }
        vd.field_trigger_builder(
            "can_construct_potential",
            if block.get_field_bool("show_disabled").unwrap_or(false) {
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
    }
}
