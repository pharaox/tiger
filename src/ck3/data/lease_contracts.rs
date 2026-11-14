use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::game::GameFlags;
use crate::item::{Item, ItemLoader};
use crate::report::{ErrorKey, err};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct LeaseContract {}

inventory::submit! {
    ItemLoader::Normal(GameFlags::Ck3, Item::LeaseContract, LeaseContract::add)
}

// TODO: verify somewhere that `theocracy_lease` is a defined item.

impl LeaseContract {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::LeaseContract, key, block, Box::new(Self {}));
    }
}

impl DbKind for LeaseContract {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        // Religions can pick the lease contract for theocratic leases, which then has a hierarchy
        let hierarchical = key.is("theocracy_lease")
            || data.database.iter_key_block(Item::Religion).any(|(_, block)| {
                block.get_field_block("religion_details").is_some_and(|block| {
                    block.field_value_is("theocracy_lease_contract_type", key.as_str())
                })
            });
        if hierarchical {
            vd.req_field("hierarchy");
            vd.field_validated_block("hierarchy", |block, data| {
                let mut vd = Validator::new(block, data);
                vd.field_choice("type", &["vassal", "clerical_region"]);
                vd.field_trigger_rooted("ruler_valid", Tooltipped::No, Scopes::Character);
                vd.field_trigger_builder("liege_or_vassal_valid", Tooltipped::No, |key| {
                    let mut sc = ScopeContext::new(Scopes::Character, key);
                    sc.define_name("target", Scopes::Character, key);
                    sc
                });
                vd.field_trigger_rooted("barony_valid", Tooltipped::No, Scopes::LandedTitle);
                let mut sc = ScopeContext::new(Scopes::Character, key);
                vd.field_target("lessee", &mut sc, Scopes::Character);
            });
        } else {
            vd.ban_field("hierarchy", || "theocratic lease contracts");
        }

        vd.field_item("government", Item::GovernmentType);
        vd.field_list_items("valid_holdings", Item::HoldingType);
        vd.field_integer("ruler_share_min_opinion_from_lessee");
        vd.field_choice("hook_strength_max_opinion", &["none", "any", "strong"]);

        vd.replaced_field("tax", "tax_split");
        vd.replaced_field("levy", "levy_split");
        for field in ["tax_split", "levy_split"] {
            vd.field_validated_block(field, |block, data| {
                // The docs say a share is from 0 to 1, and that shares that add up to more than 1
                // are an error. A share below 0 counts as 0, so shares that are not numbers add at
                // least 0.
                let mut sum = 0.0;
                for share in ["lease_liege", "top_lease_liege_direct", "ruler"] {
                    if let Some(token) = block.get_field_value(share)
                        && let Some(value) = token.get_number()
                    {
                        if !(0.0..=1.0).contains(&value) {
                            let msg = "should be between 0 and 1";
                            err(ErrorKey::Range).msg(msg).loc(token).push();
                        }
                        sum += value.max(0.0);
                    }
                }
                if sum > 1.0 {
                    let msg = "the shares add up to more than 1";
                    err(ErrorKey::Range).msg(msg).loc(block).push();
                }

                let mut vd = Validator::new(block, data);
                // The share roots are undocumented, but vanilla uses character triggers on them
                // The docs say the lease liege shares require a hierarchy definition.
                for share in ["lease_liege", "top_lease_liege_direct"] {
                    let max = format!("{share}_max");
                    if hierarchical {
                        vd.field_script_value_builder(share, |key| {
                            let mut sc = ScopeContext::new(Scopes::Character, key);
                            sc.define_name("lessee", Scopes::Character, key);
                            sc.define_name("lease_liege", Scopes::Character, key);
                            sc
                        });
                        vd.field_numeric_range(&max, 0.0..=1.0);
                    } else {
                        vd.ban_field(share, || "theocratic lease contracts");
                        vd.ban_field(&max, || "theocratic lease contracts");
                    }
                }
                vd.field_script_value_builder("ruler", |key| {
                    let mut sc = ScopeContext::new(Scopes::Character, key);
                    sc.define_name("lessee", Scopes::Character, key);
                    sc.define_name("ruler", Scopes::Character, key);
                    sc
                });
                vd.field_numeric_range("ruler_max", 0.0..=1.0);
            });
        }
    }
}
