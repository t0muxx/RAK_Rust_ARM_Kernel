cd $(pwd)

~/tools/qemu-7.2.0/build/qemu-system-aarch64 \
    -machine virt \
    -d int \
    -m 1024M \
    -cpu cortex-a53 \
    -kernel target/aarch64-unknown-none/release/kernel -d guest_errors,unimp,int,trace:bcm2835_systmr* \
    -serial stdio $1
    #-kernel target/aarch64-unknown-none/release/kernel -d trace:bcm2835_systmr*,int,trace:bcm2835_ic_read,trace:bcm2835_ic_write\
