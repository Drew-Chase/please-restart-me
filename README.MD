# Please Restart Me

"Please Restart Me" is a simple command-line tool designed to monitor the system's uptime. If the system has been running continuously for more than three days, the tool alerts the user and offers an option to restart the computer. Additionally, it ensures that the executable runs automatically at system startup.

## How to Run

To run this project, use the following steps:

1. Open your terminal or Command Prompt.
2. Navigate to the directory where the project is located.
3. Execute the following command:
   ```shell
   cargo run
   ```
4. Alternatively, you can download the built win64 executable from the [GitHub releases](https://github.com/Drew-Chase/please-restart-me/releases) section.

## How to Build

To build this project, follow these steps:

1. Open your terminal or Command Prompt.
2. Navigate to the directory where the project is located.
3. Run the following command to build the project:
   ```shell
   cargo build --release
   ```
   This will generate the executable in the `target/release` directory.

### Dependencies

This project relies on the following Rust crates:

- `chrono` version 0.4.38
- `winapi` version 0.3.9 with specific features enabled (sysinfoapi, winuser, shlobj, combaseapi, shobjidl, shobjidl_core, knownfolders, objbase, wtypes, winerror, minwindef, objidl)
- `libc` version 0.2.161
- `mslnk` version 0.1.8

