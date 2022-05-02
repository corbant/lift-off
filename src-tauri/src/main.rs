#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{time::Duration, sync::Mutex};

use tauri::Manager;

struct ActivePort(Mutex<String>);

fn main() {
  	tauri::Builder::default()
	  .setup(|app| {
		  let main_window = app.get_window("main").unwrap();
		  Ok(())
	  })
	  	.manage(ActivePort(String::new().into()))
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}


#[tauri::command]
fn connect_to_station(state: tauri::State<ActivePort>) -> bool {
	let open_ports = serialport::available_ports().unwrap();
	for port in open_ports {
		let port_name = port.port_name.to_string();
		let mut port = match serialport::new(&port_name, 9600).timeout(Duration::from_millis(10)).open() {
			Ok(port) => port,
			Err(_) => continue
		};
		port.write("ping".as_bytes()).unwrap();

		let mut serial_buf: Vec<u8> = vec![0; 32];
		port.read(serial_buf.as_mut_slice());
		if String::from_utf8(serial_buf).unwrap().eq("pong") {
			let mut active_port = state.0.lock().unwrap();
			*active_port = port_name;
			return true;
		};
	}
	return false;
}

fn check_connection(state: tauri::State<ActivePort>) -> bool {
	let mut port = match serialport::new(state.0.lock().unwrap().as_str(), 9600).timeout(Duration::from_millis(10)).open() {
		Ok(port) => port,
		Err(_) => return false
	};
	port.write("ping".as_bytes()).unwrap();

	let mut serial_buf: Vec<u8> = vec![0; 32];
	port.read(serial_buf.as_mut_slice());
	if String::from_utf8(serial_buf).unwrap().eq("pong") {
		return true;
	};
	return false;
}