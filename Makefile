default: run

tap_name = virttap0

run: rust
	qemu-system-i386 \
		-m 64M \
		-nic tap,ifname=$(tap_name),script=no,downscript=no,model=e1000 \
		-drive file=target/stage0.bin,media=disk -boot n

tftp: rust
	qemu-system-x86_64 \
		-m 64M \
		-nic tftp=target,bootfile=stage0.bin 

mkdir:
	mkdir -p target

asm: mkdir
	nasm -felf32 asm/boot.asm -o target/asm.o

rust: asm
	cargo build --release
