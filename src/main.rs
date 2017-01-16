extern crate xinput;

use xinput::*;
#[cfg(not(test))]
fn main() {
    println!("Hello World");
    let res = get_battery_information(0, 0).unwrap();
    println!("{:?}", res);
    enable(true);
}