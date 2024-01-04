#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{println, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Pull},
    peripherals::PA0,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let water_sensor = ExtiInput::new(Input::new(p.PA0, Pull::Up), p.EXTI0);
    // let led = Output::new(p.PA5, Level::Low, Speed::VeryHigh);
    // let button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    // unwrap!(spawner.spawn(blinker(led, water_sensor)));
    // unwrap!(spawner.spawn(button_tracker(button)));
    unwrap!(spawner.spawn(water_handler(water_sensor, Duration::from_millis(200))));
}

// #[embassy_executor::task]
// async fn blinker(mut led: Output<'static, PA5>, mut water_sensor: ExtiInput<'static, PA0>) {
//     loop {
//         water_sensor.wait_for_any_edge().await;
//         if water_sensor.is_high() {
//             led.set_high();
//         } else {
//             led.set_low();
//         }
//     }
// }

// #[embassy_executor::task]
// async fn button_tracker(mut button: ExtiInput<'static, PC13>) {
//     loop {
//         button.wait_for_any_edge().await;

//         if button.is_low() {
//             println!("кнопка нажата");
//         }
//     }
// }

#[embassy_executor::task]
async fn water_handler(mut water_sensor: ExtiInput<'static, PA0>, interval: Duration) {
    loop {
        if water_sensor.is_high() {
            println!("обнаружена вода");
        } else {
            println!("сухо");
        }

        Timer::after(interval).await;
    }
}
