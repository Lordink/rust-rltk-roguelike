use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

use crate::{
    components::{
        CombatStats, GameplayName, Healer, Item, MonsterChar, PlayerChar, Position, Renderable,
        TileBlocker, Viewshed,
    },
    level::MAP_WIDTH_PIX,
    util::Rect,
};

const MAX_NUM_MONSTERS_PER_ROOM: i32 = 2;
const MAX_NUM_ITEMS_PER_ROOM: i32 = 2;

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

pub fn spawn_room_content(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_idx: Vec<usize> = Vec::new();
    let mut item_spawn_idx: Vec<usize> = Vec::new();

    // Keeping the borrow checker happy by scoping stuff
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_NUM_MONSTERS_PER_ROOM + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_NUM_ITEMS_PER_ROOM + 2) - 3;

        for _ in 0..num_monsters {
            let new_idx = find_new_monster_idx(room, &mut rng, &monster_spawn_idx);
            monster_spawn_idx.push(new_idx);
        }

        for _ in 0..num_items {
            let new_idx = find_new_item_idx(room, &mut rng, &item_spawn_idx);
            item_spawn_idx.push(new_idx);
        }
    }

    // Actually spawn the monsters
    for idx in monster_spawn_idx.iter() {
        let x = *idx % MAP_WIDTH_PIX;
        let y = *idx / MAP_WIDTH_PIX;
        spawn_rand_monster(ecs, (x as i32, y as i32));
    }
    // Actually spawn the items (pots)
    for idx in item_spawn_idx.iter() {
        let x = *idx % MAP_WIDTH_PIX;
        let y = *idx / MAP_WIDTH_PIX;
        spawn_health_potion(ecs, x as i32, y as i32);
    }
}

fn find_new_monster_idx(
    room: &Rect,
    rng: &mut specs::shred::FetchMut<RandomNumberGenerator>,
    monster_spawn_idx: &Vec<usize>,
) -> usize {
    loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;

        let idx = (y * MAP_WIDTH_PIX) + x;
        if !monster_spawn_idx.contains(&idx) {
            return idx;
        }
    }
}

fn find_new_item_idx(
    room: &Rect,
    rng: &mut specs::shred::FetchMut<RandomNumberGenerator>,
    item_spawn_idx: &Vec<usize>,
) -> usize {
    loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let idx = (y * MAP_WIDTH_PIX) + x;
        if !item_spawn_idx.contains(&idx) {
            return idx;
        }
    }
}

fn spawn_health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
        })
        .with(GameplayName {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Healer { heal_amount: 8 })
        .build();
}
