#!/bin/bash

~/tools/qemu-7.2.0/build/qemu-system-aarch64 \
    -machine raspi3b \
    -m 1024M   \
    -cpu cortex-a53 \
    -semihosting \
    -kernel $1 \
    -serial stdio 
