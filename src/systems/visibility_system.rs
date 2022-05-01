use crate::components::{PlayerChar, Position, Viewshed};
use crate::level::Level;
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, PlayerChar>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, entities, mut viewsheds, positions, player_chars) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewsheds, &positions).join() {
            // Do nothing if this entity doesn't seem to require visibility recalc
            if !viewshed.is_dirty {
                continue;
            } else {
                // Clear the visibility recalc flag
                viewshed.is_dirty = false;
            }
            // dbg!("Recalculating vis...");
            // Fill viewshed of this entity with the tiles it can see
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*level)
                    .into_iter()
                    .filter(|&p| p.x >= 0 && p.x < level.width && p.y >= 0 && p.y < level.height)
                    .collect();

            // .get() on a Storage with entity will ret the component, if this entity HAS such component
            let is_player = player_chars.get(ent).is_some();

            if is_player {
                level.clear_fov_tiles();
                for &Point { x, y } in viewshed.visible_tiles.iter() {
                    let idx = level.xy_idx(x, y);
                    level.reveal_tile(idx);
                }
            }
        }
    }
}
