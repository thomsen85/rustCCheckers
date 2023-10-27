use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerNumsTurn(pub usize);

impl Default for PlayerNumsTurn {
    fn default() -> Self {
        Self(1)
    }
}
