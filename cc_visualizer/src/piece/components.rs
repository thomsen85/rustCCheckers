use bevy::prelude::*;
use cc::Point;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub point: Point,
}

#[derive(Component)]
pub struct HighlightedSquare {
    pub point: Point,
}
