// #![no_std]
// #![no_main]

// use panic_halt as _;

// use cortex_m_rt::entry;
// use embassy::executor::{task, Executor};
// use embassy::time::{Duration, Timer};
// use rtt_target::{rprintln, rtt_init_print};

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();

//     rprintln!("Hello, world!");

//     let a = 2 + 5;

//     rprintln!("{}", a);

//     loop {
//         // your code goes here
//     }
// }

// #[task]
// async fn tick_periodic() -> ! {
//     loop {
//         rprintln!("tick!");
//         Timer::after(Duration::from_millis(500)).await;
//     }
// }

#![no_std]
#![no_main]
// #![feature(type_alias_impl_trait)]

use cortex_m_rt::entry;
// use embassy_executor::Spawner;
// use embassy_stm32::exti::ExtiInput;
// use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

// #[embassy_executor::main]
#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    // let mut led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    // let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    loop {
        // button.wait_for_any_edge().await;
        // if button.is_low() {
        //     led.set_high();
        // } else {
        //     led.set_low();
        // }
    }
}
