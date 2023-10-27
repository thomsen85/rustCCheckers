use bevy::prelude::*;

use crate::GameState;

pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerNumsTurn>()
            .add_systems(Update, (ai_vs_ai).run_if(in_state(GameState::AiVsAi)));
    }
}
