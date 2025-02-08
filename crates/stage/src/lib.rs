/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_system::{IntoSystem, System, SystemParam};
use limnus_system_state::State;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

/// A marker trait used to uniquely identify stages.
///
/// Implement this trait for any type you wish to use as a stage identifier.
pub trait StageTag: 'static {}

/// Manages multiple stages, each identified by a unique `StageTag`.
///
/// The `Stages` struct provides methods to add, retrieve, and modify stages.
#[derive(Default, Debug)]
pub struct Stages {
    pub stages: HashMap<TypeId, Stage>,
}

impl Stages {
    /// Creates a new, empty `Stages` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::Stages;
    ///
    /// let stages = Stages::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            stages: HashMap::default(),
        }
    }

    /// Adds a stage to the `Stages` collection with the specified tag.
    ///
    /// # Type Parameters
    ///
    /// - `S`: A type that implements the `StageTag` trait, used to uniquely identify the stage.
    ///
    /// # Parameters
    ///
    /// - `stage`: The `Stage` instance to be added.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::{Stages, Stage, StageTag};
    ///
    /// struct UpdateStage;
    /// impl StageTag for UpdateStage {}
    ///
    /// let mut stages = Stages::new();
    /// let stage = Stage::new();
    /// stages.add::<UpdateStage>(stage);
    /// ```
    pub fn add<S>(&mut self, stage: Stage)
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.insert(stage_id, stage);
    }

    /// Retrieves a mutable reference to a stage identified by the specified tag.
    ///
    /// # Type Parameters
    ///
    /// - `S`: A type that implements the `StageTag` trait, used to uniquely identify the stage.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the `Stage` if it exists, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::{Stages, Stage, StageTag};
    ///
    /// struct UpdateStage;
    /// impl StageTag for UpdateStage {}
    ///
    /// let mut stages = Stages::new();
    /// let stage = Stage::new();
    /// stages.add::<UpdateStage>(stage);
    ///
    /// if let Some(stage) = stages.get_mut::<UpdateStage>() {
    ///     // Modify the stage
    /// }
    /// ```
    #[must_use]
    pub fn get_mut<S>(&mut self) -> Option<&mut Stage>
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.get_mut(&stage_id)
    }

    /// Retrieves an immutable reference to a stage identified by the specified tag.
    ///
    /// # Type Parameters
    ///
    /// - `S`: A type that implements the `StageTag` trait, used to uniquely identify the stage.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Stage` if it exists, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::{Stages, Stage, StageTag};
    ///
    /// struct UpdateStage;
    /// impl StageTag for UpdateStage {}
    ///
    /// let mut stages = Stages::new();
    /// let stage = Stage::new();
    /// stages.add::<UpdateStage>(stage);
    ///
    /// if let Some(stage) = stages.get::<UpdateStage>() {
    ///     // Access the stage
    /// }
    /// ```
    #[must_use]
    pub fn get<S>(&self) -> Option<&Stage>
    where
        S: StageTag,
    {
        let stage_id = TypeId::of::<S>();
        self.stages.get(&stage_id)
    }

    /// Retrieves an immutable reference to a stage by its `TypeId`.
    ///
    /// # Parameters
    ///
    /// - `stage_id`: A reference to the `TypeId` of the stage to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Stage` if it exists, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::{Stages, Stage};
    /// use std::any::TypeId;
    ///
    /// pub struct MyStageTag;
    ///
    /// let stages = Stages::new();
    /// let stage_id = TypeId::of::<MyStageTag>();
    /// if let Some(stage) = stages.get_by_id(&stage_id) {
    ///     // Access the stage
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_by_id(&self, stage_id: &TypeId) -> Option<&Stage> {
        self.stages.get(stage_id)
    }
}

/// Represents a single stage containing a collection of systems.
///
/// A `Stage` can have multiple systems that execute with access to shared state.
pub struct Stage {
    systems: Vec<Box<dyn System>>,
}

impl Debug for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stage")
    }
}

impl Default for Stage {
    /// Creates a new `Stage` instance with no systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::Stage;
    ///
    /// let stage = Stage::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Stage {
    /// Creates a new, empty `Stage` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::Stage;
    ///
    /// let stage = Stage::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    /// Adds a system to the stage.
    ///
    /// Systems are the core units of logic that run within a stage. They are executed in the order they are added.
    ///
    /// # Type Parameters
    ///
    /// - `F`: A function or closure that can be converted into a `System`.
    /// - `Params`: Parameters required by the system, implementing the `SystemParam` trait.
    ///
    /// # Parameters
    ///
    /// - `function`: The system function or closure to add.
    ///
    pub fn add_system<F, Params>(&mut self, function: F)
    where
        F: IntoSystem<Params>,
        Params: SystemParam,
    {
        self.systems.push(Box::new(function.into_system()));
    }

    /// Executes all systems within the stage, providing mutable access to the shared `State`.
    ///
    /// Systems are run in the order they were added to the stage.
    ///
    /// # Parameters
    ///
    /// - `state`: A mutable reference to the shared `State` that systems can modify.
    ///
    /// # Examples
    ///
    /// ```
    /// use limnus_stage::Stage;
    /// use limnus_system_state::State;
    ///
    /// let stage = Stage::new();
    /// let mut state = State::default();
    /// stage.run(&mut state);
    /// ```
    pub fn run(&self, state: &mut State) {
        for system in &self.systems {
            system.run(state);
        }
    }
}
