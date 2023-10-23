use bevy::prelude::*;

#[derive(Resource)]
pub struct Board(pub cc::Board);

impl Default for Board {
    fn default() -> Self {
        let mut board = Self(cc::Board::new());
        board.0.add_player(cc::board::PLAYER_1_POS, 1);
        board.0.add_player(cc::board::PLAYER_2_POS, 2);
        board
    }
}
