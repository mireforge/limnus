/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use limnus_stage::Stages;
use limnus_system_state::State;
use std::fmt::Debug;

pub trait Scheduler: Debug + 'static {
    fn schedule(&self, stages: &Stages, state: &mut State);
}
