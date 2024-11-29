/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod prelude;

use limnus_app::prelude::{App, AppReturnValue, Plugin};
use limnus_asset_registry::AssetRegistryPlugin;
use limnus_assets_loader::AssetLoaderRegistryPlugin;
use limnus_loader::LoaderPlugin;
use limnus_log::LogPlugin;
use limnus_wgpu_window::WgpuWindowPlugin;

#[cfg(feature = "audio")]
use limnus_audio_device::AudioDevicePlugin;
#[cfg(feature = "audio")]
use limnus_audio_mixer::AudioMixerPlugin;
#[cfg(feature = "audio")]
use limnus_audio_sample::AudioSamplePlugin;
#[cfg(feature = "audio")]
use limnus_audio_stream::AudioStreamPlugin;
use limnus_gamepad::GamepadResourcePlugin;
use limnus_gamepad_gilrs::GamepadGilrsPlugin;

pub struct Main;

impl Main {
    pub fn run() -> AppReturnValue {
        App::new().add_plugins(DefaultPlugins).run()
    }
}

pub struct WindowRunnerPlugin;

impl Plugin for WindowRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(limnus_window_runner::runner);
    }
}

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LogPlugin,
            LoaderPlugin,
            AssetLoaderRegistryPlugin,
            AssetRegistryPlugin,
        ));
        app.add_plugins((WindowRunnerPlugin, WgpuWindowPlugin));

        #[cfg(feature = "audio")]
        app.add_plugins((
            AudioDevicePlugin,
            AudioSamplePlugin,
            AudioMixerPlugin,
            AudioStreamPlugin,
        ));

        app.add_plugins((GamepadResourcePlugin, GamepadGilrsPlugin));
    }
}
