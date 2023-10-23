mod board;

use bevy::{
    ecs::entity, input::mouse::MouseButtonInput, prelude::*, sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use board::{components, resources};
use cc::board::Board;

#[derive(Resource, Default)]
struct SelectedPiece {
    piece: Option<Piece>,
}

const BACKGROUND_COLOR: Color = Color::rgb(0.96, 0.96, 0.96);
const PLAYER1_COLOR: Color = Color::rgb(0.60, 0.73, 0.91);
const PLAYER2_COLOR: Color = Color::rgb(0.65, 0.81, 0.50);
const PIECE_RADIUS: f32 = 15.;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, board::BoardPlugin))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<SelectedPiece>()
        .add_systems(Startup, setup)
        .add_systems(Update, (on_click_piece, highlight_possible_squares))
        .run();
}

#[derive(Component, Clone, Copy, Debug)]
struct Piece {
    x: i8,
    y: i8,
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<board::resources::Board>,
) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));

    let c_board: &Board = &board.0;
    for (y, row) in c_board.board.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let transform = get_transform_for_empty(x as i8, y as i8, c_board);

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
                        transform: Transform {
                            translation: transform.translation + Vec3::new(0., 0., 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Piece {
                        x: x as i8,
                        y: y as i8,
                    },
                ));
            }
        }
    }
}

fn get_transform_for_empty(x: i8, y: i8, c_board: &Board) -> Transform {
    let transform = Transform::from_xyz(
        (x as f32 - (c_board.width as f32) / 2.) * 25.,
        (y as f32 - (c_board.height as f32) / 2.) * 35.,
        0.,
    );
    transform
}

fn get_transform_for_piece(x: i8, y: i8, c_board: &Board) -> Transform {
    let transform = Transform::from_xyz(
        (x as f32 - (c_board.width as f32) / 2.) * 25.,
        (y as f32 - (c_board.height as f32) / 2.) * 35.,
        1.,
    );
    transform
}

fn on_click_piece(
    mouse_buttons: Res<Input<MouseButton>>,
    q_pieces: Query<(Entity, &Piece, &Transform)>,
    q_empties: Query<(&board::components::Empty, &Transform)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut board: ResMut<resources::Board>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut commands: Commands,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    let world_pos = get_world_pos(camera, camera_transform, window);

    let clicked_piece = q_pieces.iter().find_map(|(entity, piece, transform)| {
        if (transform.translation.truncate() - world_pos).length() < PIECE_RADIUS {
            Some((entity, piece, transform))
        } else {
            None
        }
    });

    if let Some(piece) = clicked_piece {
        selected_piece.piece = Some(*piece.1);
        selected_piece.entity = Some(piece.0);
        return;
    }

    let clicked_pos = q_empties.iter().find_map(|(piece, transform)| {
        if (transform.translation.truncate() - world_pos).length() < PIECE_RADIUS {
            Some(piece)
        } else {
            None
        }
    });

    if clicked_pos.is_none() {
        selected_piece.piece = None;
        return;
    }

    let clicked_pos = clicked_pos.unwrap();

    if let Some(selcted_piece) = selected_piece.piece {
        let c_board = &mut board.as_mut().0;
        let possible_moves = c_board.get_legal_moves((selcted_piece.x, selcted_piece.y));
        let clicked_point = (clicked_pos.x, clicked_pos.y);

        if possible_moves.contains(&clicked_point) {
            dbg!(&selcted_piece, &clicked_point);
            move_piece(c_board, clicked_point, commands, selected_piece);
        }
    }
}

fn move_piece(
    c_board: &mut Board,
    clicked_point: (i8, i8),
    mut commands: Commands<'_, '_>,
    mut selected_piece: ResMut<'_, SelectedPiece>,
) {
    c_board.move_pice((
        (
            selected_piece.piece.unwrap().x,
            selected_piece.piece.unwrap().y,
        ),
        clicked_point,
    ));

    commands.entity(selected_piece.entity.unwrap()).insert((
        get_transform_for_piece(clicked_point.0, clicked_point.1, c_board),
        Piece {
            x: clicked_point.0,
            y: clicked_point.1,
        },
    ));
    selected_piece.piece = None;
}

#[derive(Component)]
struct PossibleDot;

fn highlight_possible_squares(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<resources::Board>,
    selected_piece: Res<SelectedPiece>,
    highlighted_squares: Query<Entity, With<PossibleDot>>,
) {
    if !selected_piece.is_changed() {
        return;
    }

    // Remove old highlighted squares
    for entity in highlighted_squares.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(piece) = selected_piece.piece {
        let c_board: &Board = &board.0;
        let possible_moves = c_board.get_legal_moves((piece.x, piece.y));

        for (x, y) in possible_moves {
            let transform = Transform::from_xyz(
                (x as f32 - (c_board.width as f32) / 2.) * 25.,
                (y as f32 - (c_board.height as f32) / 2.) * 35.,
                1.,
            );

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform,
                    ..Default::default()
                },
                PossibleDot,
            ));
        }
    }
}

fn get_world_pos(camera: &Camera, camera_transform: &GlobalTransform, window: &Window) -> Vec2 {
    let world_pos = camera
        .viewport_to_world(camera_transform, window.cursor_position().unwrap())
        .unwrap()
        .origin
        .truncate();
    world_pos
}
