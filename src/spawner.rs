use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

use crate::components::{
    CombatStats, GameplayName, MonsterChar, PlayerChar, Position, Renderable, TileBlocker, Viewshed,
};

pub fn spawn_player(ecs: &mut World, player_pos: (i32, i32)) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_pos.0,
            y: player_pos.1,
        })
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build()
}

pub fn spawn_rand_monster(ecs: &mut World, pos: (i32, i32)) {
    let roll = {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        rng.roll_dice(1, 2)
    };
    match roll {
        1 => spawn_orc(ecs, pos),
        _ => spawn_goblin(ecs, pos),
    };
}

fn spawn_orc(ecs: &mut World, pos: (i32, i32)) {
    spawn_monster(ecs, pos, rltk::to_cp437('o'), "Orc");
}
fn spawn_goblin(ecs: &mut World, pos: (i32, i32)) {
    spawn_monster(ecs, pos, rltk::to_cp437('g'), "Goblin");
}

fn spawn_monster<S: ToString>(
    ecs: &mut World,
    pos: (i32, i32),
    glyph: rltk::FontCharType,
    name: S,
) {
    ecs.create_entity()
        .with(Position { x: pos.0, y: pos.1 })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed::new())
        .with(MonsterChar {})
        .with(GameplayName {
            name: name.to_string(),
        })
        .with(TileBlocker {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build();
}
