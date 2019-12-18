//! A simple debounced pin example.
//! Target board: STM32F3DISCOVERY

#![deny(unsafe_code)]
#![deny(dead_code)]
#![deny(unused_imports)]
// Handle the cases where the example is build with the wrong target architecture
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {
    eprintln!("Error:");
    eprintln!("\tExample does not work with choosen target_arch.");
    eprintln!("\tBuild with for example --target thumbv7em-none-eabihf instead!");
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    cortex_m_rt::entry,
    debounced_pin::prelude::*,
    debounced_pin::ActiveHigh,
    panic_semihosting as _,
    stm32f3xx_hal::{
        delay::Delay,
        hal::digital::v2::{InputPin, OutputPin},
        prelude::*,
        stm32,
    },
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    led.set_low().unwrap();

    let user_button = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    // button is externally pulled down, and is pulled up via a button press
    let mut user_button = DebouncedInputPin::new(user_button, ActiveHigh);

    loop {
        user_button.update().unwrap();
        if user_button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        delay.delay_ms(1_u16);
    }
}
