use crate::block::Block;
use crate::ck3::modif::ModifKinds;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::modif::validate_modifs;
use crate::token::Token;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct Road {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::Road, Road::add)
}

impl Road {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Road, key, block, Box::new(Self {}));
    }
}

impl DbKind for Road {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.field_integer_range("tier", 0..=5);
        vd.field_integer_range("spline_asset_id", 0..=1);
        vd.field_list_items("provinces", Item::Province);
    }
}

#[derive(Clone, Debug)]
pub struct RoadModifiers {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::RoadModifiers, RoadModifiers::add)
}

impl RoadModifiers {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::RoadModifiers, key, block, Box::new(Self {}));
    }
}

impl DbKind for RoadModifiers {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        for tier in 0..=5 {
            vd.field_validated_block(&format!("tier_{tier}"), |block, data| {
                let mut vd = Validator::new(block, data);
                for (field, kinds) in [
                    ("modifier", ModifKinds::Road),
                    ("province_modifier", ModifKinds::Province),
                    ("trade_route_modifier", ModifKinds::TradeRoute),
                    ("travel_modifier", ModifKinds::TravelPlan),
                    ("character_modifier", ModifKinds::Character),
                ] {
                    vd.field_validated_block(field, |block, data| {
                        let vd = Validator::new(block, data);
                        validate_modifs(block, data, kinds, vd);
                    });
                }
            });
        }
    }
}
