#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use hal::{
    clocks,
    delay::Delay,
    i2c::I2c,
    pac,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
};
use stm32f3xx_hal_v2 as hal;

// Handle panics and println.
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Enable RTT debug output (printing)
    rtt_init_print!();

    // Set up CPU peripherals
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Set up microcontroller peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Set up clocks
    let mut rcc = dp.RCC.constrain();

    let mut clocks = clocks::Clocks::default();
    let clocks = clocks.make_rcc_clocks();

    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up gpio pins if required
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut input_pin = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);
    // Or:
    //        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let pin_val = input_pin.is_high().unwrap();

    let mut output_pin = gpioa // todo use the right pin
        .pa1
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    // Or:
    //        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);

    output_pin.set_high().unwrap();

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
        4.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // To print things to the debug console:
    rprintln!("Hello, world");

    loop {
        delay.delay_ms(1_000_u16);
    }
}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("Panic: {:?}", info);
    loop {}
}
