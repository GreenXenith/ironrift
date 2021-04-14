use bevy::prelude::*;
use bevy::app::Events;
use bevy::input::keyboard::KeyCode;

mod camera;
use camera::CameraPlugin;

mod map;
use map::MapPlugin;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Ironrift".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(quit.system())
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .run();
}

fn quit (keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<bevy::app::AppExit>>) {
    if keys.pressed(KeyCode::Escape) {
        return exit.send(bevy::app::AppExit);
    }
}
