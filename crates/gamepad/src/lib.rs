/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use fixstr::FixStr;
use limnus_app::prelude::{App, Plugin};
use limnus_message::prelude::Message;
use limnus_message::Messages;
use limnus_resource::prelude::Resource;
use std::collections::HashMap;
use tracing::{debug, error, trace};

#[repr(usize)]
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
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
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

pub type GamePadId = usize;

pub type AxisValueType = f32;
pub type ButtonValueType = f32;

/// Messages sent when gamepad state changes
#[derive(Debug, Message)]
pub enum GamepadMessage {
    Connected(GamePadId, String),
    Disconnected(GamePadId),
    Activated(GamePadId), // Sent when first button is pressed
    ButtonChanged(GamePadId, Button, ButtonValueType),
    AxisChanged(GamePadId, Axis, AxisValueType),
}

/// Represents a single gamepad's state
#[derive(Default, Debug, Clone)]
pub struct Gamepad {
    pub axis: [AxisValueType; 4],
    pub buttons: [ButtonValueType; 17],
    pub name: FixStr<64>,
    pub id: GamePadId,
    pub is_active: bool,
}

impl Gamepad {
    #[must_use]
    pub fn new(id: GamePadId, name: &str) -> Self {
        let truncated_name: String = name.chars().take(32).collect();
        Self {
            axis: [0.0; 4],
            buttons: [0.0; 17],
            name: FixStr::new(&truncated_name).expect("gamepad name too long"), // TODO: Make a better solution for this
            id,
            is_active: false,
        }
    }

    /// Gets the button state as a boolean
    #[must_use]
    pub fn is_pressed(&self, button: Button) -> bool {
        self.buttons[button as usize] > 0.1
    }

    #[must_use]
    pub const fn axis(&self, axis: Axis) -> AxisValueType {
        self.axis[axis as usize]
    }

    #[must_use]
    pub const fn button(&self, button: Button) -> ButtonValueType {
        self.buttons[button as usize]
    }
}

#[derive(Debug, Resource)]
pub struct Gamepads {
    gamepads: HashMap<GamePadId, Gamepad>,
}

impl Default for Gamepads {
    fn default() -> Self {
        Self::new()
    }
}

impl Gamepads {
    /// Creates a new GamePad instance
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this gamepad
    /// * `name` - Human-readable name of the gamepad
    pub fn new() -> Self {
        Self {
            gamepads: HashMap::new(),
        }
    }
    pub fn connected(&mut self, id: GamePadId, name: &str, queue: &mut Messages<GamepadMessage>) {
        debug!(id=%id, name=name, "connected gamepad");
        self.gamepads.insert(id, Gamepad::new(id, name));
        queue.send(GamepadMessage::Connected(id, name.to_string()));
    }

    pub fn disconnected(&mut self, id: GamePadId, queue: &mut Messages<GamepadMessage>) {
        if let Some(existing) = self.gamepads.remove(&id) {
            debug!(id=%id, name=?existing.name, "disconnected gamepad");
            queue.send(GamepadMessage::Disconnected(id));
        } else {
            error!(id=%id, "gamepad not found");
        }
    }

    #[must_use]
    pub fn gamepad(&self, id: GamePadId) -> Option<&Gamepad> {
        self.gamepads.get(&id)
    }

    /// Gets the axis value for a gamepad
    #[must_use]
    pub fn axis(&self, id: GamePadId, axis: Axis) -> Option<AxisValueType> {
        self.gamepad(id).map(|pad| pad.axis[axis as usize])
    }

    /// Gets the button value for a gamepad
    #[must_use]
    pub fn button(&self, id: GamePadId, button: Button) -> Option<ButtonValueType> {
        self.gamepad(id).map(|pad| pad.buttons[button as usize])
    }

    pub fn iter_active(&self) -> impl Iterator<Item = &Gamepad> {
        self.gamepads.values().filter(|gamepad| gamepad.is_active)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Gamepad> {
        self.gamepads.values()
    }

    pub fn set_axis(
        &mut self,
        id: GamePadId,
        axis: Axis,
        value: AxisValueType,
        queue: &mut Messages<GamepadMessage>,
    ) -> Option<()> {
        trace!(id=?id, axis=?axis, value=?value, "set axis");
        let gamepad = self.gamepads.get_mut(&id)?;

        queue.send(GamepadMessage::AxisChanged(id, axis, value));
        gamepad.axis[axis as usize] = value;

        Some(())
    }

    pub fn set_button(
        &mut self,
        id: GamePadId,
        button: Button,
        value: ButtonValueType,
        queue: &mut Messages<GamepadMessage>,
    ) -> Option<()> {
        trace!(id=?id, button=?button, value=?value, "set button");

        let gamepad = self.gamepads.get_mut(&id)?;

        if !gamepad.is_active && value > 0.1 {
            debug!(id=%id, button=?button, name=%gamepad.name, "gamepad activated");
            queue.send(GamepadMessage::Activated(id));
            gamepad.is_active = true;
        }

        queue.send(GamepadMessage::ButtonChanged(id, button, value));
        gamepad.buttons[button as usize] = value;
        Some(())
    }

    /// Gets the name of a gamepad
    #[must_use]
    pub fn name(&self, id: GamePadId) -> Option<&str> {
        self.gamepad(id).map(|pad| pad.name.as_str())
    }
}

pub struct GamepadResourcePlugin;

impl Plugin for GamepadResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gamepads::new());
        app.create_message_type::<GamepadMessage>();
    }
}
