extern crate libc;
extern crate winapi;

mod ffi;
pub mod battery;
pub mod vibration;
pub mod input;

use ffi::{XInputGetCapabilities, XInputGetState};
use winapi::{BOOL, DWORD, XINPUT_CAPABILITIES};
use std::mem;
use input::Gamepad;
use std::convert::From;


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
}

impl From<winapi::XINPUT_CAPABILITIES> for DeviceCapabilities {
    fn from(raw: XINPUT_CAPABILITIES) -> DeviceCapabilities {
        DeviceCapabilities {
            dev_type : raw.Type,
            dev_subtype: raw.SubType,
            flags: raw.Flags,
            gamepad: Gamepad::from(raw.Gamepad),
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

    Ok(DeviceCapabilities::from(raw_caps))
}
