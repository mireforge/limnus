/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod prelude;

use limnus_local_resource::{LocalResource, LocalResourceStorage};
use limnus_message::{Message, Messages};
use limnus_resource::{Resource, ResourceStorage};
use limnus_system::SystemParam;
use limnus_system_state::State;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

// Mutable resource access
pub struct ReM<'a, T: 'static> {
    value: &'a mut T,
}

impl<'a, T> ReM<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self { value }
    }
}

impl<'a, T> Deref for ReM<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for ReM<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

// Mutable resource access
pub struct Re<'a, T: 'static> {
    value: &'a T,
}

impl<'a, T> Re<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T> Deref for Re<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: Resource + 'static> SystemParam for ReM<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.resource_mut::<T>()?;
        let static_ref: &'static mut T = unsafe { transmute(actual_ref) };
        Some(ReM::new(static_ref))
    }
}

impl<T: Resource + 'static> SystemParam for Re<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.resource_mut::<T>()?;
        let static_ref: &'static mut T = unsafe { transmute(actual_ref) };
        Some(Re::new(static_ref))
    }
}

impl<T: 'static + Message> SystemParam for Msg<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.message::<T>()?;
        let static_ref: &'static Messages<T> = unsafe { transmute(actual_ref) };
        Some(Msg::new(static_ref))
    }
}

impl<T: 'static + Message> SystemParam for MsgM<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.message_mut::<T>()?;
        let static_ref: &'static mut Messages<T> = unsafe { transmute(actual_ref) };
        Some(MsgM::new(static_ref))
    }
}

pub struct ReAll<'a> {
    value: &'a mut ResourceStorage,
}
impl<'a> ReAll<'a> {
    pub fn new(value: &'a mut ResourceStorage) -> Self {
        Self { value }
    }
}

impl<'a> Deref for ReAll<'a> {
    type Target = ResourceStorage;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a> DerefMut for ReAll<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl SystemParam for ReAll<'static> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref: &mut ResourceStorage = world.resources_mut();
        let static_ref: &'static mut ResourceStorage = unsafe { transmute(actual_ref) };
        Some(ReAll::new(static_ref))
    }
}

// ====================

pub struct Msg<'a, T: 'static + Message> {
    value: &'a Messages<T>,
}

impl<'a, T: Message> Msg<'a, T> {
    #[must_use]
    pub const fn new(value: &'a Messages<T>) -> Self {
        Self { value }
    }
}

impl<'a, T: Message> Deref for Msg<'a, T> {
    type Target = Messages<T>;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

// Mutable message access
pub struct MsgM<'a, T: 'static + Message> {
    value: &'a mut Messages<T>,
}

impl<'a, T: Message> MsgM<'a, T> {
    pub fn new(value: &'a mut Messages<T>) -> Self {
        Self { value }
    }
}

impl<'a, T: Message> Deref for MsgM<'a, T> {
    type Target = Messages<T>;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: Message> DerefMut for MsgM<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

// ==========  Local resources

pub struct LocReAll<'a> {
    value: &'a mut LocalResourceStorage,
}
impl<'a> LocReAll<'a> {
    pub fn new(value: &'a mut LocalResourceStorage) -> Self {
        Self { value }
    }
}

impl<'a> Deref for LocReAll<'a> {
    type Target = LocalResourceStorage;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a> DerefMut for LocReAll<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl SystemParam for LocReAll<'static> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref: &mut LocalResourceStorage = world.local_resources_mut();
        let static_ref: &'static mut LocalResourceStorage = unsafe { transmute(actual_ref) };
        Some(LocReAll::new(static_ref))
    }
}

// === Local Resources

// Mutable local resource access
pub struct LoReM<'a, T: 'static> {
    value: &'a mut T,
}

impl<'a, T> crate::LoReM<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self { value }
    }
}

impl<'a, T> Deref for crate::LoReM<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for crate::LoReM<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<T: LocalResource + 'static> SystemParam for crate::LoReM<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.local_resource_mut::<T>()?;
        let static_ref: &'static mut T = unsafe { transmute(actual_ref) };
        Some(LoReM::new(static_ref))
    }
}

// Mutable local resource access
pub struct LoRe<'a, T: 'static> {
    value: &'a T,
}

impl<'a, T> crate::LoRe<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T> Deref for crate::LoRe<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: LocalResource + 'static> SystemParam for crate::LoRe<'static, T> {
    type Item = Self;

    fn get(world: &mut State) -> Option<Self::Item> {
        let actual_ref = world.local_resource_mut::<T>()?;
        let static_ref: &'static mut T = unsafe { transmute(actual_ref) };
        Some(LoRe::new(static_ref))
    }
}
