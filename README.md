# rust-bluepill
STM32F103C8 "Bluepill" LED blink example in Rust. Toggles the onboard LED on PC13 every second.

Derived from stm32f1xx-hal 0.9.0 [delay.rs](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/delay.rs) example and an older set of code/instructions from [Jonathan Klimt](https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/) which lead me to the useful flash tool used below.

## Pre-requisites
**Update rust**

    rustup update

**Install target**

    rustup target install thumbv7m-none-eabi

**Install flash tool (For ST-Link V2 and clones)**

    cargo install cargo-flash

**Get the code**

    git clone https://github.com/mmmstew/rust-bluepill.git

**Jump into the folder**

    cd rust-bluepill


## Usage
**Build the project**

    cargo build --release

**Flash to target**

    cargo flash --chip STM32F103C8 --connect-under-reset --release

**If updating linker script or things Cargo might not notice, it can be helpful to follow up with**

    cargo clean
