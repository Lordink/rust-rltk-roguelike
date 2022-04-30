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
