default: run

run: rust
	qemu-system-x86_64 -hda target/boot.bin -boot order=c

mkdir:
	mkdir -p target

asm: mkdir
	nasm -felf32 boot.asm -o target/asm.o

rust: asm
	rustc src/lib.rs \
		--target i686-unknown-linux-gnu \
		-C relocation-model=static \
		-C panic=abort \
		-C lto=fat \
		-C opt-level=z \
		-C codegen-units=1 \
		-C strip=debuginfo \
		--emit=obj \
		-o target/rust.o 
	ld target/asm.o target/rust.o -T link.ld -m elf_i386 -nmagic -o target/boot.bin

