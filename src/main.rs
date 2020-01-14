#![allow(unused_imports)]
mod corelib;
use corelib::*;
use std::thread;

mod ev3api;
use ev3api::*;

fn main() {
	let mut ev3_2 = Ev3Api::by_name("SHELI");
	ev3_2.write_mail_box("1R".into(), "100".into()).send();
	ev3_2.end();
}
