#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let _p = embassy_stm32::init(config);

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}

// #[task]
// async fn tick_periodic() -> ! {
//     loop {
//         rprintln!("tick!");
//         Timer::after(Duration::from_millis(500)).await;
//     }
// }

// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]

// use embassy_executor::Spawner;
// use embassy_stm32::exti::ExtiInput;
// use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
// use {defmt_rtt as _, panic_probe as _};

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let p = embassy_stm32::init(Default::default());
//     let mut led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
//     let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

//     loop {
//         button.wait_for_any_edge().await;
//         if button.is_low() {
//             led.set_high();
//         } else {
//             led.set_low();
//         }
//     }
// }
