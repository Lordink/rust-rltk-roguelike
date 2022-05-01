use crate::{
    components::{PlayerChar, Viewshed},
    util::rect::Rect,
};
use rltk::{to_cp437, Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Level {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    /// Has the same len as the total level indices
    /// Each tile is either false (not revealed) or true
    /// TODO try replacing with a set
    pub are_tiles_revealed: Vec<bool>,
}

//--------------START RLTK Trait implementations

impl Algorithm2D for Level {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
impl BaseMap for Level {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

//--------------END RLTK Trait implementations

impl Level {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        x as usize + (y as usize * self.width as usize)
    }
    fn apply_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let tile_idx = self.xy_idx(x, y);
                // Just a small self-testing piece of code
                assert!(tile_idx < self.tiles.len());
                self.tiles[tile_idx] = TileType::Floor;
            }
        }
    }

    fn apply_horiz_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    pub fn new() -> Self {
        let mut level = Level {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            are_tiles_revealed: vec![false; 80 * 50],
        };
        const NUM_MAX_ROOMS: u8 = 30;
        const MIN_ROOM_SIZE: u8 = 6;
        const MAX_ROOM_SIZE: u8 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..NUM_MAX_ROOMS {
            let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE) as i32;
            let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE) as i32;
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);

            let no_intersections = level
                .rooms
                .iter()
                .find(|&room| new_room.intersects(room))
                .is_none();

            // TODO later: we could sometimes allow intersections
            // to create more interesting rooms.
            if !no_intersections {
                continue;
            }

            level.apply_room(&new_room);

            // Corridorize
            if !level.rooms.is_empty() {
                add_corridors(&new_room, &mut rng, &mut level);
            }

            level.rooms.push(new_room);
        }

        level
    }
}

fn add_corridors(new_room: &Rect, rng: &mut RandomNumberGenerator, level: &mut Level) {
    let (new_x, new_y) = new_room.get_center();
    let (prev_x, prev_y) = level.rooms[level.rooms.len() - 1].get_center();
    if rng.range(0, 2) == 1 {
        level.apply_horiz_tunnel(prev_x, new_x, prev_y);
        level.apply_vert_tunnel(prev_y, new_y, new_x);
    } else {
        level.apply_vert_tunnel(prev_y, new_y, prev_x);
        level.apply_horiz_tunnel(prev_x, new_x, new_y);
    }
}

pub fn draw_tiles(ecs: &World, ctx: &mut Rltk) {
    let level = ecs.fetch::<Level>();

    let (mut x, mut y) = (0, 0);
    for (idx, tile) in level.tiles.iter().enumerate() {
        // Render a type depending on its type
        if level.are_tiles_revealed[idx] {
            draw_tile(tile, ctx, x, y);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn draw_tile(tile: &TileType, ctx: &mut Rltk, x: i32, y: i32) {
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
}
