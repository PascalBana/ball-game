use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<StarSpawnTimer>()
            // Enter State Systems
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // On Update Sate Systems
            .add_systems(
                (
                    tick_star_spawn_timer,
                    spawn_stars_over_time,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));      
    }
}