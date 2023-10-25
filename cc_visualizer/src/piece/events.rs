use bevy::prelude::*;
use cc::Point;

use super::components::Piece;

#[derive(Event)]
pub struct PieceMoved {
    pub piece: Piece,
    pub entity: Entity,
    pub to_point: Point,
}
