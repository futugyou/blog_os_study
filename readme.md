1.
rustc --version --verbose
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf

2.
rustup override set nightly
rustup component add rust-src
cargo install cargo-xbuild
cargo xbuild
cargo install bootimage --version "^0.7.7"
rustup component add llvm-tools-preview
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=\target\x86_64-blog_os\debug\bootimage-blog_os_study.bin


// qemu: could not load PC BIOS 'bios-256k.bin'
qemu-system-x86_64 -drive format=raw,file=target\x86_64-blog_os\debug\deps\bootimage-blog_os_study-e5d336880d8c1d8a.bin -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none -L E:\qemu
qemu-system-x86_64 -drive format=raw,file=target\x86_64-blog_os\debug\bootimage-blog_os_study.bin -L E:\qemu