use ffi;
use DeviceError;
use std::mem;
use winapi::{DWORD, BYTE};
use winapi;

#[derive(Copy, Clone, Debug)]
pub enum BatteryType {
    Disconnected,
    Wired,
    Alkaline,
    NiMH,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub enum BatteryLevel {
    Empty,
    Low,
    Medium,
    Full,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub struct BatteryInformation {
    pub battery_type: BatteryType,
    pub battery_level: BatteryLevel,
}

pub fn get_battery_information(user_index: u32,
                               dev_type: u8)
                               -> Result<BatteryInformation, DeviceError> {
    use winapi::xinput;
    let raw_user_index: DWORD = user_index;
    let raw_dev_type: BYTE = dev_type;
    let mut raw_battery_info: xinput::XINPUT_BATTERY_INFORMATION = unsafe { mem::uninitialized() };
    let raw_result = unsafe {
        ffi::XInputGetBatteryInformation(raw_user_index, raw_dev_type, &mut raw_battery_info)
    };
    if raw_result == winapi::winerror::ERROR_DEVICE_NOT_CONNECTED {
        return Err(DeviceError::DeviceNotConnected);
    }

    let battery_type = match raw_battery_info.BatteryType {
        xinput::BATTERY_TYPE_DISCONNECTED => BatteryType::Disconnected,
        xinput::BATTERY_TYPE_WIRED => BatteryType::Wired,
        xinput::BATTERY_TYPE_ALKALINE => BatteryType::Alkaline,
        xinput::BATTERY_TYPE_NIMH => BatteryType::NiMH,
        _ => BatteryType::Unknown,
    };

    let battery_level = match raw_battery_info.BatteryLevel {
        xinput::BATTERY_LEVEL_EMPTY => BatteryLevel::Empty,
        xinput::BATTERY_LEVEL_LOW => BatteryLevel::Low,
        xinput::BATTERY_LEVEL_MEDIUM => BatteryLevel::Medium,
        xinput::BATTERY_LEVEL_FULL => BatteryLevel::Full,
        _ => BatteryLevel::Unknown,
    };

    Ok(BatteryInformation {
        battery_type: battery_type,
        battery_level: battery_level,
    })
}
