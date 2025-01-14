/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_local_resource::{LocalResource, LocalResourceStorage};
use limnus_message::{Message, MessageId, MessageStorage, Messages, MessagesIterator};
use limnus_resource::prelude::*;
use limnus_scheduler::Scheduler;
use limnus_scheduler_runner::Runner;
use limnus_stage::{Stage, StageTag, Stages};
use limnus_system::{IntoSystem, SystemParam};
use limnus_system_state::State;
use std::any::type_name;
use tracing::{debug, info};

type AppRunner = dyn FnOnce(App) -> AppReturnValue;

pub enum AppPhase {
    WaitingForPlugins,
    Running,
}

pub struct App {
    app_runner: Option<Box<AppRunner>>,
    schedulers_runner: Runner,
    plugins: Vec<Box<dyn Plugin>>,
    state: State,
    phase: AppPhase,
    stages: Stages,
}

impl App {
    pub(crate) fn internal_add_plugin(&mut self, boxed_plugin: Box<dyn Plugin>) {
        boxed_plugin.build(self);
        debug!(plugin=?boxed_plugin, "Added");
        self.plugins.push(boxed_plugin);
    }

    pub fn update(&mut self) {
        if matches!(self.phase, AppPhase::WaitingForPlugins) {
            let mut all_are_ready = true;

            for plugin in &self.plugins {
                if !plugin.is_initialized(self) {
                    info!("...waiting for {plugin:?}");
                    all_are_ready = false;
                }
            }

            if !all_are_ready {
                return;
            }
            debug!("all plugins are ready, starting post initialization");
            let mut plugins = std::mem::take(&mut self.plugins); // Temporarily take ownership of the plugins
            for plugin in &mut plugins {
                plugin.post_initialization(self);
            }

            info!("...post initialization complete. start running systems in schedules!");

            self.plugins = plugins;

            self.phase = AppPhase::Running;
        }

        self.schedulers_runner
            .run_schedulers(&self.stages, &mut self.state);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum AppReturnValue {
    Value(u32),
}

#[derive(Resource, Debug)]
pub struct ApplicationExit {
    pub value: AppReturnValue,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            app_runner: None,
            state: State::new(),
            plugins: Vec::default(),
            phase: AppPhase::WaitingForPlugins,
            schedulers_runner: Runner::new(),
            stages: Stages::new(),
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            app_runner: None,
            state: State::new(),
            plugins: Vec::default(),
            phase: AppPhase::WaitingForPlugins,
            schedulers_runner: Runner::new(),
            stages: Stages::new(),
        }
    }

    /// Consume self and start the runner function. It is not certain that it will ever return.
    pub fn run(&mut self) -> AppReturnValue {
        // Replace the runner with Dummy to take ownership
        let runner = self.app_runner.take();

        // Replace self with an empty App to take ownership and get the current self returned.
        let app = core::mem::replace(self, Self::empty());

        runner.unwrap()(app)
    }

    pub fn add_plugins<P: PluginCollection>(&mut self, collection: P) -> &mut Self {
        collection.attach_to_app(self);
        self
    }

    pub fn add_stage<S>(&mut self)
    where
        S: StageTag,
    {
        let stage = Stage::new();
        self.stages.add::<S>(stage);
    }

    /// # Panics
    /// a `Stage` for the type parameter `S` must exist
    pub fn add_system<F, Params, S>(&mut self, _stage_tag: S, system: F)
    where
        F: IntoSystem<Params>,
        Params: SystemParam,
        S: StageTag,
    {
        self.stages
            .get_mut::<S>()
            .expect("could not find stage")
            .add_system(system);
    }

    pub fn add_scheduler<T>(&mut self, scheduler: T)
    where
        T: Scheduler,
    {
        self.schedulers_runner.add_scheduler(scheduler);
    }

    /// The function supplied by `app_runner` can in some scenarios never return.
    pub fn set_runner(
        &mut self,
        app_runner: impl FnOnce(Self) -> AppReturnValue + 'static,
    ) -> &mut Self {
        self.app_runner = Some(Box::new(app_runner));
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) -> &mut Self {
        debug!(resource_type=type_name::<R>(), value=?value, "inserting resource");
        self.state.resources_mut().insert(value);
        self
    }

    pub fn insert_local_resource<R: LocalResource>(&mut self, value: R) -> &mut Self {
        debug!(resource_type=type_name::<R>(), value=?value, "inserting local resource");
        self.state.local_resources_mut().insert(value);
        self
    }

    #[inline]
    pub fn resource_take<R: Resource>(&mut self) -> R {
        self.state.resources_mut().remove::<R>().unwrap()
    }

    #[inline]
    #[must_use]
    pub fn get_resource_ref<R: Resource>(&self) -> Option<&R> {
        self.state.resources().get::<R>()
    }

    #[inline]
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.state.resources_mut().get_mut::<R>()
    }

    #[inline]
    #[must_use]
    pub fn resource<R: Resource>(&self) -> &R {
        self.state.resources().fetch::<R>()
    }

    #[inline]
    pub fn resource_mut<R: Resource>(&mut self) -> &mut R {
        self.state.resources_mut().fetch_mut::<R>()
    }

    #[must_use]
    pub const fn resources(&self) -> &ResourceStorage {
        self.state.resources()
    }

    pub fn resources_mut(&mut self) -> &mut ResourceStorage {
        self.state.resources_mut()
    }

    #[must_use]
    pub const fn local_resources(&self) -> &LocalResourceStorage {
        self.state.local_resources()
    }

    #[inline]
    #[must_use]
    pub fn has_resource<R: Resource>(&self) -> bool {
        self.state.resources().contains::<R>()
    }

    pub fn create_message_type<M: Message>(&mut self) {
        debug!(channel_type = type_name::<M>(), "creating message queue");
        self.state.messages_mut().register_message_type::<M>();
    }

    #[must_use]
    pub fn get_messages<M: Message>(&self) -> Option<&Messages<M>> {
        self.state.messages().get::<M>()
    }

    pub fn send<M: Message>(&mut self, message: M) -> MessageId<M> {
        self.state
            .messages_mut()
            .get_mut::<M>()
            .unwrap()
            .send(message)
    }

    #[must_use]
    pub const fn messages(&self) -> &MessageStorage {
        self.state.messages()
    }

    pub fn messages_mut(&mut self) -> &mut MessageStorage {
        self.state.messages_mut()
    }

    #[must_use]
    pub fn iter_current<M: Message>(&self) -> MessagesIterator<M> {
        self.state.messages().get::<M>().unwrap().iter_current()
    }

    #[must_use]
    pub fn iter_previous<M: Message>(&self) -> MessagesIterator<M> {
        self.state.messages().get::<M>().unwrap().iter_previous()
    }
}

pub trait PluginCollection {
    fn attach_to_app(self, app: &mut App);
}

impl<T: Plugin> PluginCollection for T {
    fn attach_to_app(self, app: &mut App) {
        let boxed = Box::new(self);
        app.internal_add_plugin(boxed);
    }
}

impl<T1: Plugin, T2: Plugin> PluginCollection for (T1, T2) {
    fn attach_to_app(self, app: &mut App) {
        let boxed_plugin1 = Box::new(self.0);
        let boxed_plugin2 = Box::new(self.1);

        app.internal_add_plugin(boxed_plugin1);
        app.internal_add_plugin(boxed_plugin2);
    }
}

impl<T1: Plugin, T2: Plugin, T3: Plugin> PluginCollection for (T1, T2, T3) {
    fn attach_to_app(self, app: &mut App) {
        let boxed_plugin1 = Box::new(self.0);
        let boxed_plugin2 = Box::new(self.1);
        let boxed_plugin3 = Box::new(self.2);

        app.internal_add_plugin(boxed_plugin1);
        app.internal_add_plugin(boxed_plugin2);
        app.internal_add_plugin(boxed_plugin3);
    }
}

impl<T1: Plugin, T2: Plugin, T3: Plugin, T4: Plugin> PluginCollection for (T1, T2, T3, T4) {
    fn attach_to_app(self, app: &mut App) {
        let boxed_plugin1 = Box::new(self.0);
        let boxed_plugin2 = Box::new(self.1);
        let boxed_plugin3 = Box::new(self.2);
        let boxed_plugin4 = Box::new(self.3);

        app.internal_add_plugin(boxed_plugin1);
        app.internal_add_plugin(boxed_plugin2);
        app.internal_add_plugin(boxed_plugin3);
        app.internal_add_plugin(boxed_plugin4);
    }
}

impl<T1: Plugin, T2: Plugin, T3: Plugin, T4: Plugin, T5: Plugin> PluginCollection
    for (T1, T2, T3, T4, T5)
{
    fn attach_to_app(self, app: &mut App) {
        let boxed_plugin1 = Box::new(self.0);
        let boxed_plugin2 = Box::new(self.1);
        let boxed_plugin3 = Box::new(self.2);
        let boxed_plugin4 = Box::new(self.3);
        let boxed_plugin5 = Box::new(self.4);

        app.internal_add_plugin(boxed_plugin1);
        app.internal_add_plugin(boxed_plugin2);
        app.internal_add_plugin(boxed_plugin3);
        app.internal_add_plugin(boxed_plugin4);
        app.internal_add_plugin(boxed_plugin5);
    }
}

/// Plugins are not allowed to mutate themselves, just reference the app
pub trait Plugin: 'static {
    // Send + Sync +
    fn build(&self, _app: &mut App) {}

    fn is_initialized(&self, _app: &App) -> bool {
        true
    }

    fn post_initialization(&self, _app: &mut App) {}

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl std::fmt::Debug for dyn Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name())
    }
}
