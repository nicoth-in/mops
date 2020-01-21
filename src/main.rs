#![allow(unused_imports)]

mod ev3api;

use std::thread;
use std::process::{Command, Stdio};
use ev3api::*;

fn main() {
	let mut ev3_2 = Ev3Api::by_name("SHELI");
	ev3_2.write_mail_box("1R".into(), "trshh\t".into()).send();
	ev3_2.end();
}
