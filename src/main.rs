// src/main.rs

// Blinks the Bluepill's onboard LED on PC13 of the STM32F103C8 microcontroller.

// Derived from stm32f1xx-hal 0.9.0 delay.rs example:
// https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/delay.rs
// and an older set of code from Jonathan Klimt which lead me to the useful flash tool:
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

// Pre-requisites:
// rustup update
// rustup target install thumbv7m-none-eabi
// cargo install cargo-flash
// git clone this project

// Usage:
// cargo build --release
// cargo flash --chip STM32F103C8 --connect-under-reset --release
// cargo clean (If updating linker script or things Cargo might not notice)

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry; // The runtime
use stm32f1xx_hal::{pac, prelude::*}; // STM32F1 specific functions (Can optionally include timer::Timer - see below)

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Configure the syst timer for a blocking delay (also check out blink.rs example)
    //let mut delay = Timer::syst(cp.SYST, &clocks).delay();
    // or
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        // Use `embedded_hal::DelayMs` trait
        delay.delay_ms(1_000_u16);
        led.set_low();
        // or use `fugit` duration units
        delay.delay(1.secs());
    }
}
