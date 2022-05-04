use rltk::{console, Point};
use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::components::Viewshed;
use crate::components::{CombatStats, PlayerChar};
use crate::components::{GameplayName, Renderable};
use crate::components::{MeleeAttackIntent, Position};
use crate::level::{draw_tiles, Level};
use crate::systems::{DamageSystem, MapIndexingSystem, MonsterAISystem};
use crate::systems::{MeleeCombatSystem, VisibilitySystem};

/// Current status of the game, used in tick to accomodate the turn-based nature of the gameplay
#[derive(PartialEq, Copy, Clone)]
pub enum GameStatus {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub status: GameStatus,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        if self.status == GameStatus::Running {
            // Run the systems a single time and then pause until the next input
            self.run_systems();
            // Clean up dead after what happens above, e.g. combat
            destroy_dead_entities(&mut self.ecs);
            self.status = GameStatus::Paused;
        } else {
            self.status = process_input(self, ctx);
        }

        ctx.cls();

        // Render map
        draw_tiles(&mut self.ecs, ctx);

        // Render entities
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let level = self.ecs.fetch::<Level>();

        for (pos, ren) in (&positions, &renderables).join() {
            if level.is_tile_visible(level.xy_idx(pos.x, pos.y)) {
                ctx.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph);
            }
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut monster_ai = MonsterAISystem {};
        monster_ai.run_now(&self.ecs);

        let mut map_indexer = MapIndexingSystem {};
        map_indexer.run_now(&self.ecs);

        let mut melee_combat_syster = MeleeCombatSystem {};
        melee_combat_syster.run_now(&self.ecs);

        let mut dmg_system = DamageSystem {};
        dmg_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

fn process_input(gs: &mut State, ctx: &mut Rltk) -> GameStatus {
    match ctx.key {
        None => return GameStatus::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Q => move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::E => move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::C => move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Z => move_player(-1, 1, &mut gs.ecs),

            _ => return GameStatus::Paused,
        },
    }
    GameStatus::Running
}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<PlayerChar>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut melee_attackers = ecs.write_storage::<MeleeAttackIntent>();
    let level = ecs.fetch::<Level>();

    for (ent, _, pos, vs) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let target_idx = level.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // Before moving - let's see if we attack anything:
        for target in level.tile_content[target_idx].iter() {
            match combat_stats.get(*target) {
                None => {}
                Some(target_combat_stats) => {
                    // Attack the target
                    melee_attackers.insert(ent, MeleeAttackIntent { target: *target })
                        .expect("Should be able to insert melee attack intent to the player entity when moving to attack");
                    // Prevent from further movement
                    return;
                }
            }
        }

        if !level.is_tile_blocked(target_idx) {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            // Notify the viewshed that it's dirty
            vs.is_dirty = true;
            // Update the globally available player loc storage
            let mut player_pos_storage = ecs.write_resource::<Point>();
            player_pos_storage.x = pos.x;
            player_pos_storage.y = pos.y;
        }
    }
}

fn destroy_dead_entities(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let player_chars = ecs.read_storage::<PlayerChar>();
        let entities = ecs.entities();
        let names = ecs.read_storage::<GameplayName>();
        for (ent, stats, name) in (&entities, &combat_stats, &names).join() {
            if stats.hp < 1 {
                let is_player = player_chars.get(ent).is_some();
                if is_player {
                    // TODO: disable player controls if dead.
                    println!("You are dead. Not a big surprise!");
                } else {
                    dead.push(ent);
                    println!("{} dies.", name.name);
                }
            }
        }
    }

    ecs.delete_entities(&dead)
        .expect("Should be able to destroy dead bodies from the world");
}
