use ffi;
use winapi;

pub struct Vibration {
    pub w_left_motor_speed: u16,
    pub w_right_motor_speed: u16,
}

impl Vibration {
    pub fn new(left_motor_speed: u16, right_motor_speed: u16) -> Vibration {
        Vibration {
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

//TODO: Return Restult<(), DeviceError>
pub fn set_vibration(user_index: u32, left_motor_speed: u16, right_motor_speed: u16) {
    let mut raw_vib = Vibration::new(left_motor_speed, right_motor_speed).to_raw();
    let raw_result = unsafe { ffi::XInputSetState(user_index, &mut raw_vib) };
}