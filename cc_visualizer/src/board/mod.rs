use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use events::BoardClick;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_event::<BoardClick>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, board_click);
    }
}
