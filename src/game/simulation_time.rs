use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, First, IntoSystemConfigs, Res, ResMut, Resource, States, Time};

pub struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default())
            .add_systems(First, update.run_if(in_state(SimulationState::Running)));
    }
}

#[derive(Resource)]
struct SimulationTime {
    delta_seconds: f32,
    elapsed: f32,
    scale: f32,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum SimulationState {
    #[default]
    Running,
    Paused,
}

fn update(mut simulation: ResMut<SimulationTime>, time: Res<Time>) {
    simulation.delta_seconds = time.delta_seconds() * simulation.scale;
    simulation.elapsed += simulation.delta_seconds;
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            delta_seconds: 0.0,
            elapsed: 0.0,
            scale: 1.0,
        }
    }
}

impl SimulationTime {
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }
}
