use rltk::{RGB, Rltk, Point };
use specs::prelude::*;

use crate::{components::{CombatStats, PlayerChar, GameplayName, Position}, game_log::GameLog, level::Level};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    draw_player_hp(ecs, ctx);
    draw_game_log(ecs, ctx);
    draw_mouse(ecs, ctx);
    draw_tooltips(ecs, ctx);
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

fn draw_mouse(_ecs: &World, ctx: &mut Rltk) {
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let level = ecs.fetch::<Level>();
    let names = ecs.read_storage::<GameplayName>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    // Don't draw anything when mouse is out of bounds
    if mouse_pos.0 > level.width || mouse_pos.1 >= level.height {
        return;
    }

    let mut tooltip: Vec<String> = Vec::new();
    for (name, pos) in (&names, &positions).join() {
        let idx = level.xy_idx(pos.x, pos.y);
        if pos.x == mouse_pos.0 && pos.y == mouse_pos.1 && level.is_tile_visible(idx) {
            tooltip.push(name.name.to_string());
        }
    }

    if tooltip.is_empty() {
        return;
    }

    // Find biggest str width:
    let mut width: i32 = 0;
    for s in tooltip.iter() {
        if width < s.len() as i32 {
            width = s.len() as i32;
        }
    }
    width += 3;

    if mouse_pos.0 > 40 {
        let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
        let left_x = mouse_pos.0 - width;
        let mut y = mouse_pos.1;
        for s in tooltip.iter() {
            ctx.print_color(left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
            let padding = (width - s.len() as i32) - 1;
            for i in 0..padding {
                ctx.print_color(arrow_pos.x - i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
            }

            y += 1;
        }
        ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"->".to_string());

    } else {
        let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
        let left_x = mouse_pos.0 + 3;
        let mut y = mouse_pos.1;
        for s in tooltip.iter() {
            ctx.print_color(left_x + 1, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
            let padding = (width - s.len() as i32) - 1;
            for i in 0..padding {
                ctx.print_color(arrow_pos.x + 1 + i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
            }

            y += 1;
        }
        ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"<-".to_string());

    }
}