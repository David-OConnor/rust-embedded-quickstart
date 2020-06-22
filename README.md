# Instructions for getting started with embedded in Rust
## Based off [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits and libraries.



## Setup:
- Clone this repo
- Install [Rust](https://www.rust-lang.org/tools/install)
- Download and install [OpenOCD](http://openocd.org/) Modify `ocd.sh` or 
`ocd.ps1` to reflect the pasth you installed this into.
- Install the appropriate compile target, eg `rustup target add thumbv7em-none-eabi`
([a list of ARM Cortext targets](https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/))
- Install the [ST-LINK USB Driver](https://www.st.com/en/development-tools/stsw-link009.html)
- (Install OCD linker??)

# Customize for your microcontroller (MCU)
- This repo is configured for a `stm32f3` MCU: This can be changed
by swapping out the `stm32f3xx-hal`, `f3`,  and `cortex-m` libraries in `src/main.rs`
and `Cargo.toml` to ones suitablef or your MCU. [Reference this](https://github.com/rust-embedded/awesome-embedded-rust)
for a listing.
- Change `memory.x` in accordance with your MCU's datsheet.

## Running
- Run `ocd.sh` or `ocd.ps1`, to connect to the microcontroller. This terminal will
show print statements and panics.
- Run `cargo run` in a different terminal to compile your program and upload it to the MCU and
 run it. To stop it, press `ctrl` + `c`, then `quit` to exit to the terminal.


## Reference:
[Rust embedded book](https://rust-embedded.github.io/book)