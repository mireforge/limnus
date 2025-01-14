use int_math::UVec2;
use limnus::prelude::{LocalResource, WgpuWindow};
use limnus::{
    prelude::{ScreenMode, Window},
    DefaultPlugins,
};
use limnus_app::prelude::App;
use limnus_default_stages::{FixedUpdate, RenderUpdate};
use limnus_system_params::{LoRe, LoReM};

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

    app.run();
}
