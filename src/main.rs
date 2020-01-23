#![allow(unused_imports)]
mod corelib;
use corelib::*;

mod ev3api;
use ev3api::*;

mod pathfinder;
use pathfinder::*;

fn main() {

//	let mut ev3_1 = Ev3Api::new();
//	let mut ev3_2 = Ev3Api::by_name("EV3");
//	ev3_1.bip();
//	ev3_1.end();
//	ev3_2.bip();
//	ev3_2.end();
	let mut a = Cores::new();
	a.create_const_map(1, vec![2, 11]);
	a.create_const_map(2, vec![2, 12]);
	a.create_const_map(11, vec![2, 12]);
	a.create_const_map(12, vec![2, 11]);
	if a.get_time() == 0 {
		
		
		
		a.set_cars(vec![(1, 0)]);

		a.add_order(1, 12);
		//a.add_order(12, 1);

		for _i in 0..10 {
			let r = a.next();
			println!("{}", r);
		}

		


	} else {
		
	}


}
