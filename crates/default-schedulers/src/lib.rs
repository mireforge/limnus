use limnus_app::prelude::{App, Plugin};
use limnus_clock::MonotonicTime;
use limnus_default_stages::{
    First, FixedFirst, FixedPostUpdate, FixedPreUpdate, FixedUpdate, PostUpdate, PreUpdate,
    RenderFirst, RenderPostUpdate, RenderPreUpdate, RenderUpdate, Update,
};
use limnus_resource::prelude::Resource;
use limnus_scheduler::Scheduler;
use limnus_stage::Stages;
use limnus_system_params::MsgAll;
use limnus_system_state::State;
use monotonic_time_rs::{Millis, MillisDuration};
use std::any::TypeId;

#[derive(Debug)]
pub struct MainScheduler;
impl Scheduler for MainScheduler {
    fn schedule(&self, stages: &Stages, state: &mut State) {
        let stage_ids = {
            vec![
                TypeId::of::<First>(),
                TypeId::of::<PreUpdate>(),
                TypeId::of::<Update>(),
                TypeId::of::<PostUpdate>(),
            ]
        };

        for &stage_id in &stage_ids {
            stages
                .get_by_id(&stage_id)
                .expect("stage missing")
                .run(state);
        }
    }
}

#[derive(Debug, Resource)]
pub struct FixedSchedulerData {
    pub consumed_up_to_time: Millis,
    pub fixed_time_step: MillisDuration,
}

#[derive(Debug)]
pub struct FixedScheduler;
impl Scheduler for FixedScheduler {
    fn schedule(&self, stages: &Stages, state: &mut State) {
        let time = { state.resources().fetch::<MonotonicTime>().time };

        let (mut consumed_up_to_time, fixed_time_step) = {
            let data = state.resources().fetch::<FixedSchedulerData>();
            (data.consumed_up_to_time, data.fixed_time_step)
        };

        let stage_ids = {
            vec![
                TypeId::of::<FixedFirst>(),
                TypeId::of::<FixedPreUpdate>(),
                TypeId::of::<FixedUpdate>(),
                TypeId::of::<FixedPostUpdate>(),
            ]
        };

        let mut tick_count = 0;

        while consumed_up_to_time < time {
            // TODO: Make a better algorithm for this
            tick_count += 1;
            for &stage_id in &stage_ids {
                stages
                    .get_by_id(&stage_id)
                    .expect("stage missing")
                    .run(state);
            }
            consumed_up_to_time += fixed_time_step;
            if consumed_up_to_time < time {
                let still_lagging_duration = time - consumed_up_to_time;
                if still_lagging_duration < fixed_time_step {
                    break;
                }
            }
            if tick_count > 2 {
                break;
            }
        }

        {
            let fixed_scheduler_data = state.resources_mut().fetch_mut::<FixedSchedulerData>();
            fixed_scheduler_data.consumed_up_to_time = consumed_up_to_time;
        }
    }
}

#[derive(Debug)]
pub struct RenderScheduler;
impl Scheduler for RenderScheduler {
    fn schedule(&self, stages: &Stages, state: &mut State) {
        // TODO: Should have settings for min and max fps
        let stage_ids = {
            vec![
                TypeId::of::<RenderFirst>(),
                TypeId::of::<RenderPreUpdate>(),
                TypeId::of::<RenderUpdate>(),
                TypeId::of::<RenderPostUpdate>(),
            ]
        };

        for &stage_id in &stage_ids {
            stages
                .get_by_id(&stage_id)
                .expect("stage missing")
                .run(state);
        }
    }
}

fn swap_messages(mut messages: MsgAll) {
    messages.swap_all();
}

pub struct DefaultSchedulersPlugin;

impl Plugin for DefaultSchedulersPlugin {
    fn build(&self, app: &mut App) {
        let time = { app.resources().fetch::<MonotonicTime>().time };

        app.insert_resource(FixedSchedulerData {
            consumed_up_to_time: time,
            fixed_time_step: MillisDuration::from_millis(16),
        });

        app.add_scheduler(MainScheduler);
        app.add_scheduler(FixedScheduler);
        app.add_scheduler(RenderScheduler);

        app.add_system(First, swap_messages);
    }
}
