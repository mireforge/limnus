/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use crate::{AssetName, RawWeakId};
use message_channel::Sender;
use std::fmt::{Debug, Display};
use tracing::info;

// Event sent when owner is dropped
#[derive(Debug)]
pub struct DropMessage {
    pub asset_id: RawWeakId,
}

// NOTE: Do NOT implement copy or clone for AssetOwner
pub struct AssetOwner {
    id: RawWeakId,
    #[allow(unused)]
    asset_name: Option<AssetName>,
    drop_channel: Sender<DropMessage>,
}

impl Debug for AssetOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let asset_name = self
            .asset_name
            .map_or("unknown".to_string(), |name| name.to_string());
        write!(f, "({:?} {})", self.id, asset_name)
    }
}

impl Display for AssetOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialEq<Self> for AssetOwner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.asset_name == other.asset_name
    }
}

impl PartialOrd<Self> for AssetOwner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for AssetOwner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for AssetOwner {}

impl Drop for AssetOwner {
    fn drop(&mut self) {
        info!(id=%self.id, "notice: asset owner is dropped");
        let _ = self.drop_channel.send(DropMessage { asset_id: self.id });
    }
}

impl AssetOwner {
    #[must_use]
    pub const fn new(
        id: RawWeakId,
        asset_name: Option<AssetName>,
        drop_channel: Sender<DropMessage>,
    ) -> Self {
        Self {
            id,
            asset_name,
            drop_channel,
        }
    }

    #[must_use]
    pub const fn raw_id(&self) -> RawWeakId {
        self.id
    }

    #[must_use]
    pub const fn asset_name(&self) -> Option<AssetName> {
        self.asset_name
    }
}
