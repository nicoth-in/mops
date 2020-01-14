pub use std::collections::VecDeque;
use std::collections::HashMap;
use std::cmp::max;

const N: i32 = 1100;

struct Move {
	pos: i32,
	view: i32,
	time: i32,
	rotation: String,
	rotation_dgr: i32,
}
impl Move {
	pub fn new() -> Self {
		Self {
			pos: 0,
			view: 0,
			time: 0,
			rotation: String::new(),
			rotation_dgr: 0,
		}
	}
}
struct Order {
	carid: i32,
	from: i32,
	to: i32,
	way: VecDeque<i32>,
	movements: Vec<Move>,
	time_start: i32,
	time_end: i32,
}
impl Order {
	pub fn new(from: i32, to: i32, car: i32, way: VecDeque<i32>, time_start: i32) -> Self {
		Self {
			carid: car,
			from: from,
			to: to,
			way: way,
			movements: Vec::new(),
			time_start: time_start,
			time_end: 0,
		}
	}
	pub fn create_movements(&mut self) {
		for _i in 0..self.way.len() {
			let x = Move::new();
			self.movements.push(x);
		}
	}
	pub fn get_movements(&mut self, time: i32) -> String {
		self.movements[(time-self.time_start) as usize].rotation.clone()
	}
	pub fn get_movements_dgr(&mut self, time: i32) -> i32 {
		self.movements[(time-self.time_start) as usize].rotation_dgr
	}
}
struct Car {
	last_pos: i32,
	command: String,
	pos: i32,
	future_end_pos: i32,
	id: i32,
	view_dgr: i32,
	view: String,
	usei: bool,
	orders: VecDeque<Order>,
}
impl Car {
	pub fn new(a: i32, v: i32, nu: i32) -> Self {
		let mut view = match v {
			0 => "NORTH",
			90 => "EAST",
			180 => "SOUTH",
			270 => "WEST",
			_ => "NONE",
		};
		Self {
			last_pos: 0,
			command: "S".into(),
			pos: a,
			future_end_pos: a,
			id: nu,
			view_dgr: v,
			view: view.to_string(),
			usei: false,
			orders: VecDeque::new(),
		}
	}
	pub fn detect_cmd(&mut self, last: i32, view: i32) {
		self.command = match (view - last) {
			0 => "F",
			90 => "R",
			180 => "A",
			270 => "L",
		    -90 => "L",
		    -180 => "A",
			-270 => "R",
			_ => "F",
		}.into();
	}
	fn get_front(&mut self) -> &mut Order {
		match self.orders.front_mut() {
			Some(ord) => ord,
			None => {panic!("");},
		}
	}
	pub fn next(&mut self) {
		self.last_pos = self.pos;
		if (self.orders.len() == 0) {
			self.usei = false;
			self.future_end_pos = self.pos;
			self.command = "S".into();
			return;
		}
		if (self.get_front().way.len() == 0) {
			self.orders.pop_front();
			return;
		}
		let new_pos = match self.get_front().way.back() {
			Some(n) => *n as i32,
			None => 0,
		};
		let delta = new_pos - self.pos;
		let last_view = self.view_dgr;
//		if (DIFFDCT.find({ pos, newPos }) != DIFFDCT.end()) {
//		    command = "F";
//		}
		match delta {
			1 => {
				self.view_dgr = 90;
				self.detect_cmd(last_view, self.view_dgr);
			},
			-1 => {
				self.view_dgr = 270;
				self.detect_cmd(last_view, self.view_dgr);
			},
			-10 => {
				self.view_dgr = 0;
				self.detect_cmd(last_view, self.view_dgr);
			},
			10 => {
				self.view_dgr = 180;
				self.detect_cmd(last_view, self.view_dgr);
			},
			_ => {
				self.command = "S".into();
			}
		}
		self.view = match self.view_dgr {
			90 => "EAST",
			180 => "SOUTH",
			270 => "WEST",
			_ => "NORTH", // 0
		}.into();
		self.pos = match self.get_front().way.back() {
			Some(n) => *n as i32,
			None => 0,
		};
		self.get_front().way.pop_back();
		if self.get_front().way.len() == 0 {
			self.orders.pop_front();
		}
		if self.orders.len() == 0 {
			self.usei = false;
			self.future_end_pos = self.pos;
		}
	}
}
type Path = Vec<Vec<i32>>;
pub struct Cores {
	cars_count: i32,
	car_list: Vec<Car>,
	path: Path,
	path_loc: Path,
	tick: i32,
}
impl Cores {
	pub fn new() -> Self {
		let mut path = Vec::new();
		for _i in 0..100 {
			path.push(Vec::new());
		}
		Self {
			cars_count: 0,
			car_list: Vec::new(),
			path: path,
			path_loc: Vec::new(),
			tick: 0,
		}
	}
	pub fn get_time(&mut self) -> i32 {
		self.tick
	}
	pub fn get_map(&mut self) -> &mut Path {
		&mut self.path
	}
	pub fn can_use(&mut self) -> bool {
		true
	}
	pub fn create_const_map(&mut self, i: i32, v: Vec<i32>) {
		self.path[i as usize] = v;
	}
	pub fn find(&mut self, v1: i32, v2: i32, num: i32, time: i32, n: i32) -> VecDeque<i32> {
		let mut data = VecDeque::new();
		let mut d = vec![N; n as usize];
		d[v1 as usize] = 0;
		let mut id = vec![0; n as usize];
		let mut q = VecDeque::new();
		q.push_back(v1);
		let mut p = vec![-1; n as usize];
		let mut timer = max(self.tick, time);
		let mut voider_ex = HashMap::new(); 

		while !(q.len() == 0) {
			let v: i32 = *q.front().unwrap_or(&0);
			q.pop_front();
			id[v as usize] = 1;
			for mut i in 0..self.path[v as usize].len() {
				let to = self.path[v as usize][i as usize];
				let length = 1;
				if self.can_use() {
					if d[to as usize] > (d[v as usize] + length) {
						d[to as usize] = d[v as usize] + length;
						if d[to as usize] == 0 {
							q.push_back(to);
						} else if id[to as usize] == 1 {
							q.push_front(to);
						}
						p[to as usize] = v;
						id[to as usize] = 1;
						timer += 1;
					}
				} else if self.path[v as usize].len() <= 2 {
					match voider_ex.get_mut(&self.path[v as usize][i as usize]) {
						Some(v) => {*v+=1;},
						None => {voider_ex.insert(self.path[v as usize][i as usize], 1);},
					};
					i -= 1;
					timer += 1;
				}
			}
		}
		println!("{:?}", p);
		let mut j = v2;
		while j != v1 {
			data.push_back(j);
			if *voider_ex.get(&j).unwrap_or(&0) != 0 {
				for i in 0..(*voider_ex.get(&j).unwrap_or(&0) as usize) {
					data.push_back(j);
				}
			}
			j = p[j as usize];
		}
		data.push_back(j);
		return data;
	}
	pub fn set_cars(&mut self, cars: Vec<(i32, i32)>) {
		self.cars_count += cars.len() as i32;
		for (i, car) in cars.iter().enumerate() {
			self.car_list.push(Car::new(car.0, car.1, i as i32));
		}
	}
	pub fn find_nearest_car(&mut self, pos: i32) -> i32 {
		for i in 0..self.car_list.len() {
			if self.car_list[i].future_end_pos == pos {
				return i as i32;
			}
		}
		return 0;
	}
	pub fn add_order(&mut self, pos_begin: i32, pos_end: i32) {
		let choosen = self.find_nearest_car(pos_begin);
		self.car_list[choosen as usize].usei = true;
		let this_path = self.find(pos_begin, pos_end, choosen, self.tick, 1000);
		self.car_list[choosen as usize].orders.push_back(Order::new(pos_begin, pos_end, choosen, this_path.clone(), self.tick));
		self.car_list[choosen as usize].future_end_pos = this_path[0];
	}
	pub fn next(&mut self) -> String {
		let mut ans = String::new();
		for car in &mut self.car_list {
			car.next();
			ans += &(car.command.clone() + " ");
		}
		self.tick += 1;
		return ans;
	}
}