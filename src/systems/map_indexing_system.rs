use crate::components::{Position, TileBlocker};
use crate::level::Level;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TileBlocker>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, positions, blockers, entities) = data;

        level.block_walls_only();
        level.clear_tiles_content();
        for (ent, pos) in (&entities, &positions).join() {
            let idx = level.xy_idx(pos.x, pos.y);

            // Block this tile, if it has TileBlocker
            if blockers.get(ent).is_some() {
                level.block_tile(idx)
            }

            level.tile_content[idx].push(ent);
        }
    }
}
