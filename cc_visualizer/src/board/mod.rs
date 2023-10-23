use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(Startup, setup_board);
    }
}
