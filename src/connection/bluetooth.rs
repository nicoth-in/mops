pub use std::{
    io::{Read, Write},
    time,
};
use winapi::shared::ntdef::NULL;
use std::io::{Error, ErrorKind};

// https://docs.microsoft.com/en-us/windows/win32/api/_bluetooth/index

pub unsafe fn startup() {
	use winapi::um::bluetoothapis::*;
	//use winapi::shared::ntdef::NULL;
	// First, we need to check if there is a controller
    has_controller();
    // Params to connect
    let mut search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
        dwSize: 0,

        fReturnAuthenticated: 1,
        fReturnRemembered: 0,
        fReturnUnknown: 0,
        fReturnConnected: 1,
        fIssueInquiry: 0,

        cTimeoutMultiplier: 10, // * 1.28 sec
        hRadio: NULL,
    };
    search_params.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
    let mut_search_params: *mut BLUETOOTH_DEVICE_SEARCH_PARAMS = &mut search_params;
    let const_search_params: *const BLUETOOTH_DEVICE_SEARCH_PARAMS = mut_search_params;
    // Init bt info struct
    let mut bt_info = BLUETOOTH_DEVICE_INFO::default();
    bt_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;
    let mut_bt_info: *mut BLUETOOTH_DEVICE_INFO = &mut bt_info;
    // Find first device
    let mut df = BluetoothFindFirstDevice(const_search_params, mut_bt_info);
    // Get it's name
    let mut to_str = Vec::new();
    for item in bt_info.szName.iter() {
    	let ch = *item as u8;
    	if ch != 0 {
    		to_str.push(ch);
    	}
    }
    let name = std::str::from_utf8(&to_str).unwrap();
    println!("Device {:#?} was found.", name);
    // If it is our module
    // Continue
    if name != "MEH" {
    	panic!("No MEH found");
    }
	//auth_bt(mut_bt_info);
	let mut sock = service_start();
	connect_to(sock, bt_info.Address);
	service_end(sock);
    // Receive some info
    let info = BluetoothGetDeviceInfo(NULL, mut_bt_info);
    // Close
    let closer = BluetoothFindDeviceClose(df);
    match closer {
        0 => panic!("Fail while closing."),
        1 => println!("Session closed."),
        _ => panic!("Unknown error."),
    }
}

// https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/nf-bluetoothapis-bluetoothisconnectable

pub unsafe fn has_controller() {
    let is_conn = winapi::um::bluetoothapis::BluetoothIsConnectable(winapi::shared::ntdef::NULL);
    match is_conn {
        0 => panic!("No bluetooth drivers/controllers found on this device."),
        1 => println!("Found one or more bluetooth drivers/controllers."),
        _ => panic!("Unknown error."),
    }

}

unsafe fn service_start() -> winapi::um::winsock2::SOCKET {

	let mut wrd = winapi::um::winsock2::WSADATA::default();
	let p_wrd: *mut winapi::um::winsock2::WSADATA = &mut wrd;

	let res = winapi::um::winsock2::WSAStartup(winapi::shared::minwindef::MAKEWORD(2, 2), p_wrd);

	if res != 0 {
		println!("Service start fail.");
		//return;
	} else {
		println!("Service started!");
	}

	let socket = winapi::um::winsock2::socket(winapi::shared::ws2def::AF_BTH, winapi::um::winsock2::SOCK_STREAM, winapi::um::ws2bth::BTHPROTO_RFCOMM as i32);

	if socket == winapi::um::winsock2::INVALID_SOCKET {
		println!("Error: invalid socket");
		//return;
	} else {
		println!("Socket is ok.");
	}

	socket

}
unsafe fn service_end(socket: winapi::um::winsock2::SOCKET) {
	let closed = winapi::um::winsock2::closesocket(socket);
}

unsafe fn connect_to(s: winapi::um::winsock2::SOCKET, adr: winapi::shared::bthdef::BTH_ADDR) {
	let mut bt_adr = winapi::um::ws2bth::SOCKADDR_BTH {
	    addressFamily: winapi::um::ws2bth::AF_BTH,
	    btAddr: adr,
	    serviceClassId: winapi::shared::guiddef::GUID::default(),
	    port: 1,
	};

    let mut_bt_adr: *mut winapi::um::ws2bth::SOCKADDR_BTH = &mut bt_adr;
    let const_bt_adr: *const winapi::um::ws2bth::SOCKADDR_BTH = mut_bt_adr;

	let result = winapi::um::winsock2::connect(s, const_bt_adr as *const winapi::shared::ws2def::SOCKADDR, std::mem::size_of::<winapi::um::ws2bth::SOCKADDR_BTH>() as i32);

	if result == 0 {
		println!("Connected!");
		let mut message = vec![0x0F, 0x00, 0x00, 0x01, 0x80, 0x00, 0x00, 0x94, 0x01, 0x81, 0x02, 0x82, 0xE8, 0x03, 0x82, 0xE8, 0x03];
		let mm_msg = &mut message.as_slice();
		let const_msg: *const u8 = mm_msg.as_ptr();
		winapi::um::winsock2::send(s, const_msg as *const i8, message.len() as i32, 0);
	} else {
		println!("Error!");
	}
}
unsafe fn auth_bt(bt: *mut winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO) {
	let res = winapi::um::bluetoothapis::BluetoothAuthenticateDevice(winapi::shared::ntdef::NULL as *mut winapi::shared::windef::HWND__, winapi::shared::ntdef::NULL, bt, 0 as *mut u16, 0);
	println!("{}", res);
}

pub struct Bluetooth {
	mem: BlHolder,
}

pub struct BlHolder {
	search_params: Option<Box<winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS>>,
	device_info: Option<Box<winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO>>,
	wrd: Option<Box<winapi::um::winsock2::WSADATA>>,
	socket: Option<Box<winapi::um::winsock2::SOCKET>>,
}

impl Bluetooth {
	pub fn start() -> Self {
		Self {
			mem: BlHolder {
				search_params: Option::None,
				device_info: Option::None,
				wrd: Option::None,
				socket: Option::None,
			},
		}
	}
	pub fn auth(&mut self) {
		unsafe {
			self.has_controller().unwrap();
			self.new_sp();
			self.new_btinfo();

			self.search();
		}
	}
	unsafe fn new_sp(&mut self) {
		use winapi::um::bluetoothapis::*;
		let sp = BLUETOOTH_DEVICE_SEARCH_PARAMS {
	        dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
	        fReturnAuthenticated: 1,
	        fReturnRemembered: 0,
	        fReturnUnknown: 0,
	        fReturnConnected: 1,
	        fIssueInquiry: 0,
	        cTimeoutMultiplier: 10, // * 1.28 sec
	        hRadio: NULL,
	    };
		self.mem.search_params = Option::Some(Box::new(sp));
	}
	unsafe fn new_btinfo(&mut self) {
		use winapi::um::bluetoothapis::*;
		let mut bt_info = BLUETOOTH_DEVICE_INFO::default();
		bt_info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;
		self.mem.device_info = Option::Some(Box::new(bt_info));
	}
	unsafe fn search(&mut self) {
		use winapi::um::bluetoothapis::*;
		match self.mem.search_params.as_mut() {
		    Some(v) => {
		    	let mut_search_params = Box::into_raw(v);
		    	let const_search_params: *const BLUETOOTH_DEVICE_SEARCH_PARAMS = mut_search_params;

		    },
		    None => {},
		}

	    //let mut_bt_info: *mut BLUETOOTH_DEVICE_INFO = &mut bt_info;


		//auth_bt(mut_bt_info);

		// let mut sock = service_start();
		// connect_to(sock, bt_info.Address);
		// service_end(sock);
	 //    // Receive some info
	 //    let info = BluetoothGetDeviceInfo(NULL, mut_bt_info);
	 //    // Close
	 //    let closer = BluetoothFindDeviceClose(df);
	 //    match closer {
	 //        0 => panic!("Fail while closing."),
	 //        1 => println!("Session closed."),
	 //        _ => panic!("Unknown error."),
	 //    }

	}
	unsafe fn super_search(&mut self) {
		use winapi::um::bluetoothapis::*;
		// Find first device
	    // let mut df = BluetoothFindFirstDevice(const_search_params, mut_bt_info);
	    // // Get it's name
	    // let mut to_str = Vec::new();
	    // for item in bt_info.szName.iter() {
	    // 	let ch = *item as u8;
	    // 	if ch != 0 {
	    // 		to_str.push(ch);
	    // 	}
	    // }
	    // let name = std::str::from_utf8(&to_str).unwrap();
	    // println!("Device {:#?} was found.", name);
	    // // If it is our module
	    // // Continue
	    // if name != "MEH" {
	    // 	panic!("No MEH found");
	    // }
	}
	unsafe fn has_controller(&mut self) -> std::io::Result<()> {
		use winapi::um::bluetoothapis::*;
	    let is_conn = BluetoothIsConnectable(NULL);
	    match is_conn {
	        0 => Err(Error::new(ErrorKind::Other, "No bluetooth drivers/controllers found on this device.")),
	        1 => Ok(()),
	        _ => Err(Error::new(ErrorKind::Other, "Unknown error.")),
	    }
	}
}