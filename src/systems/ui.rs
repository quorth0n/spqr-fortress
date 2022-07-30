use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    constants::TICK_LENGTH,
    resources::state::{GameSpeed, GameState},
};

pub struct TickTimer(pub Timer);

pub fn paint_ui(
    mut egui_context: ResMut<EguiContext>,
    mut speed: ResMut<GameSpeed>,
    mut timer: ResMut<TickTimer>,
    mut state: ResMut<State<GameState>>,
) {
    // Top bar
    egui::TopBottomPanel::top("cool panel").show(&egui_context.ctx_mut(), |ui| {
        // Speed controls
        if ui.button("-").clicked() {
            speed.0 = match speed.0 {
                0 => 0,
                i => i - 1,
            };

            if speed.0 == 0 && *state.current() != GameState::Paused {
                state.set(GameState::Paused).unwrap();
            }

            let adj_ms = (6 - speed.0) * TICK_LENGTH;
            timer.0.set_duration(Duration::from_millis(adj_ms));
        }
        ui.label(format!("Speed {}", speed.0));
        if ui.button("+").clicked() {
            speed.0 = match speed.0 {
                5 => 5,
                i => i + 1,
            };

            if *state.current() != GameState::Playing {
                state.set(GameState::Playing).unwrap();
            }

            let adj_ms = (6 - speed.0) * TICK_LENGTH;
            timer.0.set_duration(Duration::from_millis(adj_ms));
        }
    });
}
