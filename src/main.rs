// source: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/window/clear_color.rs

//! Shows how to set the solid color that is used to paint the window before the frame gets drawn.
//!
//! Acts as background color, since pixels that are not drawn in a frame remain unchanged.

use bevy::{core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseMotion, prelude::*};

// Useful for marking the "main" camera if we have many
#[derive(Component)]
pub struct MainCamera;

fn initialize_cameras(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube { size: 1.0 }.into()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 500_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            // renders after / on top of the main camera
            order: 1,
            ..default()
        },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            // renders after / on top of the main camera
            order: 2,
            ..default()
        },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_cameras);
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Update, change_clear_color)
        .add_systems(Update, mouse_coordinates)
        .run();
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}

fn mouse_coordinates(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut input: EventReader<MouseMotion>,
) {
    for _ in input.read() {
        let window = window_query.single();
        let (camera, camera_transform) = camera_query.single();
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            info!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }
}
