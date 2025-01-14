#![allow(unused_imports)]
use bevy::input::mouse::MouseButton;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy_pancam::PanCam;
use std::cmp::Ordering;

#[derive(Component)]
struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(100.0, 200.0, 0.0),
                ..default()
            },
            MyCameraMarker,
        ))
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Right, MouseButton::Middle],
            ..Default::default()
        });
}

use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

use crate::GuiSettings;

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut evr_scroll: EventReader<MouseWheel>,
    //mut evr_mouse: EventReader<MouseButton>,
    mut guisettings: ResMut<GuiSettings>,
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

        if keyboard_input.pressed(KeyCode::KeyF) {
            guisettings.paint = true
        } else {
            guisettings.paint = false
        }

        //for evmb in evr_mouse.read() {}

        for ev in evr_scroll.read() {
            match ev.y {
                d if d < 0.0 => {
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
                }
                _ => {}
            }
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
