extern crate libc;
extern crate winapi;

mod ffi;
pub mod battery;

use std::mem;
use winapi::{ BOOL, DWORD, BYTE };

#[derive(Copy, Clone, Debug)]
pub enum DeviceError {
    DeviceNotConnected,
}

pub fn enable(enable : bool ) -> () {
    let enable : BOOL = enable as i32;

    unsafe { ffi::XInputEnable( enable ); };
}