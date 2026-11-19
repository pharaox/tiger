use crate::block::Block;
use crate::ck3::modif::ModifKinds;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::helpers::{TigerHashMap, TigerHashSet};
use crate::item::{Item, ItemLoader};
use crate::modif::validate_modifs;
use crate::report::{ErrorKey, err, untidy};
use crate::token::Token;
use crate::validate::validate_color;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct TradeGoodType {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::TradeGoodType, TradeGoodType::add)
}

impl TradeGoodType {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::TradeGoodType, key, block, Box::new(Self {}));
    }
}

impl DbKind for TradeGoodType {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        let loca = format!("{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        // The docs say it's an integer, but vanilla uses fractions
        vd.field_numeric("base_price");
        vd.field_validated_block("origin", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.req_tokens_integers_exactly(2);
        });
        // TODO: verify; the docs call it an "integer flag" and vanilla doesn't use it
        vd.field_item("category", Item::TradeGoodCategory);
        vd.field_bool("is_basic");
    }
}

#[derive(Clone, Debug)]
pub struct TradeGoodCategory {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::TradeGoodCategory, TradeGoodCategory::add)
}

impl TradeGoodCategory {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::TradeGoodCategory, key, block, Box::new(Self {}));
    }

    /// Return the trade good types listed in a category, in the documented and the vanilla form.
    fn trade_goods(block: &Block) -> impl Iterator<Item = &Token> {
        block
            .iter_values()
            .chain(block.get_field_blocks("trade_goods").into_iter().flat_map(Block::iter_values))
    }

    /// A trade good type can be in only one need-type category.
    pub fn validate_all(db: &Db, _data: &Everything) {
        let mut categories: Vec<_> = db
            .iter_key_block(Item::TradeGoodCategory)
            .filter(|(_, block)| block.get_field_bool("need").unwrap_or(false))
            .collect();
        categories.sort_by(|(a, _), (b, _)| a.loc.cmp(&b.loc));
        let mut seen: TigerHashMap<&str, &Token> = TigerHashMap::default();
        for (_, block) in categories {
            for good in Self::trade_goods(block) {
                if let Some(&other) = seen.get(good.as_str()) {
                    let msg = format!("trade good type {good} is in two need-type categories");
                    err(ErrorKey::Conflict)
                        .msg(msg)
                        .loc(good)
                        .loc_msg(other, "other category")
                        .push();
                } else {
                    seen.insert(good.as_str(), good);
                }
            }
        }
    }
}

impl DbKind for TradeGoodCategory {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        let loca = format!("{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_bool("need");
        vd.field_validated_block("color", validate_color);

        // The docs show the trade good types as a bare list,
        // but vanilla lists them in a `trade_goods` block.
        let mut seen: TigerHashSet<&Token> = TigerHashSet::default();
        for good in Self::trade_goods(block) {
            data.verify_exists(Item::TradeGoodType, good);
            if let Some(&other) = seen.get(good) {
                let msg = format!("duplicate trade good type {good}");
                untidy(ErrorKey::DuplicateItem)
                    .msg(msg)
                    .loc(good)
                    .loc_msg(other, "first one")
                    .push();
            } else {
                seen.insert(good);
            }
        }
        vd.values();
        // undocumented
        vd.field_block("trade_goods");

        vd.field_validated_block("per_level_interval", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Character, vd);
        });
        vd.multi_field_validated_block("milestone_interval", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.field_integer_range("every_level", 1..);
            validate_modifs(block, data, ModifKinds::Character, vd);
        });
        vd.field_validated_block("levels", |block, data| {
            let mut vd = Validator::new(block, data);
            vd.unknown_block_fields(|_, block| {
                let mut vd = Validator::new(block, data);
                vd.field_validated_block("character_modifier", |block, data| {
                    let vd = Validator::new(block, data);
                    validate_modifs(block, data, ModifKinds::Character, vd);
                });
            });
        });
    }
}
