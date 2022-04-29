use specs::prelude::*;
use specs_derive::Component;

pub mod position;
pub mod renderable;

pub use position::*;
pub use renderable::*;

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
pub struct PlayerChar {}
