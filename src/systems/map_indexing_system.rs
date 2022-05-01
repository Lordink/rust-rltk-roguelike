use crate::components::{Position, TileBlocker};
use crate::level::Level;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TileBlocker>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, positions, blockers) = data;

        level.block_walls_only();
        for (pos, _) in (&positions, &blockers).join() {
            let idx = level.xy_idx(pos.x, pos.y);
            level.block_tile(idx);
        }
    }
}
