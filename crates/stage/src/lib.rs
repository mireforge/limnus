/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_local_resource::prelude::LocalResource;
use limnus_system::{IntoSystem, System, SystemParam};
use limnus_system_state::State;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub trait StageTag: 'static {}

#[derive(Default, Debug)]
pub struct Stages {
    pub stages: HashMap<TypeId, Stage>,
}

impl Stages {}

impl Stages {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stages: HashMap::default(),
        }
    }
    pub fn add<S>(&mut self, _stage_tag: &S, stage: Stage)
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.insert(stage_id, stage);
    }

    pub fn get_mut<S>(&mut self, stage_tag: &S) -> Option<&mut Stage>
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.get_mut(&stage_id)
    }

    pub fn get<S>(&self, stage_tag: &S) -> Option<&Stage>
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.get(&stage_id)
    }

    #[inline]
    #[must_use]
    pub fn get_by_id(&self, stage_id: &TypeId) -> Option<&Stage> {
        self.stages.get(stage_id)
    }
}

pub struct Stage {
    systems: Vec<Box<dyn System>>,
}
impl Debug for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stage")
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::new()
    }
}

impl Stage {
    #[must_use]
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    pub fn add_system<F, Params>(&mut self, function: F)
    where
        F: IntoSystem<Params>,
        Params: SystemParam,
    {
        self.systems.push(Box::new(function.into_system()));
    }

    pub fn run(&self, state: &mut State) {
        for system in &self.systems {
            system.run(state);
        }
    }
}
