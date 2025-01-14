/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_scheduler::Scheduler;
use limnus_stage::Stages;
use limnus_system_state::State;

#[derive(Default)]
pub struct Runner {
    schedulers: Vec<Box<dyn Scheduler>>,
}

impl Runner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            schedulers: Vec::new(),
        }
    }

    pub fn add_scheduler<T>(&mut self, schedule: T)
    where
        T: Scheduler,
    {
        self.schedulers.push(Box::new(schedule));
    }

    pub fn run_schedulers(&self, stages: &Stages, state: &mut State) {
        for scheduler in &self.schedulers {
            scheduler.schedule(stages, state);
        }
    }
}
