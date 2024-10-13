#![allow(unused_imports)]
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};

#[derive(Component, Clone)]
pub struct Stats {
    generation: u32,
    population: u32,
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
    is_playing: bool,
    reset: bool,
    time_step: u32,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self {
            is_playing: false,
            reset: false,
            time_step: 50,
        }
    }
}

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

#[derive(Component)]
pub struct Name(String);

pub fn tstprint(time: Res<Time>, mut timer: ResMut<GreetTimer>, guisettings: Res<GuiSettings>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Time step is: {:?}ms", guisettings.time_step)
    }
}

pub fn ui_gui_window(mut contexts: EguiContexts, mut guisettings: ResMut<GuiSettings>) {
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
            egui::Slider::new(&mut guisettings.time_step, 50..=1000)
                .step_by(50.0)
                .text("Game Speed:"),
        );
        ui.horizontal(|ui| {
            if ui.button("Start").clicked() {
                //self.is_paused = false;
            }
            if ui.button("Stop").clicked() {
                //self.is_paused = true;
            }
        });

        if ui.button("Step").clicked() {
            //self.population += 1;
        }
        ui.label(format!("Generation: {}", 0));
        ui.label(format!("Population: {}", 0));
    });
}
