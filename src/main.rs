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

#[derive(Component)]
pub struct LastUpdate(f64);

const MSIZE:u32 = 128;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles-no-border.png");

    let map_size = TilemapSize { x: MSIZE, y: MSIZE };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut i = 0;
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    visible: TileVisible(i % 5 == 0 || i % 13 == 0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
            i += 1;
        }
    }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        LastUpdate(0.0),
    ));
}

//#[derive(Component)]
//pub struct LastUpdate(f64);
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut tile_storage_query: Query<(&TileStorage, &TilemapSize, &mut LastUpdate)>,
    tile_query: Query<(Entity, &TilePos, &TileVisible)>,
    mut guisettings: ResMut<GuiSettings>,
    mut stats: ResMut<Stats>,
) {
    let current_time = time.elapsed_seconds_f64();
    let (tile_storage, map_size, mut last_update) = tile_storage_query.single_mut();
    if current_time - last_update.0 > 1.0 / guisettings.time_step {
        if guisettings.is_playing | guisettings.play_step {
            let mut pop: u32 = 0;
            for (entity, position, visibility) in tile_query.iter() {
                let neighbor_count =
                    Neighbors::get_square_neighboring_positions(position, map_size, true)
                        .entities(tile_storage)
                        .iter()
                        .filter(|neighbor| {
                            let (_, _, tile_visible) = tile_query.get(**neighbor).unwrap();
                            tile_visible.0
                        })
                        .count();

                let was_alive = visibility.0;
                if was_alive == true {
                    pop += 1
                }
                let is_alive = match (was_alive, neighbor_count) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                if is_alive && !was_alive {
                    commands.entity(entity).insert(TileVisible(true));
                } else if !is_alive && was_alive {
                    commands.entity(entity).insert(TileVisible(false));
                }
            }
            stats.population = pop;
            stats.generation += 1;
            guisettings.play_step = false;
        }
        last_update.0 = current_time;
    }
}

pub fn game() {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;
    use bevy_pancam::PanCamPlugin;
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Conway's Game of Life"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((EguiPlugin, TilemapPlugin, PanCamPlugin::default()))
        .insert_resource(GuiSettings::default())
        .insert_resource(Stats::default())
        .add_systems(Startup, (setup_camera, startup /*setup_world*/))
        .add_systems(Update, (ui_gui_window, /*movement,*/ update))
        //.add_systems(Update, exit_system)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn main() {
    game();
}
