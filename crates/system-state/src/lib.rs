/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_local_resource::{LocalResource, LocalResourceStorage};
use limnus_message::{Message, MessageStorage, Messages};
use limnus_resource::{Resource, ResourceStorage};

#[derive(Debug, Default)]
pub struct State {
    resources: ResourceStorage,
    local_resources: LocalResourceStorage,
    messages: MessageStorage,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            resources: ResourceStorage::new(),
            messages: MessageStorage::new(),
            local_resources: LocalResourceStorage::new(),
        }
    }

    #[must_use]
    pub const fn messages(&self) -> &MessageStorage {
        &self.messages
    }

    pub fn messages_mut(&mut self) -> &mut MessageStorage {
        &mut self.messages
    }

    #[must_use]
    pub const fn resources(&self) -> &ResourceStorage {
        &self.resources
    }

    #[must_use]
    pub fn resources_mut(&mut self) -> &mut ResourceStorage {
        &mut self.resources
    }

    #[must_use]
    pub fn local_resources_mut(&mut self) -> &mut LocalResourceStorage {
        &mut self.local_resources
    }

    #[must_use]
    pub const fn local_resources(&self) -> &LocalResourceStorage {
        &self.local_resources
    }

    #[inline]
    #[must_use]
    pub fn resource<R: Resource>(&self) -> &R {
        self.resources.fetch::<R>()
    }

    #[inline]
    pub fn resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    #[inline]
    #[must_use]
    pub fn local_resource<R: LocalResource>(&self) -> Option<&R> {
        self.local_resources.get::<R>()
    }

    #[inline]
    pub fn local_resource_mut<R: LocalResource>(&mut self) -> Option<&mut R> {
        self.local_resources.get_mut::<R>()
    }

    pub fn message_mut<M: Message>(&mut self) -> Option<&mut Messages<M>> {
        self.messages.get_mut::<M>()
    }

    #[must_use] pub fn message<M: Message>(&self) -> Option<&Messages<M>> {
        self.messages.get::<M>()
    }
}
