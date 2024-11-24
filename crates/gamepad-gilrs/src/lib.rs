/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use gilrs::{Error, Gilrs};
use limnus_gamepad::Gamepads;
use limnus_system_params::{LocReAll, ReM};
use limnus_app::prelude::{App, Plugin};
use limnus_local_resource::prelude::LocalResource;
use limnus_system_runner::UpdatePhase;

pub enum GamepadError {
    Error(Error),
}

impl From<Error> for GamepadError {
    fn from(error: Error) -> GamepadError {
        Self::Error(error)
    }
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
}

fn check_gamepads(mut local_resource: LocReAll, mut _data: ReM<Gamepads>) {
    let _gilrs = local_resource
        .get_mut::<GamepadGilrs>()
        .expect("gamepad gilrs should have been added");

    // TODO:
}
pub struct GamepadGilrsPlugin;

impl Plugin for GamepadGilrsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(UpdatePhase::First, check_gamepads);
    }
}
