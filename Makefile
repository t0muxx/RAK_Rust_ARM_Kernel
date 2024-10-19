RSFLAGS = -C link-arg=--script=aarch64-rasp3b.ld

build:
	@RUSTFLAGS="$(RSFLAGS)" cargo rustc --features "qemu" --manifest-path crates/kernel/Cargo.toml
	cp ./target/aarch64-unknown-none-softfloat/debug/kernel ~/work/reverse/kernel_perso/

run: build
	@RUSTFLAGS="$(RSFLAGS)" cargo run --features "qemu"

test:
	@RUSTFLAGS="$(RSFLAGS)" cargo test -p kernel --lib --release --features qemu,test_build

