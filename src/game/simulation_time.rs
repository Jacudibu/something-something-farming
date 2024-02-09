use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::ecs::reflect::ReflectResource;
use bevy::prelude::{
    in_state, First, IntoSystemConfigs, Reflect, Res, ResMut, Resource, States, Time,
};
use bevy_inspector_egui::prelude::{InspectorOptions, ReflectInspectorOptions};

pub struct SimulationTimePlugin;
impl Plugin for SimulationTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTime::default())
            .register_type::<SimulationTime>()
            .insert_resource(SimulationDate::default())
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

fn update(
    mut simulation_time: ResMut<SimulationTime>,
    mut date: ResMut<SimulationDate>,
    real_time: Res<Time>,
) {
    simulation_time.advance(real_time.delta());
    *date = SimulationDate::from_time(&simulation_time);
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SimulationTime {
    elapsed: Duration,
    delta: Duration,
    delta_seconds: f32,
    scale: f32,
}

#[derive(Resource)]
pub struct SimulationDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Default for SimulationDate {
    fn default() -> Self {
        SimulationDate {
            year: 1,
            month: 1,
            day: 1,
            hour: 8,
            minute: 0,
            second: 0,
        }
    }
}

const DAYS_PER_MONTH: u64 = 28;
const MONTHS_PER_YEAR: u64 = 4;
const START_OFFSET: u64 = SECONDS_PER_HOUR * 8;

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * 60;
const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;
const SECONDS_PER_MONTH: u64 = SECONDS_PER_DAY * DAYS_PER_MONTH;
const SECONDS_PER_YEAR: u64 = SECONDS_PER_MONTH * MONTHS_PER_YEAR;

impl SimulationDate {
    fn from_time(time: &SimulationTime) -> Self {
        let mut remaining_seconds = time.elapsed.as_secs() + START_OFFSET;
        let year = remaining_seconds / SECONDS_PER_YEAR;
        remaining_seconds -= year * SECONDS_PER_YEAR;

        let month = remaining_seconds / SECONDS_PER_MONTH;
        remaining_seconds -= month * SECONDS_PER_MONTH;

        let day = remaining_seconds / SECONDS_PER_DAY;
        remaining_seconds -= day * SECONDS_PER_DAY;

        let hour = remaining_seconds / SECONDS_PER_HOUR;
        remaining_seconds -= hour * SECONDS_PER_HOUR;

        let minute = remaining_seconds / SECONDS_PER_MINUTE;
        remaining_seconds -= minute * SECONDS_PER_MINUTE;

        SimulationDate {
            year: year as u32,
            month: month as u8,
            day: day as u8,
            hour: hour as u8,
            minute: minute as u8,
            second: remaining_seconds as u8,
        }
    }
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
    pub fn elapsed_seconds_f32(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
