use bevy::prelude::*;
use cc::Point;

#[derive(Event)]
pub struct BoardClick {
    pub point: Point,
}
