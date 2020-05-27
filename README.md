# Instructions for getting started with embedded in Rust
## Based off [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits and libraries.



## Setup:
- Clone this repo
- Download and install [OpenOCD](http://openocd.org/) Modify `ocd.sh` or 
`ocd.ps1` to reflect the pasth you installed this into.
- This repo is configured for a `stm32f3` controller: This can be changed
by swapping out the `stm32f3xx-hal`, `f3`,  and `cortex-m` libraries in `src/main.rs`
and `Cargo.toml` as required.

## Running
- Run `ocd.sh` or `ocd.ps1`, to connect to the microcontroller. This window will
show print statements and panics.
- Run `cargo run` in a third terminal to compile your program and upload it to the MCU and
 run it. To stop it, press `ctrl` + `c` in this window,
then `quit()` to exit to the terminal.

