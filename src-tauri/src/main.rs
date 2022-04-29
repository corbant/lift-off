#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::time::Duration;

struct ActivePort(String);

fn main() {
  	tauri::Builder::default()
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}


#[tauri::command]
fn connect_to_station(state: tauri::State<ActivePort>) {
	let open_ports = serialport::available_ports().unwrap();
	for port in open_ports {
		let port_name = port.port_name.to_string();
		let mut port = match serialport::new(port_name, 9600).timeout(Duration::from_millis(10)).open() {
			Ok(port) => port,
			Err(_) => continue
		};
		port.write("identify".as_bytes()).unwrap();

		let mut serial_buf: Vec<u8> = vec![0; 32];
		port.read(serial_buf.as_mut_slice());
		if(String::from_utf8(serial_buf)).eq("BaseStation") {
			state.0 = port_name;
		};

	}
}