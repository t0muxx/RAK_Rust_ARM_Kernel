qemu-system-aarch64 \
    -machine virt \
    -m 1024M \
    -cpu cortex-a53 \
	-semihosting \
    -kernel target/aarch64-unknown-none-softfloat/debug/kernel -d guest_errors,unimp,int,trace:bcm2835_systmr* \
    -serial stdio $1
    #-kernel target/aarch64-unknown-none/release/kernel -d trace:bcm2835_systmr*,int,trace:bcm2835_ic_read,trace:bcm2835_ic_write\
