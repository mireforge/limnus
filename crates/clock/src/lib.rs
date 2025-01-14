pub mod prelude;

use limnus_app::prelude::{App, Plugin};
use limnus_default_stages::First;
use limnus_local_resource::LocalResource;
use limnus_macros::{LocalResource, Resource};
use limnus_resource::Resource;
use limnus_system_params::{LoRe, ReM};
use monotonic_time_rs::{create_monotonic_clock, Millis, MonotonicClock};
use std::fmt::{Debug, Formatter};

#[derive(LocalResource)]
pub struct Clock {
    pub clock: Box<dyn MonotonicClock>,
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "clock")
    }
}

#[derive(Debug, Resource)]
pub struct MonotonicTime {
    pub time: Millis,
}

fn update_time(clock: LoRe<Clock>, mut time: ReM<MonotonicTime>) {
    time.time = clock.clock.now();
}

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        let clock = create_monotonic_clock();
        let now = clock.now();
        app.insert_local_resource(Clock {
            clock: Box::new(clock),
        });
        app.insert_resource(MonotonicTime { time: now });

        app.add_system(First, update_time);
    }
}
