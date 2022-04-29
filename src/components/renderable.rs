use rltk::{FontCharType, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
