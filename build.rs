use std::process::{Command, Stdio};

fn main() {
	let output = Command::new("C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC\\Tools\\MSVC\\14.23.28105\\bin\\Hostx64\\x64\\cl.exe")
                     .arg("C:\\Users\\Ученик\\Desktop\\package\\pathfinder\\src\\lib.cpp")
                     .output()
                     .expect("Failed to execute command");
}

