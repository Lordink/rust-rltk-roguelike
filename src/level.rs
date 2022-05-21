use crate::util::rect::Rect;
use rltk::{to_cp437, Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::{Entity, World};
use std::cmp::{max, min};
use std::collections::HashSet;

// Constants
const MAP_WIDTH_PIX: usize = 80;
const MAP_HEIGHT_PIX: usize = 43;
const MAP_PIXELCOUNT: usize = MAP_WIDTH_PIX * MAP_HEIGHT_PIX;

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
    /// These tiles were seen by player at some point
    revealed_tile_indices: HashSet<usize>,
    /// These tile indices are CURRENTLY VISIBLE by the player
    fov_tile_indices: HashSet<usize>,
    /// Keeping track of tiles blocked by some entity (preventing movement)
    blocked_tile_indices: HashSet<usize>,
    pub tile_content: Vec<Vec<Entity>>,
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

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_valid_exit(x - 1, y) {
            exits.push((idx - 1, 1.0));
        }
        if self.is_valid_exit(x + 1, y) {
            exits.push((idx + 1, 1.0));
        }
        if self.is_valid_exit(x, y - 1) {
            exits.push((idx - w, 1.0));
        }
        if self.is_valid_exit(x, y + 1) {
            exits.push((idx + w, 1.0));
        }

        // Diagonals
        if self.is_valid_exit(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_valid_exit(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_valid_exit(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_valid_exit(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

//--------------END RLTK Trait implementations

impl Level {
    pub fn clear_tiles_content(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
    pub fn block_walls_only(&mut self) {
        self.blocked_tile_indices.clear();
        for (i, tile) in self.tiles.iter().enumerate() {
            if *tile == TileType::Wall {
                self.blocked_tile_indices.insert(i);
            }
        }
    }
    pub fn block_tile(&mut self, idx: usize) {
        self.blocked_tile_indices.insert(idx);
    }
    pub fn is_tile_blocked(&self, idx: usize) -> bool {
        self.blocked_tile_indices.contains(&idx)
    }
    pub fn is_valid_exit(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            false
        } else {
            let idx = self.xy_idx(x, y);
            !self.is_tile_blocked(idx as usize)
        }
    }
    pub fn clear_fov_tiles(&mut self) {
        self.fov_tile_indices.clear();
    }
    pub fn is_tile_revealed(&self, idx: usize) -> bool {
        self.revealed_tile_indices.contains(&idx)
    }
    pub fn is_tile_visible(&self, idx: usize) -> bool {
        self.fov_tile_indices.contains(&idx)
    }
    pub fn reveal_tile(&mut self, idx: usize) {
        self.revealed_tile_indices.insert(idx);
        // Also add to fov (visible) tiles when we reveal them
        self.fov_tile_indices.insert(idx);
    }
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
            if idx > 0 && idx < MAP_PIXELCOUNT {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAP_PIXELCOUNT {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    pub fn new() -> Self {
        let mut level = Level {
            tiles: vec![TileType::Wall; MAP_PIXELCOUNT],
            rooms: Vec::new(),
            width: MAP_WIDTH_PIX as i32,
            height: MAP_HEIGHT_PIX as i32,
            revealed_tile_indices: HashSet::new(),
            fov_tile_indices: HashSet::new(),
            blocked_tile_indices: HashSet::new(),
            tile_content: vec![Vec::new(); MAP_PIXELCOUNT],
        };
        const NUM_MAX_ROOMS: u8 = 30;
        const MIN_ROOM_SIZE: u8 = 6;
        const MAX_ROOM_SIZE: u8 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..NUM_MAX_ROOMS {
            let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE) as i32;
            let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE) as i32;
            let x = rng.roll_dice(1, MAP_WIDTH_PIX as i32 - w - 1) - 1;
            let y = rng.roll_dice(1, MAP_HEIGHT_PIX as i32 - h - 1) - 1;
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
        if level.is_tile_revealed(idx) {
            draw_tile(level.is_tile_visible(idx), tile, ctx, x, y);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn draw_tile(is_visible: bool, tile: &TileType, ctx: &mut Rltk, x: i32, y: i32) {
    let glyph;
    let mut fg;
    match tile {
        TileType::Floor => {
            glyph = to_cp437('.');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::Wall => {
            glyph = to_cp437('#');
            fg = RGB::from_f32(0.4, 0.4, 0.4);
        }
    }
    if !is_visible {
        const VIS_DARKEN: f32 = 0.3;
        fg.r -= VIS_DARKEN;
        fg.g -= VIS_DARKEN;
        fg.b -= VIS_DARKEN;
    }
    ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
}
