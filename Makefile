default: run

run: rust
	qemu-system-i386 -hda target/boot.bin -boot order=c
	# qemu-system-x86_64 -fda target/boot.bin -boot order=a

asm:
	nasm -felf32 boot.asm -o target/asm.o

rust: asm
	rustc --target i686-unknown-linux-gnu --emit=obj -C relocation-model=static -C panic=abort -C lto -C opt-level=z src/lib.rs -o target/rust.o
	ld target/asm.o target/rust.o -T link.ld -m elf_i386 -nmagic -o target/boot.bin

