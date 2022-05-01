use rltk::{Point, RGB};
use specs::prelude::*;

mod components;
mod game_state;
mod level;
mod systems;
mod util;

use components::{
    GameplayName, LeftMover, MonsterChar, PlayerChar, Position, Renderable, TileBlocker, Viewshed,
};
use game_state::{GameStatus, State};

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        status: GameStatus::Running,
    };
    {
        // Register comps
        register_components(&mut gs);
    }

    // Create map:
    let level = level::Level::new();

    // Spawn monsters:
    {
        let mut rng = rltk::RandomNumberGenerator::new();
        for (i, room) in level.rooms.iter().skip(1).enumerate() {
            let (x, y) = room.get_center();
            let roll = rng.roll_dice(1, 2);
            let glyph = rltk::to_cp437(match roll {
                1 => 'o',
                _ => 'g',
            });
            let name = match roll {
                1 => "Orc",
                _ => "Goblin",
            }
            .to_string();
            gs.ecs
                .create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    glyph,
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                })
                .with(Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                    is_dirty: true,
                })
                .with(MonsterChar {})
                // Give a numbered gameplay name
                .with(GameplayName {
                    name: format!("{} #{}", &name, i + 1),
                })
                .with(TileBlocker {})
                .build();
        }
    }

    // Obtian player starting loc, write it down as a resource for monsters to use
    let (pl_x, pl_y) = level.rooms[0].get_center();
    gs.ecs.insert(Point::new(pl_x, pl_y));
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
        .with(GameplayName {
            name: "Player".to_string(),
        })
        .build();

    rltk::main_loop(ctx, gs)
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<PlayerChar>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<MonsterChar>();
    gs.ecs.register::<GameplayName>();
    gs.ecs.register::<TileBlocker>();
}
