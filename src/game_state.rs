use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::components::PlayerChar;
use crate::components::Position;
use crate::components::Renderable;
use crate::level::draw_level;
use crate::level::TileType;
use crate::systems::LeftMoverSystem;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        process_input(self, ctx);
        self.run_systems();

        ctx.cls();

        // Render map
        let level = self.ecs.fetch::<Vec<TileType>>();
        draw_level(&level, ctx);

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
        let mut lw = LeftMoverSystem {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn process_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<PlayerChar>();

    for (_, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}
