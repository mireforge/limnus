/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use int_math::UVec2;
use limnus::prelude::{LocalResource, WgpuWindow};
use limnus::{
    DefaultPlugins,
    prelude::{ScreenMode, Window},
};
use limnus_app::prelude::App;
use limnus_default_stages::{FixedUpdate, RenderUpdate};
use limnus_system_params::{LoRe, LoReM};

pub fn clear_color_tick(steam_app: LoRe<SteamDeckTest>, wgpu_window: LoRe<WgpuWindow>) {
    wgpu_window
        .render(|encoder, texture_view| {
            let time = steam_app.tick % 60;
            let normalized_time = f64::from(time) / 60.0;
            let clear_color = wgpu::Color {
                r: 1.0,
                g: normalized_time,
                b: 0.0,
                a: 0.8,
            };

            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: texture_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        })
        .unwrap();
}

#[allow(clippy::missing_const_for_fn)]
pub fn update_color(mut steam_test: LoReM<SteamDeckTest>) {
    steam_test.tick += 1;
}

#[derive(Debug, LocalResource)]
pub struct SteamDeckTest {
    pub tick: u32,
}

const STEAM_DECK_WIDTH: u16 = 1280;
const STEAM_DECK_HEIGHT: u16 = 800;

fn main() {
    let mut app = App::new();

    let window_settings = Window {
        mode: ScreenMode::WindowedFullscreen,
        title: "hello".to_string(),
        requested_surface_size: UVec2::new(STEAM_DECK_WIDTH, STEAM_DECK_HEIGHT), // ignored in wasm
        minimal_surface_size: UVec2::new(STEAM_DECK_WIDTH, STEAM_DECK_HEIGHT),   // ignored in wasm,
    };

    let app = app
        .add_plugins(DefaultPlugins)
        .insert_resource(window_settings);

    app.insert_local_resource(SteamDeckTest { tick: 0 });
    app.add_system(RenderUpdate, clear_color_tick);
    app.add_system(FixedUpdate, update_color);

    app.run();
}
