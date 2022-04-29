use rltk::RGB;
use specs::prelude::*;

mod components;
mod game_state;
mod systems;

use components::LeftMover;
use components::PlayerChar;
use components::Position;
use components::Renderable;
use game_state::State;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    x as usize + (y as usize * 80)
}

fn new_level() -> Vec<TileType> {
    let mut level = vec![TileType::Floor; 80 * 50];
    let wall = TileType::Wall;

    // Boundary walls
    for x in 0..80 {
        level[xy_idx(x, 0)] = wall;
        level[xy_idx(x, 49)] = wall;
    }
    for y in 0..50 {
        level[xy_idx(0, y)] = wall;
        level[xy_idx(79, y)] = wall;
    }

    // Random bunch of walls:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            level[idx] = wall;
        }
    }

    level
}

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
    gs.ecs.insert(new_level());

    // Create entities, starting with player:
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
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
