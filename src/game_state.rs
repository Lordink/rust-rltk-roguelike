use rltk::Point;
use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::components::PlayerChar;
use crate::components::Position;
use crate::components::Renderable;
use crate::components::Viewshed;
use crate::level::{draw_tiles, Level, TileType};
use crate::systems::MonsterAISystem;
use crate::systems::VisibilitySystem;

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
            _ => return GameStatus::Paused,
        },
    }
    GameStatus::Running
}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<PlayerChar>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let level = ecs.fetch::<Level>();

    for (_, pos, vs) in (&mut players, &mut positions, &mut viewsheds).join() {
        let target_idx = level.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if level.tiles[target_idx] != TileType::Wall {
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
