/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, StreamConfig};
use std::fmt::Debug;
use std::io;

use tracing::{debug, error, info, trace};

use limnus_local_resource::prelude::*;
const MIN_SAMPLE_RATE: u32 = 44100;
const MAX_SAMPLE_RATE: u32 = 48000;

#[derive(LocalResource)]
pub struct Audio {
    #[allow(dead_code)]
    device: Device,
    config: StreamConfig,
    sample_rate: u32,
}

impl Debug for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Audio")
    }
}

#[allow(unused)]
fn debug_output(host: Host) {
    for device in host.devices().expect("should have a device") {
        info!(
            "Found device: {:?}",
            device.name().unwrap_or("unknown".to_string())
        );

        let configs = device.supported_output_configs();
        if configs.is_err() {
            continue;
        }

        for config in configs.unwrap() {
            info!(
                "  Channels: {}, Sample Rate: {} - {} Hz, Sample Format: {:?}",
                config.channels(),
                config.min_sample_rate().0,
                config.max_sample_rate().0,
                config.sample_format()
            );
        }
    }
}

impl Audio {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();

        let default_device = host.default_output_device();
        if default_device.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no ",
            )));
        }
        let device = default_device.unwrap();
        let device_name = device.name().unwrap_or("unknown".parse().unwrap());
        debug!(device = device_name, "default output device");

        let all_supported_configs = device.supported_output_configs()?.collect::<Vec<_>>();

        for config in all_supported_configs {
            debug!("Supported config: {:?}", config);
        }

        let maybe_supported_config = device
            .supported_output_configs()?
            .find(|config| {
                config.min_sample_rate().0 <= MAX_SAMPLE_RATE
                    && config.max_sample_rate().0 >= MIN_SAMPLE_RATE
            })
            .into_iter()
            .min_by_key(|config| config.max_sample_rate().0);

        if maybe_supported_config.is_none() {
            error!("No supported output configurations with with an accepted output_config.");
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "no supported output configurations found",
            )));
        }

        let supported_config = maybe_supported_config.unwrap();

        let sample_rate = supported_config.min_sample_rate().0;
        let supported_config = supported_config.with_sample_rate(cpal::SampleRate(sample_rate));

        trace!(config=?supported_config, "Selected output config");

        let config: StreamConfig = supported_config.into();

        info!(device=device_name, sample_rate, config=?&config, "selected device and configuration");

        //let scene = Arc::new(oddio::SpatialScene);

        Ok(Self {
            device,
            config,
            sample_rate,
        })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
