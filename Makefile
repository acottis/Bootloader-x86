default: run

run: rust
	qemu-system-i386 \
		-hda target/boot.bin \
		-boot order=c \
		-m 50M \
		-nic model=e1000


mkdir:
	mkdir -p target

asm: mkdir
	nasm -felf32 asm/boot.asm -o target/asm.o

rust: asm
	cargo build --release
