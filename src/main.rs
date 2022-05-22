use rltk::{Point, RGB};
use specs::prelude::*;

mod components;
mod game_log;
mod game_state;
mod gui;
mod level;
mod spawner;
mod systems;
mod util;

use components::{
    CombatStats, GameplayName, IncomingDamage, LeftMover, MeleeAttackIntent, MonsterChar,
    PlayerChar, Position, Renderable, TileBlocker, Viewshed,
};
use game_state::{GameStatus, State};

use crate::game_log::GameLog;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
    eprintln!("Serving demo profile data on {}", server_addr);
    let puffin_server = puffin_http::Server::new(&server_addr).unwrap();

    puffin::set_scopes_on(true);

    let mut ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    ctx.with_post_scanlines(true);
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);

    // Insert rng generator for utility
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // Insert profiling server
    gs.ecs.insert(puffin_server);

    // Insert globally-available turn status
    gs.ecs.insert(GameStatus::PreTurn);

    // Insert game log
    gs.ecs.insert(GameLog {
        entries: vec!["Welcome and good luck!".to_string()],
    });

    // Create map:
    let level = level::Level::new();

    // Spawn monsters:
    for room in level.rooms.iter().skip(1) {
        spawner::spawn_room_content(&mut gs.ecs, room);
    }

    // Obtian player starting loc, write it down as a resource for monsters to use
    let pl_spawn_pos = level.rooms[0].get_center();
    gs.ecs.insert(Point::new(pl_spawn_pos.0, pl_spawn_pos.1));
    // Insert map after creating monsters (to satisfy borrow checker)
    gs.ecs.insert(level);
    // Create player:
    let player_ent = spawner::spawn_player(&mut gs.ecs, pl_spawn_pos);
    gs.ecs.insert(player_ent);

    rltk::main_loop(ctx, gs)
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<PlayerChar>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<MonsterChar>();
    gs.ecs.register::<GameplayName>();
    gs.ecs.register::<TileBlocker>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<MeleeAttackIntent>();
    gs.ecs.register::<IncomingDamage>();
}
