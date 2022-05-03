#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{time::Duration, sync::Mutex, thread};

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
		.invoke_handler(tauri::generate_handler![start_connection_service])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn start_connection_service(state: tauri::State<ActivePort> , window: Window) {
	thread::spawn( ||  {
		//check port connection status every second
		loop {
			check_station_connection(state, window);
			thread::sleep(Duration::from_millis(1000));
		}
	});
}

fn check_station_connection(state: tauri::State<ActivePort>, window: Window) {
	//if we don't already know which port the station is connected to
	if state.0.lock().unwrap().is_empty() {
		let open_ports = serialport::available_ports().unwrap();
		for port in open_ports {
			//if the port is the one we want
			if verify_connection(port.port_name.to_string()) {
				let mut active_port = state.0.lock().unwrap();
				*active_port = port.port_name.to_string();
				//let frontend know that we have a connection
				window.emit("connection-status", Payload { status: true, device: state.0.lock().unwrap().to_string() });
			}
		}
		
	}
	//else we already know which device it is and we want to verify it is still connected
	else {
		if !verify_connection(state.0.lock().unwrap().to_string()) {
			//if the device is no longer connected, remove it as the active device and let the frontend know that we are no longer connected
			state.0.lock().unwrap().clear();
			window.emit("connection-status", Payload { status: false, device: String::new() });
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
	port.write("ping".as_bytes()).unwrap();
	let mut serial_buf: Vec<u8> = vec![0; 32];
	port.read(serial_buf.as_mut_slice());
	if String::from_utf8(serial_buf).unwrap().eq("pong") {
		return true;
	};
	return false;
}