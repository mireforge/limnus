use limnus_app::prelude::{App, Plugin};
use limnus_basic_input::prelude::{ButtonState, KeyCode};
use limnus_basic_input::InputMessage;
use limnus_default_stages::First;
use limnus_local_resource::LocalResource;
use limnus_macros::LocalResource;
use limnus_screen::{ScreenMode, Window};
use limnus_system_params::{LoReM, Msg, ReM};

#[derive(Debug, LocalResource)]
pub struct DefaultKeys {
    pub left_alt_key: bool,
    pub left_shift_key: bool,
}

fn check_key(
    key_input_messages: Msg<InputMessage>,
    mut default_keys: LoReM<DefaultKeys>,
    mut window_settings: ReM<Window>,
) {
    for key_input in key_input_messages.iter_previous() {
        if let InputMessage::KeyboardInput(button_state, button) = key_input {
            match button {
                KeyCode::AltLeft => {
                    default_keys.left_alt_key = *button_state == ButtonState::Pressed;
                }
                KeyCode::ShiftLeft => {
                    default_keys.left_shift_key = *button_state == ButtonState::Pressed;
                }
                KeyCode::Enter => {
                    if default_keys.left_alt_key && *button_state == ButtonState::Pressed {
                        let new_mode = match window_settings.mode {
                            ScreenMode::WindowedOnTop | ScreenMode::Windowed => {
                                ScreenMode::WindowedFullscreen
                            }
                            ScreenMode::WindowedFullscreen => {
                                if default_keys.left_shift_key {
                                    ScreenMode::WindowedOnTop
                                } else {
                                    ScreenMode::Windowed
                                }
                            }
                        };
                        window_settings.mode = new_mode;
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct DefaultKeysPlugin;

impl Plugin for DefaultKeysPlugin {
    fn build(&self, app: &mut App) {
        app.insert_local_resource(DefaultKeys {
            left_alt_key: false,
            left_shift_key: false,
        });
        app.add_system(First, check_key);
    }
}
