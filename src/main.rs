use chrono::Duration;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::um::sysinfoapi::GetTickCount64;
use winapi::um::winuser::{MessageBoxW, MB_OK};
fn main() {
	let uptime = get_system_uptime();
	println!("System uptime: {} days {} hours {} minutes {} seconds", uptime.num_days(), uptime.num_hours() % 24, uptime.num_minutes() % 60, uptime.num_seconds() % 60);
	if uptime > Duration::days(3) {
		let title = "Uptime Alert";
		let message = "System uptime is greater than 3 days! Please restart your computer.";

		let title_wide: Vec<u16> = OsStr::new(title).encode_wide().chain(Some(0)).collect();
		let message_wide: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();

		unsafe {
			MessageBoxW(
				null_mut(),
				message_wide.as_ptr(),
				title_wide.as_ptr(),
				MB_OK,
			);
		}
	}
}


fn get_system_uptime() -> Duration {
	let ticks = unsafe { GetTickCount64() };
	Duration::milliseconds(ticks as i64)
}