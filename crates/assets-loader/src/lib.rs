/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_app::prelude::{App, Plugin};
use limnus_assets::prelude::{Asset, RawWeakId};
use limnus_local_resource::LocalResourceStorage;
pub use limnus_resource::ResourceStorage;
use limnus_resource::prelude::Resource;
use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::io::Error;
use std::sync::{Arc, Mutex};
use tracing::debug;

#[derive(Debug)]
pub enum LoadError {
    MissingLoader(RawWeakId),
    ConversionError(ConversionError),
    Downcast,
}

#[derive(Debug)]
pub enum ConversionError {
    WrongFormat,
    IoError(io::Error),
}

impl From<ConversionError> for LoadError {
    fn from(err: ConversionError) -> Self {
        Self::ConversionError(err)
    }
}

impl From<io::Error> for ConversionError {
    fn from(value: Error) -> Self {
        Self::IoError(value)
    }
}

pub trait AssetLoader: Send + Sync {
    type AssetType: Asset + 'static;

    /// # Errors
    /// TODO: Add more here
    fn convert_and_insert(
        &self,
        id: RawWeakId,
        octets: &[u8],
        world: &mut ResourceStorage,
        local_resource: &mut LocalResourceStorage,
    ) -> Result<(), ConversionError>;
}

type TypeIdMap<T> = HashMap<TypeId, T>;

pub trait AnyAssetLoader: Send + Sync {
    /// # Errors
    /// TODO: Add more here
    fn convert_and_insert_erased(
        &self,
        id: RawWeakId,
        octets: &[u8],
        resources: &mut ResourceStorage,
        local_resource_storage: &mut LocalResourceStorage,
    ) -> Result<(), LoadError>;

    fn asset_type_id(&self) -> TypeId;
}

impl<T> AnyAssetLoader for T
where
    T: AssetLoader + 'static,
{
    fn convert_and_insert_erased(
        &self,
        id: RawWeakId,
        octets: &[u8],
        resources: &mut ResourceStorage,
        local_resource_storage: &mut LocalResourceStorage,
    ) -> Result<(), LoadError> {
        self.convert_and_insert(id, octets, resources, local_resource_storage)
            .map_err(LoadError::from)
    }

    fn asset_type_id(&self) -> TypeId {
        TypeId::of::<T::AssetType>()
    }
}

#[derive(Resource)]
pub struct WrappedAssetLoaderRegistry {
    pub value: Arc<Mutex<AssetLoaderRegistry>>,
}

impl Debug for WrappedAssetLoaderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WrappedAssetLoaderRegistry")
    }
}

#[derive(Default)]
pub struct AssetLoaderRegistry {
    loaders: TypeIdMap<Box<dyn AnyAssetLoader>>,
}

impl Debug for AssetLoaderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssetLoaderRegistry")
    }
}

impl AssetLoaderRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            loaders: TypeIdMap::new(),
        }
    }

    pub fn register_loader<T>(&mut self, loader: T)
    where
        T: AssetLoader + 'static,
    {
        debug!(
            asset_type = type_name::<T::AssetType>(),
            loader = type_name::<T>(),
            "registering asset loader",
        );
        self.loaders
            .insert(loader.asset_type_id(), Box::new(loader));
    }

    /// # Errors
    /// If missing or conversion failed
    pub fn convert_and_insert(
        &self,
        id: RawWeakId,
        octets: &[u8],
        resources: &mut ResourceStorage,
        local_resources: &mut LocalResourceStorage,
    ) -> Result<(), LoadError> {
        let loader = self
            .loaders
            .get(&id.type_id())
            .ok_or(LoadError::MissingLoader(id))?;

        loader.convert_and_insert_erased(id, octets, resources, local_resources)
    }
}

pub struct AssetLoaderRegistryPlugin;

impl Plugin for AssetLoaderRegistryPlugin {
    fn build(&self, app: &mut App) {
        let loader_registry = WrappedAssetLoaderRegistry {
            value: Arc::new(Mutex::new(AssetLoaderRegistry::new())),
        };
        app.insert_resource(loader_registry);
    }
}
