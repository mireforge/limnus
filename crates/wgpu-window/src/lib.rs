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
use tracing::{debug, info, trace, warn};
use wgpu::{
    Adapter, BackendOptions, Backends, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, InstanceFlags, Limits, MemoryBudgetThresholds, MemoryHints,
    PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError,
    Trace,
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

async fn try_pref(
    instance: &Instance,
    surface: Option<&Surface<'_>>,
    pref: PowerPreference,
) -> Option<wgpu::Adapter> {
    debug!(?pref, "trying to find adapter with power preference");
    let res = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: pref,
            compatible_surface: surface,
            force_fallback_adapter: false,
        })
        .await;

    match res {
        Ok(adapter) => Some(adapter),
        Err(err) => {
            warn!("request_adapter({pref:?}) failed: {err}");
            None
        }
    }
}

/// # Errors
///
pub async fn pick_best_adapter(
    instance: &Instance,
    surface: Option<&Surface<'_>>,
) -> Result<Adapter, InitError> {
    for &pref in &[
        PowerPreference::HighPerformance,
        PowerPreference::LowPower,
        PowerPreference::None,
    ] {
        if let Some(a) = try_pref(instance, surface, pref).await {
            return Ok(a);
        }
    }
    Err(InitError::NoAdapter)
}

#[derive(Debug)]
pub enum WgpuInitError {
    NoAdapter,
    NoDevice,
}

#[derive(Debug, Clone, Copy)]
pub enum LimitsTier {
    ModernDefault,
    Downlevel,
    WebGL2,
}

#[derive(Debug)]
pub struct DeviceSelection {
    pub device: Device,
    pub queue: Queue,
    pub tier: LimitsTier,
    pub applied_limits: Limits,
}

async fn try_with(adapter: &Adapter, base: Limits, tier: LimitsTier) -> Option<DeviceSelection> {
    // Use .using_resolution(adapter.limits()) before request_device to clamp your
    // desired baseline limits to what the GPU actually supports. This avoids
    // “requested limits exceed adapter capabilities” errors on older or downlevel GPUs.
    let applied = base.using_resolution(adapter.limits());

    info!(?tier, "trying to find device with tier");

    adapter
        .request_device(&DeviceDescriptor {
            label: Some("wgpu device"),
            required_features: Features::empty(), // require nothing
            required_limits: applied.clone(),
            memory_hints: MemoryHints::default(),
            trace: Trace::default(),
        })
        .await
        .ok()
        .map(|(device, queue)| DeviceSelection {
            device,
            queue,
            tier,
            applied_limits: applied,
        })
}

/*
    let device_descriptor = DeviceDescriptor {
        label: None,
        required_features: Features::empty(), // Specify features as needed
        required_limits: if cfg!(target_arch = "wasm32") {
            Limits::downlevel_webgl2_defaults() // TODO: Not sure if this is needed?
        } else {
            Limits::default()
        },
        memory_hints: MemoryHints::default(), // Use default memory hints
        trace: Trace::default(),
    };


    debug!(?device_descriptor, "device descriptor");

    let (device, queue) = adapter
        .request_device(&device_descriptor)
        .await
        .expect("Failed to request device");
    debug!(?device, "device");
*/

/// # Errors
///
pub async fn request_device_smart(adapter: &Adapter) -> Result<DeviceSelection, InitError> {
    if let Some(sel) = try_with(adapter, Limits::default(), LimitsTier::ModernDefault).await {
        return Ok(sel);
    }
    if let Some(sel) = try_with(adapter, Limits::downlevel_defaults(), LimitsTier::Downlevel).await
    {
        return Ok(sel);
    }
    if let Some(sel) = try_with(
        adapter,
        Limits::downlevel_webgl2_defaults(),
        LimitsTier::WebGL2,
    )
    .await
    {
        return Ok(sel);
    }

    Err(InitError::NoDevice)
}

#[derive(Debug)]
pub enum InitError {
    CreateSurface(wgpu::CreateSurfaceError),
    NoAdapter,
    NoDevice,
    RequestDevice(wgpu::RequestDeviceError),
}

/// # Errors
///
/// # Panics
///
pub async fn annoying_async_device_creation(
    window: Arc<Window>,
) -> Result<BasicDeviceInfo, InitError> {
    let instance = Instance::new(&InstanceDescriptor {
        backends: Backends::default(),
        flags: InstanceFlags::default(),
        memory_budget_thresholds: MemoryBudgetThresholds::default(),
        backend_options: BackendOptions::default(),
    });
    debug!(?instance, "found instance");

    let surface = instance
        .create_surface(Arc::clone(&window))
        .map_err(InitError::CreateSurface)?;
    debug!(?surface, "found surface");

    let adapter = pick_best_adapter(&instance, Some(&surface)).await?;
    debug!(?adapter, "found adapter");

    let device_selection = request_device_smart(&adapter).await?;
    debug!(?device_selection, "found device selection");

    let inner_size = window.inner_size();
    debug!(?inner_size, "window size detected");

    let device_info = BasicDeviceInfo {
        adapter,
        device: device_selection.device.into(),
        queue: device_selection.queue.into(),
        surface: surface.into(),
        physical_surface_size: inner_size,
    };

    debug!(?device_info, "final device info selection");

    Ok(device_info)
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

        let present_mode = wgpu::PresentMode::Fifo; // Fifo should be available on all devices
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: info.physical_surface_size.width,
            height: info.physical_surface_size.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        info.surface.configure(&info.device, &config);

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

    pub fn configure_surface(&mut self) {
        debug!("configured surface! {:?}", self.config);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn resize(&mut self, new_size: (u16, u16)) {
        let width = new_size.0 as usize;
        let height = new_size.1 as usize;

        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width as u32;
        self.config.height = height as u32;
        self.configure_surface();
    }

    pub fn render(
        &self,
        mut render_fn: impl FnMut(&mut wgpu::CommandEncoder, &wgpu::TextureView),
    ) -> Result<(), SurfaceError> {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => {
                // Surface is out of date: reconfigure and bail out of this frame.
                // TODO: Get window size somehow
                //                self.surface.configure(&self.device, &self.config);
                return Ok(()); // hopefully next frame will work
            }
            Err(wgpu::SurfaceError::Lost) => {
                // Lost is treated the same as Outdated in wgpu 0.13+
                return Ok(()); // hopefully next frame will work
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                // fatal – bail out of the app
                eprintln!("GPU out of memory, exiting");
                std::process::exit(1);
            }
            Err(e) => {
                // other errors (Timeout, etc): log and skip this frame
                eprintln!("wgpu error getting next swap chain texture: {e:?}");
                return Err(SurfaceError::Other);
            }
        };
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
