mod corelib;
use corelib::*;

mod connection;
use connection::*;

fn main() {

	let mut ev3 = Ev3Api::new();
	ev3.bip();
	ev3.end();

}
