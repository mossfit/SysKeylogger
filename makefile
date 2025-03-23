# Build the keylogger
build:
	cargo build --release

# Run the keylogger (replace /dev/input/eventX with your device file)
run: build
	sudo target/release/keylogger /dev/input/eventX

clean:
	cargo clean
