use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use self::events::PieceMoved;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPiece>()
            .add_event::<PieceMoved>()
            .add_systems(Startup, setup_pieces)
            .add_systems(
                Update,
                (
                    select_click_piece,
                    clicked_to_move_to_highlighted,
                    highlight_possible_squares,
                    move_piece,
                ),
            );
    }
}
