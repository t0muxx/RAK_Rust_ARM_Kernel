RSFLAGS = -C link-arg=--script=aarch64-rasp3b.ld

qemu:
	@RUSTFLAGS="$(RSFLAGS)" cargo rustc --features "qemu" --release --manifest-path crates/kernel/Cargo.toml

run:
	@RUSTFLAGS="$(RSFLAGS)" cargo run --release --features "qemu"

test:
	@RUSTFLAGS="$(RSFLAGS)" cargo test -p kernel --lib --release --features qemu,test_build

raspb:
	@RUSTFLAGS="$(RSFLAGS)" cargo rustc --features "raspberry" --release --manifest-path crates/kernel/Cargo.toml
	/home/t0mux/tools/builroot/buildroot-2022.02.1/output/host/aarch64-buildroot-linux-uclibc/bin/objcopy -O binary target/aarch64-unknown-none/release/kernel kernel8.img
