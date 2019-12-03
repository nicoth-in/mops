mod corelib;
use corelib::*;

mod connection;
use connection::*;

fn main() {
	// unfinished code
	// https://le-www-live-s.legocdn.com/sc/media/files/ev3-developer-kit/lego%20mindstorms%20ev3%20communication%20developer%20kit-f691e7ad1e0c28a4cfb0835993d76ae3.pdf
	// for arg in env::args_os().skip(1) {
	//     let mut port = serial::open(&arg).unwrap();
	//     let mut msg = Vec::new();
	//     let counter = [0x00, 0x01];
	//     msg.extend_from_slice(&counter);
	//     let recieve = [0x01];
	//     msg.extend_from_slice(&recieve);
	//     let command = [0x99];
	//     msg.extend_from_slice(&command);
	//     let max_bytes_to_read = [0x00, 0x00, 0x00, 0x06]; // 6
	//     msg.extend_from_slice(&max_bytes_to_read);
	//     let test_name = "testSolA/".as_bytes();
	//     msg.extend_from_slice(&test_name);
	//     //let byte_new_line = 0x0a;
	//
	//     port.write(msg.as_slice());
	//     println!("{:?}", msg);
	// }
	let mut ev3 = Ev3Api::new(Ev3Connection::Bluetooth);
	ev3.test();
	//tar();
}
