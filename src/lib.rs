extern crate libc;
extern crate winapi;

mod ffi;
pub mod battery;
pub mod vibration;
pub mod input;

use std::mem;
use ffi::{XInputGetCapabilities, XInputGetState};
use input::Gamepad;
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

pub struct InputState {
    pub packet_number: u32,
    pub input_gamepad: Gamepad,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            packet_number: 0,
            input_gamepad: Gamepad::new(),
        }
    }

    fn from_raw(raw: &winapi::XINPUT_STATE) -> InputState {
        InputState {
            packet_number: raw.dwPacketNumber,
            input_gamepad: Gamepad::from_raw(&raw.Gamepad),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceCapabilities {
    pub dev_type: u8,
    pub dev_subtype: u8,
    pub flags: u16,
    pub gamepad: Gamepad,
}

impl DeviceCapabilities {
    pub fn new() -> DeviceCapabilities {
        DeviceCapabilities {
            dev_type: 0,
            dev_subtype: 0,
            flags: 0,
            gamepad: Gamepad::new(),
        }
    }

    fn from_raw(raw: &winapi::XINPUT_CAPABILITIES) -> DeviceCapabilities {
        let dev_type = raw.Type;
        let dev_subtype = raw.SubType;
        let flags = raw.Flags;
        let gamepad = Gamepad::from_raw(&raw.Gamepad);
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
