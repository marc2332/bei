cargo install bootimage
cargo build
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-bei/debug/bootimage-bei.bin