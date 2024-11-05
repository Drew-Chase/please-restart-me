#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Duration;
use mslnk::ShellLink;
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;
use winapi::um::sysinfoapi::GetTickCount64;
use winapi::um::winuser::{MessageBoxW, MB_YESNO};

/// Main function to start the application.
fn main() {
	// Add the executable to the startup folder.
	add_executable_to_startup();

	loop {
		// Get the system uptime.
		let uptime = get_system_uptime();

		// Print the system uptime.
		println!(
			"System uptime: {} days {} hours {} minutes {} seconds",
			uptime.num_days(),
			uptime.num_hours() % 24,
			uptime.num_minutes() % 60,
			uptime.num_seconds() % 60
		);

		// Check if the system uptime is greater than 3 days.
		if uptime > Duration::days(3) {
			let title = "Uptime Alert";
			let message = "System uptime is greater than 3 days! Click 'Yes' to restart the computer.";

			// Convert strings to wide string format.
			let title_wide: Vec<u16> = OsStr::new(title).encode_wide().chain(Some(0)).collect();
			let message_wide: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();

			// Display a message box with an alert.
			unsafe {
				let response = MessageBoxW(
					null_mut(),
					message_wide.as_ptr(),
					title_wide.as_ptr(),
					MB_YESNO
				);

				// If the user clicks "Yes", restart the computer.
				if response == 6 {
					// Restart the computer.
					let result = std::process::Command::new("shutdown")
						.arg("/r")
						.arg("/t")
						.arg("0")
						.spawn();

					// Check if the command was successful.
					if let Err(e) = result {
						println!("Failed to restart computer: {:?}", e);
					}
				}
			}
		}

		// Sleep for 10 minutes before the next check.
		std::thread::sleep(std::time::Duration::from_secs(10 * 60));
	}
}

/// Get the system uptime using the `GetTickCount64` function.
///
/// # Returns
/// * `Duration` - The duration since the system started.
fn get_system_uptime() -> Duration {
	let ticks = unsafe { GetTickCount64() };
	Duration::milliseconds(ticks as i64)
}

/// Add the current executable to the Windows startup folder.
fn add_executable_to_startup() {
	// Get the path of the current executable.
	if let Some(executable) = std::env::current_exe().ok() {
		// Get the startup directory from the APPDATA environment variable.
		let startup_dir = match std::env::var("appdata") {
			Ok(val) => Path::new(&val).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
			Err(e) => panic!("Shell Startup: {:?}", e),
		};

		// Construct the full path for the startup shortcut.
		let startup_path = startup_dir.join(format!("{}.lnk", executable.file_stem().unwrap().to_string_lossy()));

		// Remove existing startup shortcut if it exists.
		if startup_path.exists() {
			fs::remove_file(&startup_path).expect("Failed to remove existing startup shortcut");
		}

		// Create a new ShellLink for the executable.
		let sl = ShellLink::new(executable).expect("Failed to create ShellLink");

		// Create the shortcut link in the startup folder.
		sl.create_lnk(startup_path).unwrap()
	}
}