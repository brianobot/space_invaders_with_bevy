use bevy::{prelude::*, window::PrimaryWindow};

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimension: Vec2,
    pub pixel_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.insert_resource(Resolution {
        screen_dimension: Vec2::new(window.width(), window.height()),
        pixel_ratio: 2.0,
    });
}
