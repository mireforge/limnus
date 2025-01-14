use limnus_stage::Stages;
use limnus_system_state::State;
use std::fmt::Debug;

pub trait Scheduler: Debug + 'static {
    fn schedule(&self, stages: &Stages, state: &mut State);
}
