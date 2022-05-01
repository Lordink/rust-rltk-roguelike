use specs::prelude::*;
use specs_derive::Component;

pub mod position;
pub mod renderable;
pub mod viewshed;

pub use position::*;
pub use renderable::*;
pub use viewshed::*;

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
pub struct PlayerChar {}

#[derive(Component, Debug)]
pub struct MonsterChar {}

/// Anyone with this comp has a name we can show to the player
#[derive(Component, Debug)]
pub struct GameplayName {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct TileBlocker {}
