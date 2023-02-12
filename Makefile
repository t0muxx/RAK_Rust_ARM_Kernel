TARGETDIR = target/aarch64-unknown-none/release
TARGETFILESDIR = targetfiles
LINKERFILESDIR = linkerfiles


qemu:
	cargo xbuild --target=$(TARGETFILESDIR)/aarch64-unknown-none.json --release --features "qemu"

run:
	cargo xrun --target=$(TARGETFILESDIR)/aarch64-unknown-none.json --release --features "qemu"

doc:
	cargo xdoc --target=$(TARGETFILESDIR)/aarch64-unknown-none.json --release --features "qemu" --open

test:
	cargo xtest --target=$(TARGETFILESDIR)/aarch64-unknown-none.json -p kernel --lib --release --features qemu,test_build

raspb:
	cargo xbuild --target=$(TARGETFILESDIR)/aarch64-unknown-none-raspb.json --release --features "raspberry"
