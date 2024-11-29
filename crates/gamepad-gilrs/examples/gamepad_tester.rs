use limnus_app::prelude::{App, AppReturnValue};
use limnus_gamepad::GamepadResourcePlugin;
use limnus_gamepad_gilrs::GamepadGilrsPlugin;
use limnus_log::LogPlugin;
use std::thread::sleep;
use std::time::Duration;

fn test_runner(mut app: App) -> AppReturnValue {
    for _ in 0..1000 {
        app.update();
        sleep(Duration::from_millis(100));
    }
    AppReturnValue::Value(0)
}

fn main() {
    let mut app = App::new();
    app.add_plugins((LogPlugin, GamepadResourcePlugin, GamepadGilrsPlugin));

    app.set_runner(test_runner);
    app.run();
}
