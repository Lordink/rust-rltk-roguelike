use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: u16,
    pub hp: u16,
    pub defense: i32,
    pub power: u16,
}
