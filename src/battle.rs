use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TeamId {
    NONE,
    ONE,
    TWO,
}

pub struct Team {
    id: TeamId,
    spawn_point: Vec3,
}

pub struct Battle {
    teams: Vec<Team>,
    units_per_team: i32,
    started: bool,
}

impl Default for Battle {
    fn default() -> Self {
        Self {
            teams: vec![
                Team {id: TeamId::NONE, spawn_point: Vec3::ZERO},
                Team {id: TeamId::NONE, spawn_point: Vec3::ZERO},
            ],
            units_per_team: 0,
            started: false,
        }
    }
}

fn new_battle(
    mut commands: Commands,
) {
    commands.spawn().insert(Battle {
        teams: vec![
            Team {id: TeamId::ONE, spawn_point: Vec3::new(50.0, 1.0, 0.0)},
            Team {id: TeamId::TWO, spawn_point: Vec3::new(-50.0, 1.0, 0.0)},
        ],
        units_per_team: 20,
        ..Default::default()
    });
}

fn battle_handler(
    mut queue: ResMut<crate::npc::SpawnQueue>,
    mut battles: Query<&mut Battle>,
) {
    for mut battle in battles.iter_mut() {
        if !battle.started {
            for team in &battle.teams {
                for _ in 0..battle.units_per_team {
                    queue.waiting.push((team.spawn_point, team.id));
                }
            }
            battle.started = true;
        }
    }
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(new_battle.system().after("spawn_player").after("npc_queue"));
        app.add_system(battle_handler.system());
    }
}
