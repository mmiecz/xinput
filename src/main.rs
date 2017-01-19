extern crate xinput;

use xinput::*;
use xinput::battery::*;
#[cfg(not(test))]
fn main() {
    enable(true);
    let batt = get_battery_information(0, 0).unwrap();
    let caps = get_capabilities(0, 0).unwrap();
    println!("{:?}\n{:?}", batt, caps);

    use std::{thread, time};
    let ten_millis = time::Duration::from_millis(10);
    loop {
        thread::sleep(ten_millis);
        let input = get_input_state(0).unwrap();
        println!( "packets: {:?}\n{:?}", input.packet_number, input.input_gamepad );
    }
}