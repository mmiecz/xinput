use ffi;
use winapi;
use winapi::{ DWORD };
use std::mem;

use ::DeviceError;
use ::InputState;

#[derive(Clone, Copy, Debug)]
pub struct Gamepad {
    pub w_buttons: u16,
    pub b_left_trigger: u8,
    pub b_right_trigger: u8,
    pub s_thumb_lx: i16,
    pub s_thumb_ly: i16,
    pub s_thumb_rx: i16,
    pub s_thumb_ry: i16,
}

impl Gamepad {
    pub fn new() -> Gamepad {
        Gamepad {
            w_buttons: 0,
            b_left_trigger: 0,
            b_right_trigger: 0,
            s_thumb_lx: 0,
            s_thumb_ly: 0,
            s_thumb_rx: 0,
            s_thumb_ry: 0,
        }
    }
    pub fn from_raw(raw_gamepad: &winapi::XINPUT_GAMEPAD) -> Gamepad {
        let w_buttons = raw_gamepad.wButtons;
        let b_left_trigger = raw_gamepad.bLeftTrigger;
        let b_right_trigger = raw_gamepad.bRightTrigger;
        let s_thumb_lx = raw_gamepad.sThumbLX;
        let s_thumb_ly = raw_gamepad.sThumbLY;
        let s_thumb_rx = raw_gamepad.sThumbRX;
        let s_thumb_ry = raw_gamepad.sThumbRY;
        Gamepad {
            w_buttons: w_buttons,
            b_left_trigger: b_left_trigger,
            b_right_trigger: b_right_trigger,
            s_thumb_lx: s_thumb_lx,
            s_thumb_ly: s_thumb_ly,
            s_thumb_rx: s_thumb_rx,
            s_thumb_ry: s_thumb_ry,
        }
    }
}

pub fn get_input_state(user_index: u32) -> Result<InputState, DeviceError> {
    let mut raw_input_state: winapi::XINPUT_STATE = unsafe { mem::zeroed() };
    let raw_user_index: DWORD = user_index;
    let raw_result = unsafe { ffi::XInputGetState(raw_user_index, &mut raw_input_state) };
    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }
    Ok(InputState::from_raw(&raw_input_state))
}
