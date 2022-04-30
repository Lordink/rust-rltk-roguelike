use crate::util::rect::Rect;
use rltk::{to_cp437, Rltk, RGB};
use rust_roguelike::PLAYER_START_POS;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    x as usize + (y as usize * 80)
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_level_v1() -> Vec<TileType> {
    let mut level = vec![TileType::Floor; 80 * 50];
    let wall = TileType::Wall;
    let player_idx = {
        let (player_x, player_y) = PLAYER_START_POS;
        xy_idx(player_x, player_y)
    };

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
        if idx != player_idx {
            level[idx] = wall;
        }
    }

    level
}

/// Rooms and corridors
pub fn new_level_v2() -> Vec<TileType> {
    let mut level = vec![TileType::Wall; 80 * 50];

    // Make rooms...
    {
        let room1 = Rect::new(20, 15, 10, 15);
        let room2 = Rect::new(35, 15, 10, 15);

        apply_room_to_level(&room1, &mut level);
        apply_room_to_level(&room2, &mut level);
    }

    // Make tunnel between rooms
    mk_horiz_tunnel(&mut level, 25, 40, 23);

    level
}

fn apply_room_to_level(room: &Rect, level: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let tile_idx = xy_idx(x, y);
            // Just a small self-testing piece of code
            assert!(tile_idx < level.len());
            level[tile_idx] = TileType::Floor;
        }
    }
}

fn mk_horiz_tunnel(level: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            level[idx as usize] = TileType::Floor;
        }
    }
}

fn mk_vert_tunnel(level: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            level[idx as usize] = TileType::Floor;
        }
    }
}

pub fn draw_level(level: &[TileType], ctx: &mut Rltk) {
    let (mut x, mut y) = (0, 0);

    for tile in level.iter() {
        // Render a type depending on its type
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
