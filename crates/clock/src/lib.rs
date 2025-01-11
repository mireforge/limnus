pub mod prelude;

use limnus_app::prelude::{App, Plugin};
use limnus_local_resource::LocalResource;
use limnus_macros::{LocalResource, Resource};
use limnus_resource::Resource;
use limnus_system_params::{LoRe, ReM};
use limnus_system_runner::UpdatePhase;
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
        app.insert_local_resource(Clock {
            clock: Box::new(create_monotonic_clock()),
        });
        app.insert_resource(MonotonicTime {
            time: Millis::from(0),
        });

        app.add_system(UpdatePhase::First, update_time);
    }
}
