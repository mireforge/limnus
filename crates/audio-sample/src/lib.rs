/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_app::prelude::{App, Plugin};
use limnus_asset_registry::AssetRegistry;
use limnus_assets::Assets;
use limnus_assets::prelude::{AssetName, RawWeakId};
use limnus_assets_loader::{
    AssetLoader, ConversionError, ResourceStorage, WrappedAssetLoaderRegistry,
};
use limnus_audio_mixer::StereoSample;
use limnus_local_resource::LocalResourceStorage;
use tracing::debug;

pub fn load_wav(payload: &[u8]) -> StereoSample {
    let mut reader = hound::WavReader::new(payload).expect("could not convert wav file");
    let hound::WavSpec {
        sample_rate: source_sample_rate,
        sample_format,
        bits_per_sample,
        channels,
        ..
    } = reader.spec();
    //let length_samples = reader.duration();
    //let length_seconds = length_samples as f32 / source_sample_rate as f32;

    assert_eq!(channels, 2);

    // Force it to f32 for now
    let samples_result: Result<Vec<f32>, _> = match sample_format {
        hound::SampleFormat::Int => {
            let max_value = 2_u32.pow(bits_per_sample as u32 - 1) - 1;
            reader
                .samples::<i32>()
                .map(|sample| sample.map(|sample| sample as f32 / max_value as f32))
                .collect()
        }
        hound::SampleFormat::Float => reader.samples::<f32>().collect(),
    };
    let mut samples = samples_result.unwrap();

    let samples_stereo = oddio::frame_stereo(&mut samples);
    let sound_frames = oddio::Frames::from_slice(source_sample_rate, samples_stereo);

    StereoSample {
        stereo_frames: sound_frames,
    }
}

pub struct AudioSamplePlugin;

impl Plugin for AudioSamplePlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.resource_mut::<WrappedAssetLoaderRegistry>();
            let loader = StereoSampleConverter::new();

            registry.value.lock().unwrap().register_loader(loader);
        }
        app.insert_resource(Assets::<StereoSample>::default());
    }
}

#[derive(Default)]
pub struct StereoSampleConverter;

impl StereoSampleConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl AssetLoader for StereoSampleConverter {
    type AssetType = StereoSample;

    fn convert_and_insert(
        &self,
        id: RawWeakId,
        octets: &[u8],
        resources: &mut ResourceStorage,
        _local_resources: &mut LocalResourceStorage,
    ) -> Result<(), ConversionError> {
        let name: AssetName;
        {
            let asset_container = resources.fetch::<AssetRegistry>();
            name = asset_container
                .name_raw(id)
                .expect("should know about this Id");
        }

        debug!("convert from wav {name}");
        let stereo_sample = load_wav(octets);

        debug!("converted wav {name}");
        let stereo_sample_assets = resources.fetch_mut::<Assets<StereoSample>>();

        stereo_sample_assets.set_raw(id, stereo_sample);

        Ok(())
    }
}
