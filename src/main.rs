use rltk::RGB;
use specs::prelude::*;

mod components;
mod game_state;
mod level;
mod systems;
mod util;

use components::{LeftMover, PlayerChar, Position, Renderable, Viewshed};
use game_state::State;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    {
        // Register comps
        gs.ecs.register::<Position>();
        gs.ecs.register::<Renderable>();
        gs.ecs.register::<LeftMover>();
        gs.ecs.register::<PlayerChar>();
        gs.ecs.register::<Viewshed>();
    }

    // Create map:
    let level = level::Level::new();
    let (pl_x, pl_y) = level.rooms[0].get_center();

    // Create monsters:
    for room in level.rooms.iter().skip(1) {
        let (x, y) = room.get_center();
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('g'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .build();
    }

    // Insert map after creating monsters (to satisfy borrow checker)
    gs.ecs.insert(level);
    // Create player:
    gs.ecs
        .create_entity()
        .with(Position { x: pl_x, y: pl_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(PlayerChar {})
        .with(Viewshed::new())
        .build();

    rltk::main_loop(ctx, gs)
}
