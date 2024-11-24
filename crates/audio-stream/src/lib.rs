/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, StreamConfig};
use limnus_app::prelude::{App, Plugin};
use limnus_audio_device::low_level::Audio;
use limnus_audio_mixer::AudioMixer;
use limnus_local_resource::prelude::LocalResource;
use oddio;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use tracing::error;

fn start_stream(
    device: &Device,
    config: &StreamConfig,
    sample_rate: u32,
    low_level_mixer: Arc<Mutex<oddio::Mixer<[f32; 2]>>>,
) -> cpal::Stream {
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let out_frames = oddio::frame_stereo(data);
                oddio::run(
                    low_level_mixer.lock().unwrap().deref_mut(),
                    sample_rate,
                    out_frames,
                );
            },
            move |err| {
                error!("Stream error: {}", err);
            },
            None,
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to start stream");
    stream
}

#[derive(LocalResource)]
pub struct AudioStream {
    #[allow(dead_code)]
    stream: cpal::Stream,
}

impl Debug for AudioStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AudioStream")
    }
}

pub struct AudioStreamPlugin;

impl Plugin for AudioStreamPlugin {
    fn build(&self, app: &mut App) {
        let resources = app.local_resources();
        let audio = resources.fetch::<Audio>();
        let mixer = resources.fetch::<AudioMixer>();

        let device = audio.device();
        let config = audio.config();
        let stream = start_stream(device, config, audio.sample_rate(), mixer.mixer.clone());
        let audio_stream = AudioStream { stream };
        app.insert_local_resource(audio_stream);
    }
}
