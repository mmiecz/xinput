extern crate xinput;

use xinput::*;
use xinput::battery::*;
use xinput::vibration::*;
use xinput::input::*;

#[cfg(not(test))]
fn main() {
    enable(true);
    let batt = get_battery_information(0, 0).unwrap();
    let caps = get_capabilities(0, 0).unwrap();
    println!("{:?}\n{:?}", batt, caps);

    use std::{thread, time};
    let ten_millis = time::Duration::from_millis(100);
    let mut packet_number = 0;
    loop {
        thread::sleep(ten_millis);
        let input = get_input_state(0).unwrap();
        if packet_number != input.packet_number {
            packet_number = input.packet_number;
            set_vibration(0, 36000, 36000).unwrap();
            println!("packets: {:?}\n{:?}",
                     input.packet_number,
                     input.input_gamepad);
        }
        else {
            set_vibration(0, 0, 0).unwrap();
        }

    }
    enable(false);
}
