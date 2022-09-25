use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use plugins::{StatePlugin, UIPlugin};

mod components;
mod constants;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // tick systems
        .add_plugin(StatePlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("index.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(75.0, 75.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    });
}
