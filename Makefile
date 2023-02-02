TARGETDIR = target/aarch64-unknown-none/release
TARGETFILESDIR = targetfiles
LINKERFILESDIR = linkerfiles


qemu:
	cargo xbuild --target=$(TARGETFILESDIR)/aarch64-unknown-none.json --release --features "qemu"

raspb:
	cargo xbuild --target=$(TARGETFILESDIR)/aarch64-unknown-none-raspb.json --release --features "raspberry"
