use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use cc::Board;

use super::components::*;
use super::constants::*;
use super::resources;

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<resources::Board>,
) {
    let c_board: &Board = &board.0;
    for (y, row) in c_board.board.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let transform = get_transform_for_empty(x as i8, y as i8, c_board);

            if *item != cc::INVALID {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(EMPTY_RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(EMPTY_COLOR)),
                        transform,
                        ..Default::default()
                    },
                    Empty {
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
