use bevy::prelude::*;
use bevy::app::Events;
use bevy::input::keyboard::KeyCode;

mod camera;
mod map;
mod player;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Ironrift".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(quit.system())
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .run();
}

fn quit (keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<bevy::app::AppExit>>) {
    if keys.pressed(KeyCode::Escape) {
        return exit.send(bevy::app::AppExit);
    }
}
