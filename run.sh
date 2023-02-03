~/tools/qemu-7.2.0/build/qemu-system-aarch64 \
    -machine raspi3b \
    -m 1024M   \
    -cpu cortex-a53 \
    -kernel target/aarch64-unknown-none/release/kernel \
    -serial stdio $1
