use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::desc::validate_desc;
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validate::validate_possibly_named_color;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct TitleType {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::TitleType, TitleType::add)
}

impl TitleType {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::TitleType, key, block, Box::new(Self {}));
    }
}

impl DbKind for TitleType {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.field_value("prefix");
        vd.field_bool("trade_post");
        vd.field_item("domicile_type", Item::DomicileType);
        vd.field_bool("landless");
        vd.field_bool("house_title");
        vd.field_validated_block("properties", |block, data| {
            let mut vd = Validator::new(block, data);
            validate_title_properties(&mut vd);
        });

        validate_personal_relation(&mut vd, key);
    }
}

/// The personal relation entry that both landed titles and title types can set.
pub fn validate_personal_relation(vd: &mut Validator, key: &Token) {
    let mut sc = ScopeContext::new(Scopes::Character, key);
    sc.define_name("target", Scopes::Character, key);
    sc.define_name("title", Scopes::LandedTitle, key);
    sc.define_name("liege", Scopes::Character, key);
    sc.define_name("vassal", Scopes::Character, key);
    vd.field_validated_sc("personal_relation_entry", &mut sc, validate_desc);
    vd.field_trigger_rooted("personal_relation_vassal", Tooltipped::No, Scopes::Character);
}

/// The title properties that both landed titles and title types can set.
pub fn validate_title_properties(vd: &mut Validator) {
    vd.field_validated("color", validate_possibly_named_color);
    vd.field_bool("figurehead");
    vd.field_bool("allow_domicile");
    vd.field_bool("definite_form");
    vd.field_bool("ruler_uses_title_name");
    vd.field_bool("can_be_named_after_dynasty");
    vd.field_bool("landless");
    vd.field_bool("require_landless");
    vd.field_bool("no_automatic_claims");
    vd.field_bool("always_follows_primary_heir");
    vd.field_bool("destroy_if_invalid_heir");
    vd.field_bool("destroy_on_succession");
    vd.field_bool("delete_on_destroy");
    vd.field_bool("delete_on_gain_same_tier");
    vd.field_bool("noble_family");
    vd.field_bool("nomad");
    vd.field_bool("can_use_nomadic_naming");
}
