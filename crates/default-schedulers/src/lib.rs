/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

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
    pub ticks_per_second: usize,
}

#[derive(Debug)]
pub struct FixedScheduler;
impl Scheduler for FixedScheduler {
    fn schedule(&self, stages: &Stages, state: &mut State) {
        let current_time = { state.resources().fetch::<MonotonicTime>().time };

        let (mut consumed_time, ticks_per_second) = {
            let data = state.resources().fetch::<FixedSchedulerData>();
            (data.consumed_up_to_time, data.ticks_per_second)
        };

        let steps_to_perform = if consumed_time > current_time {
            // We are ahead
            let time_ahead = consumed_time - current_time;
            let exact_steps_ahead = (time_ahead.as_millis() * ticks_per_second as u64) / 1_000;
            #[allow(clippy::bool_to_int_with_if)]
            if exact_steps_ahead > 4 { 0 } else { 1 }
        } else {
            // We are behind
            let time_debt = current_time - consumed_time;

            let exact_steps_needed = (time_debt.as_millis() * ticks_per_second as u64) / 1_000;

            // If we need significantly more than 1 step, consider doing 2
            if exact_steps_needed > 1 { 2 } else { 1 }
        };

        let stage_ids = {
            vec![
                TypeId::of::<FixedFirst>(),
                TypeId::of::<FixedPreUpdate>(),
                TypeId::of::<FixedUpdate>(),
                TypeId::of::<FixedPostUpdate>(),
            ]
        };

        let fixed_time_step_ms = 1000 / ticks_per_second;

        for _ in 0..steps_to_perform {
            for &stage_id in &stage_ids {
                stages
                    .get_by_id(&stage_id)
                    .expect("stage missing")
                    .run(state);
            }
            consumed_time += MillisDuration::from_millis(fixed_time_step_ms as u64);
        }

        {
            let fixed_scheduler_data = state.resources_mut().fetch_mut::<FixedSchedulerData>();
            fixed_scheduler_data.consumed_up_to_time = consumed_time;
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
            ticks_per_second: 60,
        });

        app.add_scheduler(MainScheduler);
        app.add_scheduler(FixedScheduler);
        app.add_scheduler(RenderScheduler);

        app.add_system(First, swap_messages);
    }
}
