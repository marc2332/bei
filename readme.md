# bei - a toy OS 🦀

Bei is a work-in-progress OS written in Rust 🦀. It was made thanks to [Philipp Oppermann's blog](https://os.phil-opp.com/).

Run:
```shell
cargo run
```

--- 
- cargo install bootimage
- cargo build
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-bei/debug/bootimage-bei.bin
- cargo test --package bei
- cargo run