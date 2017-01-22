extern crate libc;
extern crate winapi;

mod ffi;
pub mod battery;

use std::mem;
use ffi::{XInputGetCapabilities, XInputGetState};
use winapi::{BOOL, DWORD, XINPUT_CAPABILITIES};

#[derive(Copy, Clone, Debug)]
pub enum DeviceError {
    DeviceNotConnected,
}

///Turn on communication with the controller.
pub fn enable(enable: bool) -> () {
    let enable: BOOL = enable as i32;
    unsafe {
        ffi::XInputEnable(enable);
    };
}

//pub struct Vibration XINPUT_VIBRATION

///Represents state of buttons, triggers, etc. of the controller.
#[derive(Clone, Copy, Debug)]
pub struct InputGamepad {
    pub w_buttons: u16,
    pub b_left_trigger: u8,
    pub b_right_trigger: u8,
    pub s_thumb_lx: i16,
    pub s_thumb_ly: i16,
    pub s_thumb_rx: i16,
    pub s_thumb_ry: i16,
}

impl InputGamepad {
    pub fn new() -> InputGamepad {
        InputGamepad {
            w_buttons: 0,
            b_left_trigger: 0,
            b_right_trigger: 0,
            s_thumb_lx: 0,
            s_thumb_ly: 0,
            s_thumb_rx: 0,
            s_thumb_ry: 0,
        }
    }
    fn from_raw(raw_gamepad: &winapi::XINPUT_GAMEPAD) -> InputGamepad {
        let w_buttons = raw_gamepad.wButtons;
        let b_left_trigger = raw_gamepad.bLeftTrigger;
        let b_right_trigger = raw_gamepad.bRightTrigger;
        let s_thumb_lx = raw_gamepad.sThumbLX;
        let s_thumb_ly = raw_gamepad.sThumbLY;
        let s_thumb_rx = raw_gamepad.sThumbRX;
        let s_thumb_ry = raw_gamepad.sThumbRY;

        InputGamepad {
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

pub struct InputState {
    pub packet_number: u32,
    pub input_gamepad: InputGamepad,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            packet_number: 0,
            input_gamepad: InputGamepad::new(),
        }
    }

    fn from_raw(raw: &winapi::XINPUT_STATE) -> InputState {
        InputState {
            packet_number: raw.dwPacketNumber,
            input_gamepad: InputGamepad::from_raw(&raw.Gamepad),
        }
    }
}

struct InputVibration {
    w_left_motor_speed: u16,
    w_right_motor_speed: u16,
}

impl InputVibration {
    pub fn new(left_motor_speed: u16, right_motor_speed: u16) -> InputVibration {
        InputVibration {
            w_left_motor_speed: left_motor_speed,
            w_right_motor_speed: right_motor_speed,
        }
    }
    fn to_raw(&self) -> winapi::XINPUT_VIBRATION {
        winapi::XINPUT_VIBRATION {
            wLeftMotorSpeed: self.w_left_motor_speed,
            wRightMotorSpeed: self.w_right_motor_speed,
        }
    }
}

pub fn set_state(user_index: u32, left_motor_speed: u16, right_motor_speed: u16) {
    let mut raw_vib = InputVibration::new(left_motor_speed, right_motor_speed).to_raw();
    let raw_result = unsafe { ffi::XInputSetState(user_index, &mut raw_vib) };
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceCapabilities {
    pub dev_type: u8,
    pub dev_subtype: u8,
    pub flags: u16,
    pub gamepad: InputGamepad, //pub vibration : Vibration
}

impl DeviceCapabilities {
    pub fn new() -> DeviceCapabilities {
        DeviceCapabilities {
            dev_type: 0,
            dev_subtype: 0,
            flags: 0,
            gamepad: InputGamepad::new(),
        }
    }

    fn from_raw(raw: &winapi::XINPUT_CAPABILITIES) -> DeviceCapabilities {
        let dev_type = raw.Type;
        let dev_subtype = raw.SubType;
        let flags = raw.Flags;
        let gamepad = InputGamepad::from_raw(&raw.Gamepad);
        DeviceCapabilities {
            dev_type: dev_type,
            dev_subtype: dev_subtype,
            flags: flags,
            gamepad: gamepad,
        }
    }
}

pub fn get_capabilities(user_index: u32, flags: u32) -> Result<DeviceCapabilities, DeviceError> {
    let raw_index: DWORD = user_index;
    let mut raw_caps: XINPUT_CAPABILITIES = unsafe { mem::zeroed() };
    let raw_result =
        unsafe { XInputGetCapabilities(raw_index, winapi::XINPUT_FLAG_GAMEPAD, &mut raw_caps) };

    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }

    Ok(DeviceCapabilities::from_raw(&raw_caps))
}

pub fn get_input_state(user_index: u32) -> Result<InputState, DeviceError> {
    let mut raw_input_state: winapi::XINPUT_STATE = unsafe { mem::zeroed() };
    let raw_user_index: DWORD = user_index;
    let raw_result = unsafe { XInputGetState(raw_user_index, &mut raw_input_state) };
    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }
    Ok(InputState::from_raw(&raw_input_state))
}
