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

const PREFERRED_SAMPLE_RATE: u32 = 44100;
const MAX_SUPPORTED_RATE: u32 = 48000;
const REQUIRED_CHANNELS: u16 = 2;

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

        for config in &all_supported_configs {
            debug!("Supported config: {:?}", config);
        }

        let supported_configs: Vec<_> = all_supported_configs
            .into_iter()
            .filter(|config| {
                matches!(
                    config.sample_format(),
                    cpal::SampleFormat::I16 | cpal::SampleFormat::F32
                ) && config.channels() == REQUIRED_CHANNELS
                    && config.min_sample_rate().0 <= MAX_SUPPORTED_RATE
                    && config.max_sample_rate().0 >= PREFERRED_SAMPLE_RATE
            })
            .collect();

        for config in &supported_configs {
            debug!(
                "Valid config - Format: {:?}, Channels: {}, Rate range: {} - {}",
                config.sample_format(),
                config.channels(),
                config.min_sample_rate().0,
                config.max_sample_rate().0
            );
        }

        let supported_config = supported_configs
            .into_iter()
            .min_by_key(|config| {
                let format_priority = match config.sample_format() {
                    cpal::SampleFormat::I16 => 0,
                    _ => 1,
                };
                (format_priority, config.max_sample_rate().0)
            })
            .ok_or_else(|| {
                error!("No supported output configurations with stereo I16/F32 format found");
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "no supported stereo output configurations found",
                )
            })?;

        // Always try to use 44.1kHz unless it's not supported
        let sample_rate = if supported_config.min_sample_rate().0 <= PREFERRED_SAMPLE_RATE
            && supported_config.max_sample_rate().0 >= PREFERRED_SAMPLE_RATE
        {
            PREFERRED_SAMPLE_RATE // Use 44.1kHz if supported
        } else {
            MAX_SUPPORTED_RATE // Otherwise use 48kHz
        };

        let supported_config = supported_config.with_sample_rate(cpal::SampleRate(sample_rate));

        trace!(config=?supported_config, "Selected output config");

        let config: StreamConfig = supported_config.into();

        info!(device=device_name, sample_rate, config=?&config, "selected device and configuration");

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
