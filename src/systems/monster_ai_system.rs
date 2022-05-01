use crate::components::{MonsterChar, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, MonsterChar>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewsheds, positions, monsters) = data;

        for (vs, pos, _) in (&viewsheds, &positions, &monsters).join() {
            console::log("Monster considers its own existence");
        }
    }
}
