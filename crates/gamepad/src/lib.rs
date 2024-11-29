/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use fixstr::FixStr;
use limnus_app::prelude::{App, Plugin};
use limnus_resource::prelude::Resource;
use seq_map::SeqMap;
use std::collections::HashMap;
use tracing::{debug, error, trace};

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
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
    DPadDown,
    DPadLeft,
    DPadRight,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

pub type GamePadId = usize;

pub type AxisValueType = f32;
pub type ButtonValueType = f32;

#[derive(Default, Debug, Clone)]
pub struct GamePad {
    pub axis: [AxisValueType; 4],
    pub buttons: [ButtonValueType; 17],
    pub name: FixStr<64>,
    pub id: GamePadId,
}

impl GamePad {
    pub fn new(id: GamePadId, name: &str) -> Self {
        let truncated_name: String = name.chars().take(32).collect();
        GamePad {
            axis: [0.0; 4],
            buttons: [0.0; 17],
            name: FixStr::new(&truncated_name).expect("gamepad name too long"), // TODO: Make a better solution for this
            id,
        }
    }
}

#[derive(Debug, Resource)]
pub struct Gamepads {
    gamepads: HashMap<GamePadId, GamePad>,
}

impl Gamepads {}

impl Gamepads {
    pub fn new() -> Self {
        Self {
            gamepads: HashMap::new(),
        }
    }
    pub fn connected(&mut self, id: GamePadId, name: &str) {
        debug!(id=%id, name=name, "connected gamepad");
        self.gamepads.insert(id, GamePad::new(id, name));
    }

    pub fn disconnected(&mut self, id: GamePadId) {
        if let Some(existing) = self.gamepads.remove(&id) {
            debug!(id=%id, name=?existing.name, "disconnected gamepad");
        } else {
            error!(id=%id, "gamepad not found");
        }
    }

    pub fn axis(&self, id: GamePadId, axis: Axis) -> AxisValueType {
        self.gamepads[&id].axis[axis as usize]
    }
    pub fn button(&self, id: GamePadId, button: Button) -> ButtonValueType {
        self.gamepads[&id].buttons[button as usize]
    }

    pub fn set_axis(&mut self, id: GamePadId, axis: Axis, value: AxisValueType) {
        trace!(id=?id, axis=?axis, value=?value, "set axis");
        self.gamepads.get_mut(&id).unwrap().axis[axis as usize] = value;
    }

    pub fn set_button(&mut self, id: GamePadId, button: Button, value: ButtonValueType) {
        trace!(id=?id, button=?button, value=?value, "set button");
        self.gamepads.get_mut(&id).unwrap().buttons[button as usize] = value;
    }

    pub fn name(&self, id: GamePadId) -> &str {
        &self.gamepads[&id].name.as_str()
    }
}

pub struct GamepadResourcePlugin;

impl Plugin for GamepadResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gamepads::new());
    }
}
