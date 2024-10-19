#!/bin/bash

qemu-system-aarch64 \
    -machine virt \
    -m 1024M   \
    -cpu cortex-a53 \
    -semihosting \
    -kernel $1 \
    -serial stdio \
	-d int,unimp,guest_errors 
