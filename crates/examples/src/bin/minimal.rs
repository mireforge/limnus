/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use int_math::UVec2;
use limnus::prelude::{LocalResource, WgpuWindow};
use limnus::{
    prelude::{ScreenMode, Window},
    DefaultPlugins,
};
use limnus_app::prelude::App;
use limnus_default_stages::{FixedUpdate, RenderUpdate};
use limnus_system_params::{LoRe, LoReM, ReM};

pub fn clear_color_tick(minimal: LoRe<Minimal>, wgpu_window: LoRe<WgpuWindow>) {
    let time = minimal.tick % 60;
    let normalized_time = time as f64 / 60.0;

    let color = wgpu::Color {
        r: 1.0,
        g: normalized_time,
        b: 0.0,
        a: 0.8,
    };
    wgpu_window.render(color, |_render_pass| {}).unwrap();
}

pub fn update_color(mut minimal: LoReM<Minimal>) {
    minimal.tick += 1;
}

pub fn change_size(mut minimal: LoReM<Minimal>, mut window_settings: ReM<Window>) {
    let x = (minimal.tick / 60) % 60;
    let width = 640 + x;
    window_settings.requested_surface_size.x = width as u16;
    let height = 320 + x * 4;
    window_settings.requested_surface_size.y = height as u16;
}

#[derive(Debug, LocalResource)]
pub struct Minimal {
    pub tick: u32,
}

fn main() {
    let mut app = App::new();

    let window_settings = Window {
        mode: ScreenMode::Windowed, // in wasm this will be ignored
        title: "hello".to_string(),
        requested_surface_size: UVec2::new(320, 200), // ignored in wasm
        minimal_surface_size: UVec2::new(320, 200),   // ignored in wasm,
    };

    let app = app
        .add_plugins(DefaultPlugins)
        .insert_resource(window_settings);

    app.insert_local_resource(Minimal { tick: 0 });
    app.add_system(RenderUpdate, clear_color_tick);
    app.add_system(FixedUpdate, update_color);
    app.add_system(FixedUpdate, change_size);

    app.run();
}
