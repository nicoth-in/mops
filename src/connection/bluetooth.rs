pub use std::{
    io::{Read, Write},
    time,
};

// https://docs.microsoft.com/en-us/windows/win32/bluetooth/windows-sockets-support-for-bluetooth

// pub fn test_bt() {
//
//     // Startup socket
//     unsafe {
//
//         let word = winapi::shared::minwindef::MAKEWORD(0x02, 0x02);
//         let mut wrd = winapi::um::winsock2::WSADATA {
//             wVersion: 0,
//             wHighVersion: 0,
//             iMaxSockets: 10,
//             iMaxUdpDg: 10,
//             lpVendorInfo: 0 as *mut winapi::ctypes::c_char,
//             szDescription: [0; 257],
//             szSystemStatus: [0; 129],
//         };
//         let wrd_ptr: *mut winapi::um::winsock2::WSADATA = &mut wrd;
//
//         let is_socket_supported = winapi::um::winsock2::WSAStartup(word, wrd_ptr);
//
//         // Error handle
//         let error = winapi::um::winsock2::WSAGetLastError();
//         println!("Answer - {:?}, error - {:?}", is_socket_supported, error);
//
//     }
//
//     unsafe {
//         // Create socket
//         let socket = winapi::um::winsock2::socket(winapi::shared::ws2def::AF_BTH, winapi::um::winsock2::SOCK_STREAM, winapi::um::ws2bth::BTHPROTO_RFCOMM as i32);
//
//         if socket == winapi::um::winsock2::INVALID_SOCKET {
//             println!("Error: invalid socket");
//         }
//
//         let sab = winapi::um::ws2bth::SOCKADDR_BTH {
//             addressFamily: winapi::um::ws2bth::AF_BTH,
//             btAddr: 0,
//             serviceClassId: winapi::shared::guiddef::GUID {
//                 Data1: 0,
//                 Data2: 0,
//                 Data3: 0,
//                 Data4: [0; 8]
//             },
//             port: 1,
//         };
//
//         let mut addr = winapi::shared::ws2def::SOCKADDR {
//             sa_family: winapi::um::ws2bth::AF_BTH,
//             sa_data: [0; 14],
//         };
//         addr.sa_data[0] = sab.btAddr as i8;
//         addr.sa_data[13] = sab.port as i8;
//
//         let mut_addr: *mut winapi::shared::ws2def::SOCKADDR = &mut addr;
//         std::ptr::write_bytes(mut_addr, 0, 1);
//         let const_addr: *const winapi::shared::ws2def::SOCKADDR = mut_addr;
//
//         let bind_res = winapi::um::winsock2::bind(socket, const_addr, (std::mem::size_of::<winapi::shared::ws2def::SOCKADDR>() * 2) as i32);
//         println!("SAB = {:?}", bind_res);
//         let error = winapi::um::winsock2::WSAGetLastError();
//         println!("error - {:?}", error);
//     }
//
// }

// https://docs.microsoft.com/en-us/windows/win32/api/_bluetooth/index

pub unsafe fn startup() {
    has_controller();
    let mut search_params = winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS {
        dwSize: 0,

        fReturnAuthenticated: 1,
        fReturnRemembered: 1,
        fReturnUnknown: 1,
        fReturnConnected: 1,
        fIssueInquiry: 0,

        cTimeoutMultiplier: 10, // * 1.28 sec
        hRadio: winapi::shared::ntdef::NULL,
    };
    search_params.dwSize = std::mem::size_of::<winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;

    let mut_search_params: *mut winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS = &mut search_params;
    let const_search_params: *const winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS = mut_search_params;

    let mut bt_info = winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO::default();
    bt_info.dwSize = std::mem::size_of::<winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO>() as u32;
    let mut_bt_info: *mut winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO = &mut bt_info;

    let mut df = winapi::um::bluetoothapis::BluetoothFindFirstDevice(const_search_params, mut_bt_info);

    println!("{:#?}", df);
    println!("Device: {:#?}", bt_info.fConnected);

    let closer = winapi::um::bluetoothapis::BluetoothFindDeviceClose(df);

    println!("{:#?}", closer);
}

// https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/nf-bluetoothapis-bluetoothisconnectable

pub unsafe fn has_controller() {
    let is_conn = winapi::um::bluetoothapis::BluetoothIsConnectable(winapi::shared::ntdef::NULL);
    match is_conn {
        0 => println!("No bluetooth drivers/controllers found on this device."),
        1 => println!("Found one or more bluetooth drivers/controllers."),
        _ => println!("Unknown error."),
    }

}

// Holder for windows bluetooth stuff

pub struct WinBluetoothHolder {
    bt_search_params: winapi::um::bluetoothapis::BLUETOOTH_DEVICE_SEARCH_PARAMS,
    bt_device_info: Vec<winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO>,
}
