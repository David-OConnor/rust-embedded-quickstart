# Instructions for getting started with embedded in Rust
## Based off [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits and libraries.


## Setup:
- Clone this repo
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [probe-run](https://ferrous-systems.com/blog/probe-run/): `cargo install probe-run`
`ocd.ps1` to reflect the pasth you installed this into.
- Install the appropriate compile target, eg `rustup target add thumbv7em-none-eabihf`
([a list of ARM Cortext targets](https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/))
- Install the [ST-LINK USB Driver](https://www.st.com/en/development-tools/stsw-link009.html, or 
one appropriate for your debug/flashing setup.

# Customize for your microcontroller (MCU)
- This repo is configured for a `stm32f3` MCU: This can be changed
by swapping out the `stm32f3xx-hal`, `f3`,  and `cortex-m` libraries in `src/main.rs`
and `Cargo.toml` to ones suitable for your MCU, and changing the `runner` argument
 in `.cargo/config`. [Reference this](https://github.com/rust-embedded/awesome-embedded-rust)
for a listing.
- Change `memory.x` in accordance with your MCU's datsheet.

## Running
- Run `cargo run` to compile and flash your program.


## Reference:
[Rust embedded book](https://rust-embedded.github.io/book)