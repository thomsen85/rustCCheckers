use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    board::resources::Board,
    piece::{components::Piece, events::PieceMoved},
};

use super::resources::PlayerNumsTurn;

pub fn ai_vs_ai(
    mut move_piece: EventWriter<PieceMoved>,
    board: ResMut<Board>,
    mut turn: ResMut<PlayerNumsTurn>,
    pieces: Query<(Entity, &Piece)>,
) {
    let mut rng = thread_rng();

    let board = &board.0;
    let (from, to) = if rng.gen::<f32>() > 0.5 {
        board.closest_move(turn.0 as i8)
    } else {
        board.random_move(turn.0 as i8, &mut rng)
    };
    let piece = pieces
        .iter()
        .find(|(_, piece)| piece.point == from)
        .unwrap();

    move_piece.send(PieceMoved {
        entity: piece.0,
        piece: *piece.1,
        to_point: to,
    });

    turn.0 = if turn.0 == 1 { 2 } else { 1 };
}
