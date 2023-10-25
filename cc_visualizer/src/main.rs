use bevy::{prelude::*, window::PrimaryWindow};

mod ai;
mod board;
mod piece;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.96, 0.96, 0.96);

fn main() {
    App::new()
        .add_event::<WorldPosClick>()
        .add_plugins((DefaultPlugins, board::BoardPlugin, piece::PiecePlugin))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(PreUpdate, world_pos_click)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Event)]
pub struct WorldPosClick(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
}

fn world_pos_click(
    mouse_buttons: Res<Input<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut world_pos_click_writer: EventWriter<WorldPosClick>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let world_pos = camera
        .viewport_to_world(camera_transform, window.cursor_position().unwrap())
        .unwrap()
        .origin
        .truncate();
    world_pos_click_writer.send(WorldPosClick(world_pos));
}
