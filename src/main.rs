use bevy::prelude::*;
use bevy::app::Events;
use bevy::asset::LoadState;
use bevy::input::keyboard::KeyCode;

mod map;
mod unit;
mod bullet;
mod player;
mod camera;
mod hud;
mod npc;
mod battle;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Terrain,
    Unit,
}

// Asset loader
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Loaded,
}

#[derive(Default)]
struct AssetHandles {
    handles: Vec<HandleUntyped>,
}

fn load_assets(mut asset_handles: ResMut<AssetHandles>, assets: Res<AssetServer>) {
    asset_handles.handles = assets.load_folder("").unwrap();
}

fn check_assets(
    mut state: ResMut<State<AppState>>,
    asset_handles: ResMut<AssetHandles>,
    assets: Res<AssetServer>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(asset_handles.handles.iter().map(|handle| handle.id)) {
        state.set(AppState::Loaded).unwrap();
    }
}

fn main() {
    App::build()
        // Config
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Ironrift".to_string(),
            ..Default::default()
        })

        // Defaults and setup
        .add_plugins(DefaultPlugins)
        .add_system(quit.system())

        // Handle assets
        .init_resource::<AssetHandles>()
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets.system()))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets.system()))

        // Now load the game
        .add_plugin(map::MapPlugin)
        .add_plugin(unit::UnitPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(npc::NpcPlugin)
        .add_plugin(battle::BattlePlugin)

        .run();
}

fn quit (keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<bevy::app::AppExit>>) {
    if keys.pressed(KeyCode::Escape) {
        return exit.send(bevy::app::AppExit);
    }
}
