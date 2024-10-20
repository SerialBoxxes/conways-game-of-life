#![allow(unused_imports)]
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};

#[derive(Resource, Clone)]
pub struct Stats {
    pub generation: u32,
    pub population: u32,
    draw_traces: bool,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            generation: 0,
            population: 0,
            draw_traces: false,
        }
    }
}

#[derive(Resource, Clone)]
pub struct GuiSettings {
    pub is_playing: bool,
    pub play_step: bool,
    reset: bool,
    pub time_step: f64,
    pub paint: bool,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self {
            is_playing: false,
            play_step: false,
            reset: false,
            time_step: 10.0,
            paint: false,
        }
    }
}

pub fn ui_gui_window(
    mut contexts: EguiContexts,
    mut guisettings: ResMut<GuiSettings>,
    stats: Res<Stats>,
) {
    egui::Window::new("Conway's Game of Life").show(contexts.ctx_mut(), |ui| {
        //});
        //egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("Settings");
        /*ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name)
                .labelled_by(name_label.id);
        });*/
        ui.add(
            egui::Slider::new(&mut guisettings.time_step, 1.0..=100.0)
                .step_by(1.0)
                .text("Game Speed: TPS"),
        );
        ui.horizontal(|ui| {
            if ui.button("Start").clicked() {
                guisettings.is_playing = true;
            }
            if ui.button("Stop").clicked() {
                guisettings.is_playing = false;
            }
        });

        if ui.button("Step").clicked() {
            guisettings.play_step = true;
        }
        ui.label(format!("Generation: {}", stats.generation));
        ui.label(format!("Population: {}", stats.population));
    });
}
