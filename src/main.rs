#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod stepper;
use core::borrow::BorrowMut;
use cortex_m::interrupt::Mutex;
use defmt::{println, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
    peripherals::{PA0, PA1, PA4, PA5, PB0, PC13},
};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use stepper::StepperMotor;
use {defmt_rtt as _, panic_probe as _};

static LED: StaticCell<Mutex<Output<'static, PA5>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // let led = Output::new(p.PA5, Level::Low, Speed::VeryHigh);
    // let led: &'static mut Mutex<Output<'static, PA5>> = LED.init(Mutex::new(led));
    let button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    let motor = StepperMotor::new(
        Output::new(p.PA0, Level::Low, Speed::Medium),
        Output::new(p.PA1, Level::Low, Speed::Medium),
        Output::new(p.PA4, Level::Low, Speed::Medium),
        Output::new(p.PB0, Level::Low, Speed::Medium),
        Duration::from_millis(2),
    );

    unwrap!(spawner.spawn(motor_driver(motor, button)));
}

#[embassy_executor::task]
async fn motor_driver(mut motor: StepperMotor, mut button: ExtiInput<'static, PC13>) {
    loop {
        button.wait_for_low().await;

        motor.next_step().await
    }
}
