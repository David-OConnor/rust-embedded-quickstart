#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use hal::{
    prelude::*,
    delay::Delay,
    i2c::I2c,

    spi::{Mode, Phase, Polarity, Spi},
    stm32,
};
use stm32f3xx_hal as hal;

// Handle panics and println.
use panic_halt;
use cortex_m_semihosting::hprintln;


#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];

    // Set up microcontroller peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up gpio pins if required.
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Set up I2C if required.
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1);

    // Set up SPI if required.
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        3.mhz()
        clocks,
        &mut rcc.apb2,
    );

    // To print things to the debug console:
    hprintln!(stim, "Hello, world").unwrap();

    loop {
        delay.delay_ms(1_000);
    }
}
