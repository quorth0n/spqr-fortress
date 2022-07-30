use bevy::prelude::*;

use crate::{
    constants::TICK_LENGTH,
    resources::state::{GameSpeed, GameState},
    systems::{
        state::camera_control,
        ui::{paint_ui, TickTimer},
    },
};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // defaults
        app.insert_resource(GameSpeed(0))
            .add_state(GameState::Paused)
            .add_system(camera_control);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(TICK_LENGTH as f32, true)))
            .add_system(paint_ui);
    }
}

pub struct CharPlugin;
impl Plugin for CharPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(TICK_LENGTH as f32, true)))
            .add_system(paint_ui);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
