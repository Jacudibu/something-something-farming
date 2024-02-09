use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, First, IntoSystemConfigs, Res, ResMut, Resource, States, Time};

pub struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default())
            .add_state::<SimulationState>()
            .add_systems(First, update.run_if(in_state(SimulationState::Running)));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum SimulationState {
    #[default]
    Running,
    Paused,
}

fn update(mut simulation: ResMut<SimulationTime>, time: Res<Time>) {
    simulation.advance(time.delta());
}

#[derive(Resource)]
pub struct SimulationTime {
    elapsed: Duration,
    delta: Duration,
    delta_seconds: f32,
    scale: f32,
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            delta_seconds: 0.0,
            scale: 1.0,
        }
    }
}

impl SimulationTime {
    fn advance(&mut self, delta: Duration) {
        self.delta = delta.mul_f32(self.scale);
        self.delta_seconds = delta.as_secs_f32();

        self.elapsed += self.delta;
    }

    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    pub fn elapsed_seconds(&self) -> u64 {
        self.elapsed.as_secs()
    }
}
