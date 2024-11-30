/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use gilrs::{Error, EventType, Gilrs};
use limnus_app::prelude::{App, Plugin};
use limnus_gamepad::{Axis, Button, GamepadMessage, Gamepads};
use limnus_local_resource::prelude::LocalResource;
use limnus_message::Messages;
use limnus_system_params::prelude::*;
use limnus_system_runner::UpdatePhase;
use tracing::trace;

#[derive(Debug)]
pub enum GamepadError {
    Error(Error),
}

impl From<Error> for GamepadError {
    fn from(error: Error) -> GamepadError {
        Self::Error(error)
    }
}

#[inline]
fn convert_axis(axis: &gilrs::Axis) -> Option<Axis> {
    let converted = match axis {
        gilrs::Axis::LeftStickX => Axis::LeftStickX,
        gilrs::Axis::LeftStickY => Axis::LeftStickY,
        gilrs::Axis::RightStickX => Axis::RightStickX,
        gilrs::Axis::RightStickY => Axis::RightStickY,
        _ => return None,
    };
    Some(converted)
}

#[inline]
fn convert_button(axis: &gilrs::Button) -> Option<Button> {
    let converted = match axis {
        // Action Pad
        gilrs::Button::South => Button::South,
        gilrs::Button::East => Button::East,
        gilrs::Button::North => Button::North,
        gilrs::Button::West => Button::West,

        // Triggers
        gilrs::Button::LeftTrigger => Button::LeftTrigger,
        gilrs::Button::LeftTrigger2 => Button::LeftTrigger2,
        gilrs::Button::RightTrigger => Button::RightTrigger,
        gilrs::Button::RightTrigger2 => Button::RightTrigger2,

        // Menu Buttons
        gilrs::Button::Select => Button::Select,
        gilrs::Button::Start => Button::Start,
        gilrs::Button::Mode => Button::Mode,

        // Sticks
        gilrs::Button::LeftThumb => Button::LeftThumb,
        gilrs::Button::RightThumb => Button::RightThumb,

        // D-Pad
        gilrs::Button::DPadUp => Button::DPadUp,
        gilrs::Button::DPadDown => Button::DPadDown,
        gilrs::Button::DPadLeft => Button::DPadLeft,
        gilrs::Button::DPadRight => Button::DPadRight,

        _ => return None,
    };
    Some(converted)
}

#[derive(Debug, LocalResource)]
pub struct GamepadGilrs {
    #[allow(dead_code)]
    gilrs: Gilrs,
}

impl GamepadGilrs {
    pub fn new() -> Result<Self, GamepadError> {
        let gilrs = Gilrs::new()?;

        Ok(Self { gilrs })
    }

    pub fn debug_output(&self) {
        for (id, gamepad) in self.gilrs.gamepads() {
            trace!("{id}:{} is {:?}", gamepad.name(), gamepad.power_info());
        }
    }

    pub fn tick(&mut self, gamepads: &mut Gamepads, queue: &mut Messages<GamepadMessage>) {
        while let Some(event) = self.gilrs.next_event() {
            trace!(event=?event, "gilrs gamepad event");
            match event.event {
                EventType::ButtonPressed(_, _) => {}
                EventType::ButtonRepeated(_, _) => {}
                EventType::ButtonReleased(_, _) => {}
                EventType::ButtonChanged(gilrs_button, button_value, _) => {
                    if let Some(real_button) = convert_button(&gilrs_button) {
                        gamepads.set_button(event.id.into(), real_button, button_value, queue);
                    }
                }
                EventType::AxisChanged(gilrs_axis, axis_value, _) => {
                    if let Some(real_axis) = convert_axis(&gilrs_axis) {
                        gamepads.set_axis(event.id.into(), real_axis, axis_value, queue);
                    }
                }
                EventType::Connected => {
                    let gamepad = self.gilrs.gamepad(event.id);
                    gamepads.connected(event.id.into(), gamepad.name(), queue);
                }
                EventType::Disconnected => {
                    gamepads.disconnected(event.id.into(), queue);
                }
                EventType::Dropped => {}
                EventType::ForceFeedbackEffectCompleted => {}
                _ => {}
            }
        }
    }
}

fn check_gamepads(
    mut gilrs: LoReM<GamepadGilrs>,
    mut gamepads: ReM<Gamepads>,
    mut queue: MsgM<GamepadMessage>,
) {
    gilrs.tick(&mut gamepads, &mut queue);
}

pub struct GamepadGilrsPlugin;

impl Plugin for GamepadGilrsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_local_resource(GamepadGilrs::new().expect("Failed to initialize GamepadGilrs"));

        app.add_system(UpdatePhase::First, check_gamepads);
    }
}
