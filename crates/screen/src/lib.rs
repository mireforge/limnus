/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use int_math::UVec2;
use limnus_message::prelude::Message;
use limnus_resource::prelude::*;

#[derive(Debug, Clone)]
pub enum ScreenMode {
    WindowedFullscreen,
    Windowed,
    WindowedOnTop,
}

#[derive(Debug, Resource, Clone)]
pub struct Window {
    pub mode: ScreenMode,
    pub title: String,
    pub requested_surface_size: UVec2,
    pub minimal_surface_size: UVec2,
}

#[derive(Message, Debug)]
pub enum WindowMessage {
    CursorMoved(UVec2),
    WindowCreated(),
    Resized(UVec2),
}
