use bevy::prelude::*;

use super::components::Piece;
#[derive(Resource, Default)]
pub struct SelectedPiece {
    pub piece: Option<Piece>,
    pub entity: Option<Entity>,
}
