use crate::{
    components::{GameplayName, MonsterChar, Position, Viewshed},
    level::Level,
};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, MonsterChar>,
        ReadStorage<'a, GameplayName>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, player_pos, mut viewsheds, monsters, gnames, mut positions) = data;
        for (mut vs, _, gname, mut pos) in
            (&mut viewsheds, &monsters, &gnames, &mut positions).join()
        {
            if !vs.visible_tiles.contains(&*player_pos) {
                continue;
            }
            console::log(&format!("{} bullies you.", gname.name));

            // Pathfind and move the monster
            let path = rltk::a_star_search(
                level.xy_idx(pos.x, pos.y) as i32,
                level.xy_idx(player_pos.x, player_pos.y) as i32,
                &mut *level,
            );

            if path.success && path.steps.len() > 1 {
                pos.x = path.steps[1] as i32 % level.width;
                pos.y = path.steps[1] as i32 / level.width;
                vs.is_dirty = true;
            }
        }
    }
}
