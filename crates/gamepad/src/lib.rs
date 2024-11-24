/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use limnus_app::prelude::{App, Plugin};
use limnus_resource::prelude::Resource;

pub enum Button {
    // Right side pad
    South,
    East,
    North,
    West,

    // Triggers
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,

    // Menu Buttons
    Select,
    Start,
    Mode, // Xbox Button, PS button, etc

    // Sticks
    LeftThumb,
    RightThumb,

    // D-Pad (usually on the left side)
    DPadUp,
    DPadDow,
    DPadLeft,
    DPadRight,
}

pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

#[derive(Debug, Resource)]
pub struct Gamepads {
    // TODO: Add data here
}

pub struct GamepadResourcePlugin;

impl Plugin for GamepadResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gamepads {}); // TODO:
    }
}
