.DEFAULT_GOAL := run-win
wsl-bin:
	rustup target add x86_64-pc-windows-gnu && cargo build --release --target x86_64-pc-windows-gnu
setup-linux:
	sudo apt-get update && sudo apt-get install -y libudev-dev libgtk-3-dev libxcb-xfixes0-dev libx11-dev

run-win: wsl-bin
	./target/x86_64-pc-windows-gnu/release/thestickening.exe
