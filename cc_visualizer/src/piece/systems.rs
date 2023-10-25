use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use cc::Board;

use crate::board::events::BoardClick;

use super::components::{HighlightedSquare, Piece};
use super::constants::*;
use super::events::PieceMoved;
use super::resources::SelectedPiece;

/// Sets up the board
pub fn setup_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<crate::board::resources::Board>,
) {
    let c_board: &Board = &board.0;
    for (y, row) in c_board.board.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let transform = get_transform_for_piece(x as i8, y as i8, c_board);

            if *item != cc::INVALID && *item != cc::EMPTY {
                let color = match *item {
                    1 => PLAYER1_COLOR,
                    2 => PLAYER2_COLOR,
                    _ => unreachable!(),
                };

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(PIECE_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(color)),
                        transform,
                        ..Default::default()
                    },
                    Piece {
                        point: (x as i8, y as i8),
                    },
                ));
            }
        }
    }
}

pub fn select_click_piece(
    mut board_clicks: EventReader<BoardClick>,
    pieces: Query<(Entity, &Piece)>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for board_click in board_clicks.iter() {
        let clicked_piece = pieces.iter().find_map(|(entity, piece)| {
            if piece.point == board_click.point {
                Some((entity, piece))
            } else {
                None
            }
        });

        if let Some(piece) = clicked_piece {
            selected_piece.piece = Some(*piece.1);
            selected_piece.entity = Some(piece.0);
            return;
        }
    }
}

pub fn clicked_to_move_to_highlighted(
    mut selected_piece: ResMut<SelectedPiece>,
    highlighted: Query<&HighlightedSquare>,
    mut board_clicks: EventReader<BoardClick>,
    mut move_piece: EventWriter<PieceMoved>,
) {
    if selected_piece.piece.is_none() {
        return;
    }

    for board_click in board_clicks.iter() {
        let clicked_piece = highlighted.iter().find_map(|highlighted| {
            if highlighted.point == board_click.point {
                Some(highlighted)
            } else {
                None
            }
        });

        if let Some(piece) = clicked_piece {
            move_piece.send(PieceMoved {
                entity: selected_piece.entity.unwrap(),
                piece: selected_piece.piece.unwrap(),
                to_point: piece.point,
            });
            selected_piece.piece = None;
            dbg!(piece.point);
        }
    }
}

pub fn highlight_possible_squares(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<crate::board::resources::Board>,
    selected_piece: Res<SelectedPiece>,
    highlighted_squares: Query<Entity, With<HighlightedSquare>>,
) {
    if !selected_piece.is_changed() {
        return;
    }

    // Remove old highlighted squares
    for entity in highlighted_squares.iter() {
        commands.entity(entity).despawn();
    }

    if selected_piece.piece.is_none() {
        return;
    }

    let selected_piece = selected_piece.piece.unwrap();
    let board: &Board = &board.0;

    let possible_moves = board.get_legal_moves(selected_piece.point);

    for (x, y) in possible_moves {
        let transform = get_transform_for_piece(x, y, board);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform,
                ..Default::default()
            },
            HighlightedSquare { point: (x, y) },
        ));
    }
}

pub fn move_piece(
    mut moved_pieces: EventReader<PieceMoved>,
    mut board: ResMut<crate::board::resources::Board>,
    mut commands: Commands,
) {
    if moved_pieces.len() == 0 {
        return;
    }

    let board = &mut board.0;

    for moved_piece in moved_pieces.iter() {
        board.move_pice((moved_piece.piece.point, moved_piece.to_point));
        dbg!(moved_piece.to_point);

        let (x, y) = moved_piece.to_point;
        commands.entity(moved_piece.entity).insert((
            get_transform_for_piece(x, y, &board),
            Piece {
                point: moved_piece.to_point,
            },
        ));
    }
}
// TODO: Refacotr this out of piece and board to utils
fn get_transform_for_piece(x: i8, y: i8, board: &Board) -> Transform {
    let transform = Transform::from_xyz(
        (x as f32 - (board.width as f32) / 2.) * 25.,
        (y as f32 - (board.height as f32) / 2.) * 35.,
        1.,
    );
    transform
}
