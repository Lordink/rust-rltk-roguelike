use crate::components::{CombatStats, GameplayName, IncomingDamage, MeleeAttackIntent};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MeleeAttackIntent>,
        ReadStorage<'a, GameplayName>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, IncomingDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ents, mut melee_attkrs, gnames, cstats, mut inc_dmg) = data;

        // Go thru each from the standpoint of the ATTACKER
        for (_, melee_attack, attacker_name, attacker_stats) in
            (&ents, &melee_attkrs, &gnames, &cstats).join()
        {
            // Ignore if attacker already ded
            if attacker_stats.hp <= 0 {
                continue;
            }

            let victim_ent = melee_attack.target;
            let victim_stats = cstats
                .get(victim_ent)
                .expect("Victim stats are obtainable in MeleeCombatSystem");

            // Ignore if victim already ded
            if victim_stats.hp <= 0 {
                continue;
            }

            let victim_name = gnames
                .get(victim_ent)
                .expect("Victim GameplayName is obtainable in MeleeCombatSystem");

            let dmg = i32::max(0, attacker_stats.power as i32 - victim_stats.defense);

            if dmg == 0 {
                println!(
                    "{} is unable to hurt {}",
                    &attacker_name.name, &victim_name.name
                );
            } else {
                IncomingDamage::new(&mut inc_dmg, victim_ent, dmg);
                println!(
                    "{} hits {} for {} dmg",
                    &attacker_name.name, &victim_name.name, dmg
                );
            }
        }

        // Clean up melee intents from ALL entities.
        melee_attkrs.clear();
    }
}
