default: run

tap_name = virttap0

run: rust
	qemu-system-i386 \
		-m 64M \
		-nic tap,ifname=$(tap_name),script=no,downscript=no,model=e1000 \
		-drive file=target/stage0.bin,media=disk

tftp: rust
	qemu-system-x86_64 \
		-m 64M \
		-nic tftp=target,bootfile=stage0.bin 

rust:
	cargo build --release
