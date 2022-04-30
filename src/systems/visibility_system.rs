use crate::components::{Position, Viewshed};
use crate::level::Level;
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Level>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level, mut viewshed, pos) = data;

        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            // Fill viewshed of this entity with the tiles it can see
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*level)
                    .into_iter()
                    .filter(|&p| p.x >= 0 && p.x < level.width && p.y >= 0 && p.y < level.height)
                    .collect();
            // dbg!(viewshed.visible_tiles.len());
        }
    }
}
