use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::components::PlayerChar;
use crate::components::Position;
use crate::components::Renderable;
use crate::components::Viewshed;
use crate::level::{draw_tiles, Level, TileType};
use crate::systems::VisibilitySystem;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        process_input(self, ctx);
        self.run_systems();

        ctx.cls();

        // Render map
        draw_tiles(&mut self.ecs, ctx);

        // Render entities
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, ren) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn process_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
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
        }
    }
}
