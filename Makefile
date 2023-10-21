default: run

run: rust
	qemu-system-i386 -hda target/boot.bin -boot order=c -m 50M

mkdir:
	mkdir -p target

asm: mkdir
	nasm -felf32 asm/boot.asm -o target/asm.o

rust: asm
	rustc src/lib.rs \
		--target i686-unknown-linux-gnu \
		--emit obj \
		-C relocation-model=static \
		-C panic=abort \
		-C lto=fat \
		-C opt-level=z \
		-o target/rust.o 
	ld target/asm.o target/rust.o -T link.ld -m elf_i386 -nmagic -o target/boot.bin

