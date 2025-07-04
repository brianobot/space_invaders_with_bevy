use bevy::prelude::*;

mod games;
#[allow(unused)]
use games::*;

mod alien;
#[allow(unused)]
use alien::*;

mod resolution;
#[allow(unused)]
use resolution::*;

mod player;
#[allow(unused)]
use player::*;

fn main() {
    App::new()
        // this adds features like Window management to the application
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders 🚀"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                // this prevents the pixel from becoming blurry with the standard by linear settings
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ResolutionPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(AlienPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
