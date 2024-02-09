// adapted from the 2d/text2d.rs example here: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/2d/text2d.rs

use bevy::{input::mouse::MouseWheel, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                mouse_coordinates,
                keyboard_input_system,
                mouse_input_system,
                mouse_scroll_event_system,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct MyText;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::BLACK,
    };

    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Hello, Bevy!", text_style)
                .with_alignment(TextAlignment::Center),
            ..default()
        },
        MyText,
    ));
}

fn mouse_coordinates(window_query: Query<&Window>, mut text_query: Query<&mut Text, With<MyText>>) {
    let window = window_query.single();
    if let Some(world_position) = window.cursor_position() {
        let mut text = text_query.single_mut();
        if let Some(text) = text.sections.iter_mut().next() {
            text.value = format!("Coordinates x:{}, y:{}", world_position.x, world_position.y);
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if keyboard_input.pressed(KeyCode::Left) {
        let mut camera = camera_query.single_mut();
        camera.translation.x -= 2.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        let mut camera = camera_query.single_mut();
        camera.translation.y += 2.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        let mut camera = camera_query.single_mut();
        camera.translation.x += 2.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        let mut camera = camera_query.single_mut();
        camera.translation.y -= 2.0;
    }
}

fn mouse_input_system(
    mouse_input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(0.05);
    }

    if mouse_input.pressed(MouseButton::Left) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(-0.05);
    }

    if mouse_input.pressed(MouseButton::Middle) {
        let mut camera = camera_query.single_mut();
        camera.rotation = Quat::IDENTITY;
    }
}

fn mouse_scroll_event_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    for ev in scroll_evr.read() {
        let mut camera = camera_query.single_mut();
        match ev.y {
            _value if _value > 0.0 => {
                camera.scale.x *= 0.9;
                camera.scale.y *= 0.9;
            }
            _value if _value < 0.0 => {
                camera.scale.x *= 1.1;
                camera.scale.y *= 1.1;
            }
            _ => {}
        }
    }
}
