use rltk::RGB;
use rust_roguelike::PLAYER_START_POS;
use specs::prelude::*;

mod components;
mod game_state;
mod level;
mod systems;

use components::LeftMover;
use components::PlayerChar;
use components::Position;
use components::Renderable;
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
    }

    // Insert map:
    gs.ecs.insert(level::new_level());

    // Create entities, starting with player:
    gs.ecs
        .create_entity()
        .with({
            let (pl_x, pl_y) = PLAYER_START_POS;
            Position { x: pl_x, y: pl_y }
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(PlayerChar {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('*'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }
    rltk::main_loop(ctx, gs)
}
