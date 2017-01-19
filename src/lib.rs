extern crate libc;
extern crate winapi;

mod ffi;
pub mod battery;

use std::mem;
use ffi::{ XInputGetCapabilities, XInputGetState };
use winapi::{ BOOL, DWORD, BYTE, XINPUT_CAPABILITIES };

#[derive(Copy, Clone, Debug)]
pub enum DeviceError {
    DeviceNotConnected,
}

pub fn enable(enable : bool ) -> () {
    let enable : BOOL = enable as i32;
    unsafe { ffi::XInputEnable( enable ); };
}

//pub struct Gamepad XINPUT_GAMEPAD
//pub struct Vibration XINPUT_VIBRATION
#[derive(Clone, Copy, Debug)]
pub struct InputGamepad {
    pub wButtons : u16,
    pub bLeftTrigger : u8,
    pub bRightTrigger : u8,
    pub sThumbLX : i16,
    pub sThumbLY : i16,
    pub sThumbRX : i16,
    pub sThumbRY : i16,
}

impl InputGamepad {
    pub fn new() -> InputGamepad {
        InputGamepad{ wButtons : 0, bLeftTrigger : 0, bRightTrigger : 0,
            sThumbLX : 0, sThumbLY : 0, sThumbRX : 0, sThumbRY : 0
        }
    }
    fn from_raw( raw_gamepad : &winapi::XINPUT_GAMEPAD) -> InputGamepad {
        let wButtons = raw_gamepad.wButtons;
        let bLeftTrigger = raw_gamepad.bLeftTrigger;
        let bRightTrigger = raw_gamepad.bRightTrigger;
        let sThumbLY = raw_gamepad.sThumbLY;
        let sThumbLX = raw_gamepad.sThumbLX;
        let sThumbRX = raw_gamepad.sThumbRX;
        let sThumbRY = raw_gamepad.sThumbRY;

        InputGamepad{ wButtons : wButtons, bLeftTrigger : bLeftTrigger,
            bRightTrigger : bRightTrigger, sThumbLY : sThumbLY, sThumbLX : sThumbLX,
            sThumbRX : sThumbRX, sThumbRY : sThumbRY }

    }
}

pub struct InputState {
    pub packet_number : u32,
    pub input_gamepad : InputGamepad,
}

impl InputState {
    pub fn new() -> InputState {
        InputState{ packet_number : 0, input_gamepad : InputGamepad::new() }
    }

    fn from_raw(raw : &winapi::XINPUT_STATE) -> InputState {
        InputState { packet_number : raw.dwPacketNumber,
            input_gamepad : InputGamepad::from_raw(&raw.Gamepad) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceCapabilities {
    pub typ : u8,
    pub subtype : u8,
    pub flags : u16,
    pub gamepad : InputGamepad
    //pub vibration : Vibration
}

//pub fn XInputGetCapabilities( dwUserIndex : DWORD, dwFlags : DWORD, pCapabilities : *mut XINPUT_CAPABILITIES) -> DWORD;
pub fn get_capabilities( user_index : u32, flags : u32) -> Result<DeviceCapabilities, DeviceError> {
    let raw_index : DWORD = user_index;
    let mut raw_caps : XINPUT_CAPABILITIES = unsafe { mem::zeroed() };
    let raw_result = unsafe { XInputGetCapabilities( raw_index, winapi::XINPUT_FLAG_GAMEPAD, &mut raw_caps)};

    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }

    let typ : u8 = raw_caps.Type;
    let subtype : u8 = raw_caps.SubType;
    let flags : u16 = raw_caps.Flags;
    let raw_input_gamepad : winapi::XINPUT_GAMEPAD = raw_caps.Gamepad;
    let gamepad = InputGamepad::from_raw(&raw_caps.Gamepad);

    Ok( DeviceCapabilities{ typ: typ, subtype : subtype, flags : flags, gamepad : gamepad})
}

//pub fn XInputGetState( dwUserIndex : DWORD, pState : *mut XINPUT_STATE) -> DWORD;
pub fn get_input_state(user_index : u32) -> Result<InputState, DeviceError> {
    let mut raw_input_state : winapi::XINPUT_STATE = unsafe { mem::zeroed() };
    let raw_user_index : DWORD = user_index;
    let raw_result = unsafe { XInputGetState( raw_user_index, &mut raw_input_state)};
    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }
    Ok( InputState::from_raw(&raw_input_state) )
}