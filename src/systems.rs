use specs::prelude::*;

use crate::components::LeftMover;
use crate::components::Position;

pub struct LeftMoverSystem {}
impl<'a> System<'a> for LeftMoverSystem {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}
