use crate::components::{GameplayName, MonsterChar, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        // ReadStorage<'a, Position>,
        ReadStorage<'a, MonsterChar>,
        ReadStorage<'a, GameplayName>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewsheds, monsters, gnames) = data;

        for (vs, _, gname) in (&viewsheds, &monsters, &gnames).join() {
            if vs.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} bullies you.", gname.name));
            }
        }
    }
}
