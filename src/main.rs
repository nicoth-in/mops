#![allow(unused_imports)]
mod corelib;
use corelib::*;

mod ev3api;
use ev3api::*;

mod pathfinder;
use pathfinder::*;

use std::io::{self, Read};
use std::{thread, time};

fn main() {

	let mut a = Cores::new();

	a.create_unic_map(11, 20);
	a.create_unic_map(18, 29);
	a.create_unic_map(60, 71);
	a.create_unic_map(78, 69);
	a.create_unic_map(36, 45);

	a.create_const_map(11, vec![20, 12]);
	a.create_const_map(12, vec![11, 13]);
	a.create_const_map(13, vec![12, 23, 14]);
	a.create_const_map(14, vec![13, 24, 15]);
	a.create_const_map(15, vec![14, 16]);
	a.create_const_map(16, vec![15, 26, 17]);
	a.create_const_map(17, vec![16, 27, 18]);
	a.create_const_map(18, vec![17, 29]);

	a.create_const_map(20, vec![30, 11]);
	a.create_const_map(22, vec![23]);
	a.create_const_map(23, vec![13, 33, 22]);
	a.create_const_map(24, vec![14]);
	a.create_const_map(26, vec![16]);
	a.create_const_map(27, vec![17, 37]);
	a.create_const_map(29, vec![18, 39]);

	a.create_const_map(30, vec![40, 31, 20]);
	a.create_const_map(31, vec![30, 32]);
	a.create_const_map(32, vec![31, 33]);
	a.create_const_map(33, vec![23, 43, 32, 34]);
	a.create_const_map(34, vec![33]);
	a.create_const_map(36, vec![45, 26]);
	a.create_const_map(37, vec![47, 27]);
	a.create_const_map(39, vec![29, 49]);

	a.create_const_map(40, vec![30, 50]);
	a.create_const_map(42, vec![43]);
	a.create_const_map(43, vec![33, 42, 44, 53]);
	a.create_const_map(44, vec![43, 45]);
	a.create_const_map(45, vec![44, 36]);
	a.create_const_map(47, vec![37, 57]);
	a.create_const_map(48, vec![49]);
	a.create_const_map(49, vec![48, 59, 39]);

	a.create_const_map(50, vec![40, 60, 51]);
	a.create_const_map(51, vec![50, 52]);
	a.create_const_map(52, vec![53, 51]);
	a.create_const_map(53, vec![52, 43, 63, 54]);
	a.create_const_map(54, vec![53]);
	a.create_const_map(56, vec![57]);
	a.create_const_map(57, vec![56, 58, 67, 47]);
	a.create_const_map(58, vec![57, 59]);
	a.create_const_map(59, vec![58, 49, 69]);

	a.create_const_map(60, vec![50, 71]);
	a.create_const_map(63, vec![53, 73]);
	a.create_const_map(65, vec![75]);
	a.create_const_map(67, vec![57, 77]);
	a.create_const_map(68, vec![69]);
	a.create_const_map(69, vec![59, 68, 78]);

	a.create_const_map(71, vec![60, 72]);
	a.create_const_map(72, vec![71, 73]);
	a.create_const_map(73, vec![72, 63, 74]);
	a.create_const_map(74, vec![73, 75]);
	a.create_const_map(75, vec![74, 76, 65]);
	a.create_const_map(76, vec![75, 77]);
	a.create_const_map(77, vec![76, 67, 78]);
	a.create_const_map(78, vec![77, 69]);

	if a.get_time() == 0 {
		
		
		let mut ev3 = Ev3Api::by_name("SERVO");

		//let mut buffer = String::new();
    	//io::stdin().read_to_string(&mut buffer).unwrap();
    	
		a.set_cars(vec![(11, 0)]);

		a.add_order(42, 54);

		for _i in 0..10 {

			let r = a.next();
			println!("{:?}", r[0]);
			ev3.write_mail_box_f32("1R".into(), r[0]).send();

			// For now
			let timer = time::Duration::from_millis(2000);
			thread::sleep(timer);

		}

		ev3.end();

	}

}
