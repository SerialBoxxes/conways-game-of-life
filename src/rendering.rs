#![allow(unused_imports)]
use std::cmp::Ordering;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};

#[derive(Component)]
struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        use bevy::input::mouse::MouseScrollUnit;
        
        for ev in evr_scroll.read() {
            match ev.y {
                d if d <0.0 => {
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            ortho.scale += 0.1;
                        }
                        MouseScrollUnit::Pixel => {
                            //ortho.scale -= 0.1;
                        }
                    }
                }
                0.0 => {}
                d if d > 0.0 => {
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            ortho.scale -= 0.1;
                        }
                        MouseScrollUnit::Pixel => {
                            //ortho.scale -= 0.1;
                        }
                    }
                } _ => {}
            }
        
            if ortho.scale < 0.5 {
                ortho.scale = 0.5;
            }

            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 500.;
            // Important! We need to restore the Z values when moving the camera around.
            // Bevy has a specific camera setup and this can mess with how our layers are shown.
            transform.translation.z = z;
        }
    }
}
