# SysKeylogger

Systematic keylogger. For educational purposes ONLY. Author is not responsible for any misuse or damage.

# Keylogger Project

**Disclaimer:**  

This project is provided for educational only. Do **NOT** run any keylogger on systems where you do not have explicit permission. The author is not responsible for any misuse.

## Overview

This project implements a keylogger in two ways:

1. **Rust Implementation:**  
   A keylogger built with Rust using the [`evdev`](https://crates.io/crates/evdev) crate to capture raw key events from a Linux input device (e.g., `/dev/input/eventX`).

2. **Shell Implementation:**  
   A simple keylogger written in Bash using the `xinput` command to log key events in an X11 environment.

## Requirements

- **Rust Keylogger:**
  - Linux OS with access to input devices.
  - Root privileges (or appropriate udev rules) for reading `/dev/input/eventX`.
  - Rust toolchain installed ([rustup](https://rustup.rs/)).

- **Shell Keylogger:**
  - X11 environment.
  - `xinput` installed.

## Setup & Usage

Build with:
`make build`

Run (after replacing `/dev/input/eventX` with the proper input device):
```bash
make run
```
Clean build artifacts with:
```make clean```

Replace `/dev/input/eventX` in the Makefile or when running manually with the correct input device file:

```bash
sudo target/release/keylogger_rust /dev/input/eventX
```
You can list input devices by running (as root):

```ls /dev/input/event*```

**Shell Keylogger**

Make the Script Executable:
```bash
chmod +x keylogger.sh
```
Identify the Device ID:
```xinput list```
Run the Keylogger:
```bash
sudo ./keylogger.sh <device-id>
```
Replace `<device-id> with the device ID obtained from xinput list.
### Rust Keylogger

**Build the Project:**
```bash
make build
```

## Ethical Use Notice
Use this keylogger only for educational purposes **ONLY**. Unauthorized keylogging is illegal and unethical. Author is nor responsible for any misuse or damage.

License

[MIT License]().
