#![allow(unused_imports)]
mod corelib;
use corelib::*;

mod ev3api;
use ev3api::*;

fn main() {

	let mut ev3_1 = Ev3Api::new();
	let mut ev3_2 = Ev3Api::by_name("EV3");
	ev3_1.bip();
	ev3_1.end();
	ev3_2.bip();
	ev3_2.end();

}
