Rust on the Digispark attiny85
======

Example to get you running rust on a Digispark attiny85 board quickly

## Build Instructions

Install `avr-gcc`, `avr-libc`, `micronucleus` and `elf2nucleus`

Assuming you already have a rust envoriment, you can run

```
sudo dnf install avr-gcc avr-libc micronucleus
cargo install elf2nucleus
```

Then you can simple run

```
cargo run --release
```

to run a simple blinking program.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
