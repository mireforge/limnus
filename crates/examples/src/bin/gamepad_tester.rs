/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use limnus_app::prelude::{App, AppReturnValue};
use limnus_clock::ClockPlugin;
use limnus_default_schedulers::DefaultSchedulersPlugin;
use limnus_default_stages::Update;
use limnus_default_stages_plugin::DefaultStagesPlugin;
use limnus_gamepad::{GamepadMessage, GamepadResourcePlugin, Gamepads};
use limnus_gamepad_gilrs::GamepadGilrsPlugin;
use limnus_local_resource::prelude::LocalResource;
use limnus_log::LogPlugin;
use limnus_system_params::{LoReM, Msg, Re};
use std::thread::sleep;
use std::time::Duration;
use tracing::info;

fn test_runner(mut app: App) -> AppReturnValue {
    loop {
        app.update();
        sleep(Duration::from_millis(32));
    }
    // AppReturnValue::Value(0)
}

#[derive(Debug, LocalResource)]
pub struct TesterState {
    pub counter: u32,
}

fn tick(gamepads: Re<Gamepads>, queue: Msg<GamepadMessage>, mut state: LoReM<TesterState>) {
    for message in queue.iter_current() {
        match message {
            GamepadMessage::Connected(gamepad_id, name) => {
                info!(id=%gamepad_id, name=name, "Connected gamepad");
                println!("Connected gamepad {gamepad_id}: {name}");
            }
            GamepadMessage::Disconnected(gamepad_id) => {
                info!(id=%gamepad_id, "Disconnected gamepad");
                println!("Disconnected gamepad {gamepad_id}");
            }
            GamepadMessage::Activated(gamepad_id) => {
                info!(id=%gamepad_id, "Activated gamepad");
                println!("Activated gamepad {gamepad_id}");
            }
            GamepadMessage::ButtonChanged(gamepad_id, button, value) => {
                info!(id=%gamepad_id, button=?button, value=value, "pressed button");
                println!("Pressed button {button:?} to {value}");
            }
            GamepadMessage::AxisChanged(gamepad_id, axis, value) => {
                info!(id=%gamepad_id, axis=?axis, value=value, "moved axis");
                println!("Moved axis {axis:?} to {value}");
            }
        }
    }

    state.counter += 1;

    if (state.counter % 30) == 0 {
        for gamepad in gamepads.iter_active() {
            println!(
                "{}{}, axis:{:?} buttons:{:?}",
                gamepad.id,
                gamepad.name.as_str(),
                gamepad.axis,
                gamepad.buttons
            );
        }
    }
}

fn main() {
    println!("Gamepad Tester");

    let mut app = App::new();

    app.add_plugins((
        LogPlugin,
        DefaultStagesPlugin,
        ClockPlugin,
        DefaultSchedulersPlugin,
    ));

    app.add_plugins((GamepadResourcePlugin, GamepadGilrsPlugin));

    app.add_system(Update, tick);

    app.insert_local_resource(TesterState { counter: 0 });

    app.set_runner(test_runner);

    app.run();
}
