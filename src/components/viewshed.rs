use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    /// If true - this entity has moved, thus we need to rebuild its visibility field of view
    pub is_dirty: bool,
}
impl Viewshed {
    pub fn new() -> Self {
        Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            is_dirty: true,
        }
    }
}
