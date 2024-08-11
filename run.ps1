$QEMU_PATH="C:\Program Files\qemu\"
$NIC_NAME="TAP"

& "$QEMU_PATH/qemu-system-x86_64w.exe" `
	-m 64M `
	-nic "tap,ifname=$NIC_NAME,script=no,downscript=no,model=e1000" `
	-drive file=target/stage0.bin,media=disk
	 
