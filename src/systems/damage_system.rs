use crate::components::{CombatStats, GameplayName, IncomingDamage};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, IncomingDamage>,
        ReadStorage<'a, GameplayName>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut all_stats, mut damages, gnames) = data;

        for (mut stats, dmg, name) in (&mut all_stats, &damages, &gnames).join() {
            let dmg_amount = dmg.amount.iter().sum::<i32>();

            stats.hp -= dmg_amount;
            println!(
                "{} ({}/{}) received {} dmg",
                name.name, stats.hp, stats.max_hp, dmg_amount
            );
        }

        // Remove InflictedDamage from all entities
        damages.clear();
    }
}
