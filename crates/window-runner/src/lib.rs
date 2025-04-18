/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
mod convert;

use crate::convert::{
    from_element_state, from_mouse_scroll_delta, from_touch_phase, try_from_key_code,
    try_from_mouse_button,
};
use int_math::{UVec2, Vec2};
use limnus_app::prelude::{App, AppReturnValue, ApplicationExit};
use limnus_basic_input::prelude::*;
use limnus_screen::{ScreenMode, Window, WindowMessage};
use limnus_wgpu_window::{WgpuWindow, annoying_async_device_creation};
use limnus_window::{AppHandler, WindowMode};
use std::sync::{Arc, Mutex};
use tracing::{debug, error};
use winit::dpi;
use winit::keyboard::PhysicalKey;

pub struct WindowState {
    pub app: Arc<Mutex<App>>,
    pub wgpu_window: Option<WgpuWindow>,
    pub mode: ScreenMode,
    requested_surface_size: UVec2,
    minimal_surface_size: UVec2,
    physical_surface_size: dpi::PhysicalSize<u32>,
}

impl AppHandler for WindowState {
    fn min_size(&self) -> (u16, u16) {
        (self.minimal_surface_size.x, self.minimal_surface_size.y)
    }

    fn window_mode(&self) -> WindowMode {
        match self.mode {
            ScreenMode::WindowedFullscreen => WindowMode::WindowedFullscreen,
            ScreenMode::Windowed => WindowMode::Windowed,
            ScreenMode::WindowedOnTop => WindowMode::WindowedAlwaysOnTop,
        }
    }

    fn start_size(&self) -> (u16, u16) {
        (self.requested_surface_size.x, self.requested_surface_size.y)
    }

    fn cursor_should_be_visible(&self) -> bool {
        true
    }

    fn redraw(&mut self) -> bool {
        let mut app = self.app.lock().unwrap();
        let window_settings = app.resources().fetch::<Window>();
        self.mode = window_settings.mode.clone();
        self.requested_surface_size = window_settings.requested_surface_size;
        self.minimal_surface_size = window_settings.minimal_surface_size;

        app.update();

        !app.has_resource::<ApplicationExit>()
    }

    fn got_focus(&mut self) {}

    fn lost_focus(&mut self) {}

    fn window_created(&mut self, window: Arc<winit::window::Window>) {
        let app = Arc::clone(&self.app);
        future_runner::run_future(async move {
            let async_device_info = annoying_async_device_creation(window)
                .await
                .expect("couldn't get device info");
            app.lock().unwrap().insert_local_resource(async_device_info);
        });
        self.app
            .lock()
            .unwrap()
            .send(WindowMessage::WindowCreated());
    }

    fn resized(&mut self, size: dpi::PhysicalSize<u32>) {
        self.physical_surface_size = size;
        self.app
            .lock()
            .unwrap()
            .send(WindowMessage::Resized(UVec2::new(
                size.width as u16,
                size.height as u16,
            )));
    }

    fn keyboard_input(
        &mut self,
        element_state: winit::event::ElementState,
        physical_key: winit::keyboard::PhysicalKey,
    ) {
        if let PhysicalKey::Code(key_code) = physical_key {
            if let Ok(converted_key) = try_from_key_code(key_code) {
                self.app.lock().unwrap().send(InputMessage::KeyboardInput(
                    from_element_state(element_state),
                    converted_key,
                ));
            }
        }
    }

    fn cursor_entered(&mut self) {}

    fn cursor_left(&mut self) {}

    /// opinionated, we want origin (0,0) in the lower left corner of the window
    fn cursor_moved(&mut self, physical_position: dpi::PhysicalPosition<u32>) {
        if physical_position.x >= self.physical_surface_size.width {
            return;
        }
        if physical_position.y >= self.physical_surface_size.height {
            return;
        }

        if self.physical_surface_size.height == 0 {
            return;
        }

        if self
            .physical_surface_size
            .height
            .checked_sub(physical_position.y)
            .is_none()
        {
            error!(
                "problem! {} {:?}",
                self.physical_surface_size.height, physical_position.y
            );
        }

        self.app
            .lock()
            .unwrap()
            .send(WindowMessage::CursorMoved(UVec2::new(
                physical_position.x as u16,
                ((self.physical_surface_size.height - 1) - physical_position.y) as u16,
            )));
    }

    fn mouse_input(
        &mut self,
        element_state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) {
        if let Ok(converted_button) = try_from_mouse_button(button) {
            self.app.lock().unwrap().send(InputMessage::MouseInput(
                from_element_state(element_state),
                converted_button,
            ));
        }
    }

    fn mouse_wheel(
        &mut self,
        delta: winit::event::MouseScrollDelta,
        touch_phase: winit::event::TouchPhase,
    ) {
        self.app.lock().unwrap().send(InputMessage::MouseWheel(
            from_mouse_scroll_delta(delta),
            from_touch_phase(touch_phase),
        ));
    }

    fn pinch_gesture(&mut self, delta: f64, touch_phase: winit::event::TouchPhase) {
        let virtual_wheel_y = (delta * 50.0) as i16;
        let mouse_delta = MouseScrollDelta::LineDelta(Vec2::new(0, virtual_wheel_y));
        self.app.lock().unwrap().send(InputMessage::MouseWheel(
            mouse_delta,
            from_touch_phase(touch_phase),
        ));
    }

    fn mouse_motion(&mut self, _delta: (f64, f64)) {}

    fn touch(&mut self, _touch: winit::event::Touch) {}

    fn scale_factor_changed(
        &mut self,
        _scale_factor: f64,
        _inner_size_writer: winit::event::InnerSizeWriter,
    ) {
    }
}

pub fn runner(mut app: App) -> AppReturnValue {
    console_error_panic_hook::set_once();
    debug!("window-runner started!");

    let requested_surface_size: UVec2;
    let minimal_surface_size: UVec2;
    let title: String;
    let mode: ScreenMode;

    {
        let window_settings = app.resource::<limnus_screen::Window>();

        title = window_settings.title.clone();
        requested_surface_size = window_settings.requested_surface_size;
        minimal_surface_size = window_settings.minimal_surface_size;
        mode = window_settings.mode.clone();

        app.create_message_type::<WindowMessage>();
        app.create_message_type::<InputMessage>();
    }

    #[allow(clippy::arc_with_non_send_sync)]
    let arc_app = Arc::new(Mutex::new(app));

    let mut state = WindowState {
        app: arc_app,
        wgpu_window: None,
        mode,
        requested_surface_size,
        minimal_surface_size,
        physical_surface_size: dpi::PhysicalSize::new(
            requested_surface_size.x as u32,
            requested_surface_size.y as u32,
        ),
    };

    limnus_window::WindowRunner::run_app(&mut state, title.as_str()).expect("run_app failed");

    debug!("we returned, that is not guaranteed for all platforms");

    AppReturnValue::Value(0)
}
