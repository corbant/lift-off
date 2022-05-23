#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{time::Duration, sync::Mutex};

use tauri::{Window};

struct ActivePort(Mutex<String>);

#[derive(Clone, serde::Serialize)]
struct Payload {
	status: bool,
	device: String
}

fn main() {
  	tauri::Builder::default()
	  	.manage(ActivePort(String::new().into()))
		.invoke_handler(tauri::generate_handler![check_station_connection])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}


#[tauri::command]
fn check_station_connection(state: tauri::State<ActivePort>, window: Window) {
	//if we don't already know which port the station is connected to
	let active_port = &state.0;
	if active_port.lock().unwrap().is_empty() {
		let open_ports = serialport::available_ports().unwrap();
		for port in open_ports {
			//if the port is the one we want
			if verify_connection(port.port_name.to_string()) {
				let mut port_state = active_port.lock().unwrap();
				*port_state = port.port_name.to_string();
				//let frontend know that we have a connection
				window.emit("connection-status", Payload { status: true, device: active_port.lock().unwrap().to_string() }).unwrap();
			}
		}
		
	}
	//else we already know which device it is and we want to verify it is still connected
	else {
		if !verify_connection(active_port.lock().unwrap().to_string()) {
			//if the device is no longer connected, remove it as the active device and let the frontend know that we are no longer connected
			active_port.lock().unwrap().clear();
			window.emit("connection-status", Payload { status: false, device: String::new() }).unwrap();
		}
	}
}

//returns true if the device is a base station and connected
fn verify_connection(port: String) -> bool {
	let mut port = match serialport::new(port, 9600).timeout(Duration::from_millis(10)).open() {
		Ok(port) => port,
		Err(_) => {
			return false;
		} 
	};
	port.write("arm".as_bytes()).unwrap();
	let mut serial_buf: Vec<u8> = vec![0; 32];
	port.read(serial_buf.as_mut_slice()).unwrap();
	if String::from_utf8(serial_buf).unwrap().eq("armed") {
		return true;
	};
	return false;
}