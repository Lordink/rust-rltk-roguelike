use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: u16,
    pub hp: u16,
    pub defense: i32,
    pub power: u16,
}

/// Indicator that the owning entity wants to attack a target
#[derive(Component, Debug, /*ConvertSaveload,*/ Clone)]
pub struct MeleeAttackIntent {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct IncomingDamage {
    /// i32, cause the damage may be negative (healing)
    pub amount: Vec<i32>,
}

impl IncomingDamage {
    pub fn new(store: &mut WriteStorage<IncomingDamage>, victim: Entity, amount: i32) {
        if let Some(inc_dmg) = store.get_mut(victim) {
            // Add our damage to the list of already-existing pieces of damage
            inc_dmg.amount.push(amount);
        } else {
            let dmg = IncomingDamage {
                amount: vec![amount],
            };
            store
                .insert(victim, dmg)
                .expect("Should be able to insert damage to damage storage");
        }
    }
}
