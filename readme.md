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