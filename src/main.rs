#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    embassy_stm32::init(Config::default());

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
