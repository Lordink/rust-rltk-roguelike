use specs::prelude::*;

pub mod monster_ai_system;
pub mod visibility_system;
pub use monster_ai_system::*;
pub use visibility_system::*;

use crate::components::{LeftMover, Position};

// Quick example of a (useless) system that moves LeftMovers to the left
pub struct _LeftMoverSystem {}
impl<'a> System<'a> for _LeftMoverSystem {
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
