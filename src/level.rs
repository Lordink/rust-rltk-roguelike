use rltk::{to_cp437, Rltk, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    x as usize + (y as usize * 80)
}

pub fn new_level() -> Vec<TileType> {
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
                    RGB::from_f32(0.0, 1.0, 0.0),
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
