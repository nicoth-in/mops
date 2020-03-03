pub use std::{
    io::{Read, Write},
    time,
    slice,
    mem
};
use winapi::shared::ntdef::NULL;
//use std::io::{Error, ErrorKind};
use winapi::um::bluetoothapis::*;

// https://docs.microsoft.com/en-us/windows/win32/api/_bluetooth/index

/// Bluetooth struct wich works with device (specially Ev3) via Windows 32bit API
pub struct Bluetooth {
	sock: winapi::um::winsock2::SOCKET,
	df: winapi::um::bluetoothapis::HBLUETOOTH_DEVICE_FIND,
	info: winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO
}
impl Bluetooth {
	/// Create new Bluetooth
	// NOT TESTED: impl name/first device search
	// Also, need to check multi devices
	pub fn new(dev: Option<String>) -> Bluetooth {
		unsafe {
			// First, we need to check if there is a controller
		    has_controller();
		    // Params to connect
		    let mut search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
		        dwSize: 0,
		        fReturnAuthenticated: 1, // Only known devices
		        fReturnRemembered: 0,
		        fReturnUnknown: 0,
		        fReturnConnected: 1, // And only connected
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
		    let df = BluetoothFindFirstDevice(const_search_params, mut_bt_info);
		    // Get it's name
		    let name = parse_name(bt_info.szName);
		    println!("Device {:#?} was found.", name);
		    // If it is our module Continue
		    if bt_info.ulClassofDevice == 2052 {
				match dev {
					Some(n) => {
						if name != n {
							loop {
								next_device(df, mut_bt_info);
								let name = parse_name(bt_info.szName);
								if n == name {
									break;
								}
							}
						}
						// Ok
					},
					None => {
						// Ok
					}
				}
		    } else {
				panic!("No EV3 found");
			}
			let bl = Bluetooth {
				sock: service_start(),
				df: df,
				info: bt_info,
			};
			connect_to(bl.sock, bl.info.Address);
			//if (bl.info.fConnected == 0) {
			//	connect_to(bl.sock, bl.info.Address);
			//} else {
			//	bind_to(bl.sock, bl.info.Address);
			//}
		    return bl
		}
	}
	pub fn send(&mut self, msg: &mut Vec<u8>) {
		unsafe {
			let mm_msg = &mut msg.as_slice();
			let const_msg: *const u8 = mm_msg.as_ptr();
			winapi::um::winsock2::send(self.sock, const_msg as *const i8, msg.len() as i32, 0);
		}
	}
	pub fn read(&mut self, lenof: usize) -> &mut [u8] {
		unsafe {
			let mut msg = 0;
			winapi::um::winsock2::send(self.sock, msg as *mut i8, lenof as i32, 0);
			let p: *mut i8 = &mut msg;
			let p: *mut u8 = p as *mut u8;
			let s: &mut [u8] = slice::from_raw_parts_mut(p, mem::size_of::<i8>());
			return s;
		}
	}
	pub fn stop(&mut self) {
		unsafe {
			service_end(self.sock);
		    close_bt(self.df);
		}
	}
	pub fn connect(&mut self) {
		unsafe {
			connect_to(self.sock, self.info.Address);
		}
	}
	pub fn bind(&mut self) {
		unsafe {
			bind_to(self.sock, self.info.Address);
		}
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
pub unsafe fn close_bt(df: winapi::um::bluetoothapis::HBLUETOOTH_DEVICE_FIND) {
	let closer = BluetoothFindDeviceClose(df);
    match closer {
        0 => panic!("Fail while closing."),
        1 => println!("Session closed."),
        _ => panic!("Unknown error."),
    }
}
unsafe fn service_start() -> winapi::um::winsock2::SOCKET {
	let mut wrd = winapi::um::winsock2::WSADATA::default();
	let p_wrd: *mut winapi::um::winsock2::WSADATA = &mut wrd;
	let res = winapi::um::winsock2::WSAStartup(winapi::shared::minwindef::MAKEWORD(2, 2), p_wrd);
	if res != 0 {
		panic!("Service start fail.");
	} else {
		println!("Service started!");
	}
	let socket = winapi::um::winsock2::socket(winapi::shared::ws2def::AF_BTH, winapi::um::winsock2::SOCK_STREAM, winapi::um::ws2bth::BTHPROTO_RFCOMM as i32);
	if socket == winapi::um::winsock2::INVALID_SOCKET {
		panic!("Error: invalid socket");
	} else {
		println!("Socket is ok.");
	}
	socket
}
unsafe fn service_end(socket: winapi::um::winsock2::SOCKET) {
	//winapi::um::winsock2::WSAEventSelect(socket, );
	let _closed = winapi::um::winsock2::closesocket(socket);
	winapi::um::winsock2::WSACleanup();
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
	if result != 0 {
		panic!("Error while connecting. Maybe you are already connected your device?");
	}
}

unsafe fn bind_to(s: winapi::um::winsock2::SOCKET, adr: winapi::shared::bthdef::BTH_ADDR) {
	let mut bt_adr = winapi::um::ws2bth::SOCKADDR_BTH {
	    addressFamily: winapi::um::ws2bth::AF_BTH,
	    btAddr: adr,
	    serviceClassId: winapi::shared::guiddef::GUID::default(),
	    port: 1,
	};
    let mut_bt_adr: *mut winapi::um::ws2bth::SOCKADDR_BTH = &mut bt_adr;
    let const_bt_adr: *const winapi::um::ws2bth::SOCKADDR_BTH = mut_bt_adr;
	let result = winapi::um::winsock2::bind(s, const_bt_adr as *const winapi::shared::ws2def::SOCKADDR, std::mem::size_of::<winapi::um::ws2bth::SOCKADDR_BTH>() as i32);
	if result != 0 {
		panic!("Error while binding. Maybe you are NOT connected yet?");
	}
}

fn next_device(sp: HBLUETOOTH_DEVICE_FIND, di: *mut BLUETOOTH_DEVICE_INFO) {
	unsafe {
		let u = BluetoothFindNextDevice(sp,di);
		if u == 0 {
			panic!("No EV3 found");
		}
	}
}
fn parse_name(raw_name: [u16; 248]) -> String {
	let mut to_str = Vec::new();
	for item in raw_name.iter() {
		let ch = *item as u8;
		if ch != 0 {
		    to_str.push(ch);
		}
	}
	return std::str::from_utf8(&to_str).unwrap().to_string();
}