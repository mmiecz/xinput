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

    set_vibration(0, 64000, 64000).unwrap();

    use std::{thread, time};
    let ten_millis = time::Duration::from_millis(100);
    loop {
        thread::sleep(ten_millis);
        let input = get_input_state(0).unwrap();
        println!("packets: {:?}\n{:?}",
                 input.packet_number,
                 input.input_gamepad);
    }
    enable(false);
}
