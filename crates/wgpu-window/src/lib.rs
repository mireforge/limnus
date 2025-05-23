/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_app::prelude::{App, Plugin};
use limnus_default_stages::RenderFirst;
use limnus_local_resource::prelude::LocalResource;
use limnus_screen::WindowMessage;
use limnus_system_params::{LoReM, Msg};
use std::default::Default;
use std::sync::Arc;
use tracing::{debug, info, trace};
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    InstanceFlags, Limits, MemoryHints, Queue, RequestAdapterOptions, RequestDeviceError, Surface,
    SurfaceConfiguration, SurfaceError,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug, LocalResource)]
pub struct WgpuWindow {
    surface: Arc<Surface<'static>>,
    device: Arc<Device>,
    queue: Arc<Queue>,

    config: SurfaceConfiguration,
}

impl WgpuWindow {
    #[must_use]
    pub const fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }
}

pub struct ReceiveAnnoyingAsync {
    pub device_info: Option<BasicDeviceInfo>,
}

#[derive(Debug, LocalResource)]
pub struct BasicDeviceInfo {
    pub adapter: Adapter,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Arc<Surface<'static>>,
    pub physical_surface_size: PhysicalSize<u32>,
}

pub async fn annoying_async_device_creation(
    window: Arc<Window>,
) -> Result<BasicDeviceInfo, RequestDeviceError> {
    let instance = Instance::new(InstanceDescriptor {
        flags: InstanceFlags::advanced_debugging(),
        dx12_shader_compiler: Default::default(),
        #[cfg(not(target_arch = "wasm32"))]
        backends: Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: Backends::GL, // TODO: Default to WebGl for compatibility for now, but maybe can change that in the future
        gles_minor_version: Default::default(),
    });
    trace!(?instance, "found instance");

    let surface = instance.create_surface(Arc::clone(&window)).unwrap();
    trace!(?surface, "surface");

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: Default::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    trace!(?adapter, "found adapter");

    let device_descriptor = DeviceDescriptor {
        label: None,
        required_features: Features::empty(), // Specify features as needed
        required_limits: if cfg!(target_arch = "wasm32") {
            Limits::downlevel_webgl2_defaults() // TODO: Not sure if this is needed?
        } else {
            Limits::default()
        },
        memory_hints: MemoryHints::default(), // Use default memory hints
    };

    info!(?device_descriptor, "device descriptor");

    let (device, queue) = adapter
        .request_device(&device_descriptor, None)
        .await
        .expect("Failed to request device");
    info!(?device, "device");

    let inner_size = window.inner_size();

    info!(?inner_size, "inner size");

    Ok(BasicDeviceInfo {
        adapter,
        device: device.into(),
        queue: queue.into(),
        surface: surface.into(),
        physical_surface_size: inner_size,
    })
}

fn tick(mut wgpu_window: LoReM<WgpuWindow>, window_messages: Msg<WindowMessage>) {
    for msg in window_messages.iter_previous() {
        if let WindowMessage::Resized(size) = msg {
            debug!("resized to {:?}", size);
            wgpu_window.resize((size.x, size.y));
        }
    }
}

pub struct WgpuWindowPlugin;
impl Plugin for WgpuWindowPlugin {
    fn build(&self, _app: &mut App) {}

    fn post_initialization(&self, app: &mut App) {
        app.insert_local_resource(WgpuWindow::new(
            app.local_resources().fetch::<BasicDeviceInfo>(),
        ));
        app.add_system(RenderFirst, tick);
        info!("wgpu window plugin is done");
    }
}

impl WgpuWindow {
    #[must_use]
    pub fn new(info: &BasicDeviceInfo) -> Self {
        let config = Self::configure_render_surface(info);

        Self {
            device: Arc::clone(&info.device),
            config,
            queue: Arc::clone(&info.queue),
            surface: Arc::clone(&info.surface),
        }
    }

    #[must_use]
    pub const fn device(&self) -> &Arc<Device> {
        &self.device
    }

    fn configure_render_surface(info: &BasicDeviceInfo) -> SurfaceConfiguration {
        let surface_caps = info.surface.get_capabilities(&info.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: info.physical_surface_size.width,
            height: info.physical_surface_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        info.surface.configure(&info.device, &config);

        let present_mode = surface_caps.present_modes[0];
        let alpha_mode = surface_caps.alpha_modes[0];
        trace!(
            "found surface format {:?} {:?} {:?}",
            surface_format, present_mode, alpha_mode
        );

        config
    }

    #[must_use]
    pub const fn texture_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    pub fn resize(&mut self, new_size: (u16, u16)) {
        let width = new_size.0 as usize;
        let height = new_size.1 as usize;

        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width as u32;
        self.config.height = height as u32;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(
        &self,
        mut render_fn: impl FnMut(&mut wgpu::CommandEncoder, &wgpu::TextureView),
    ) -> Result<(), SurfaceError> {
        // Gets a new texture from the swap chain
        let surface_texture = self.surface.get_current_texture()?;

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        render_fn(&mut encoder, &texture_view);

        self.queue.submit(std::iter::once(encoder.finish()));

        surface_texture.present();

        Ok(())
    }
}
