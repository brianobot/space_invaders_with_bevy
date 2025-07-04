use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
