$QEMU_PATH="C:\Program Files\qemu\"

. "$QEMU_PATH/qemu-system-x86_64w" `
	-m 64M `
	-drive file=target/stage0.bin,media=disk
