/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use limnus::prelude::{
    AssetName, AssetRegistry, AssetRegistryPlugin, Assets, AudioDevicePlugin, AudioMixer,
    AudioMixerPlugin, AudioSamplePlugin, AudioStreamPlugin, Id, LocalResource, StereoSample,
};
use limnus_app::prelude::{App, AppReturnValue};
use limnus_assets_loader::AssetLoaderRegistryPlugin;
use limnus_loader::LoaderPlugin;
use limnus_log::LogPlugin;
use limnus_system_params::{LoReM, Re, ReM};
use limnus_system_runner::UpdatePhase;
use std::thread::sleep;
use std::time::Duration;

fn test_runner(mut app: App) -> AppReturnValue {
    loop {
        app.update();
        sleep(Duration::from_millis(32));
    }
    // AppReturnValue::Value(0)
}

#[derive(Debug, LocalResource)]
pub struct AudioTesterState {
    pub stereo_sample: Option<Id<StereoSample>>,
    pub counter: u32,
}

fn tick(
    mut mixer: LoReM<AudioMixer>,
    stereo_samples: Re<Assets<StereoSample>>,
    mut asset_loader: ReM<AssetRegistry>,
    mut state: LoReM<AudioTesterState>,
) {
    if state.stereo_sample.is_none() {
        state.stereo_sample = Some(
            asset_loader
                .load::<StereoSample>(AssetName::new("qubodup_whoosh").with_extension("wav")),
        );
    }

    state.counter += 1;
    if state.counter % 30 == 0 {
        if let Some(found_sample_id) = &state.stereo_sample {
            if let Some(found_stereo_sample) = stereo_samples.get(found_sample_id) {
                mixer.play(found_stereo_sample);
            }
        }
    }
}

fn main() {
    println!("Audio Tester");

    let mut app = App::new();

    app.add_plugins((
        LogPlugin,
        LoaderPlugin,
        AssetLoaderRegistryPlugin,
        AssetRegistryPlugin,
    ));

    app.add_plugins((
        AudioDevicePlugin,
        AudioSamplePlugin,
        AudioMixerPlugin,
        AudioStreamPlugin,
    ));

    app.add_system(UpdatePhase::Update, tick);

    app.insert_local_resource(AudioTesterState {
        stereo_sample: Default::default(),
        counter: 0,
    });

    app.set_runner(test_runner);

    app.run();
}
