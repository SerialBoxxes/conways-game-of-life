#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod gui_initialize;
use gui_initialize::*;

mod rendering;
use rendering::*;

use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_tilemap::prelude::*;

fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let map_size = TilemapSize { x: 128, y: 96 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    commands.entity(tilemap_entity).with_children(|builder| {
        // Spawn the elements of the tilemap.
        // Alternatively, you can use helpers::filling::fill_tilemap.
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };
                let center = tile_pos.center_in_world(&grid_size, &map_type);

                let tile_entity = builder
                    .spawn((TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(5),
                        ..Default::default()
                    },))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(
            &map_size, &grid_size, &map_type, 0.0,
        ),
        ..Default::default()
    });
}

fn update_world(){
    
}

pub fn game() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(
                            "Conway's Game of Life",
                        ),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        
        .add_plugins((EguiPlugin,TilemapPlugin))
        .insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(GuiSettings::default())
        .add_systems(Startup, (setup_camera, setup_world))
        .add_systems(Update, (ui_gui_window, movement))
        .run();
}

fn main() {
    game();
}
