use rltk::{RGB, Rltk, Console};
use specs::prelude::*;

use crate::{components::{CombatStats, PlayerChar}, game_log::GameLog};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    draw_player_hp(ecs, ctx);
    draw_game_log(ecs, ctx);
}

fn draw_player_hp(ecs: &World, ctx: &mut Rltk) {
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<PlayerChar>();

    for (_, stats) in (&players, &combat_stats).join() {
        let health = format!("HP {}/{}", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), health);

        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }
}

fn draw_game_log(ecs: &World, ctx: &mut Rltk) {
    let log = ecs.fetch::<GameLog>();

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }
}