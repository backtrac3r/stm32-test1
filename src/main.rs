#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod servo;
mod stepper;

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::OutputType,
    peripherals::TIM3,
    time::hz,
    timer::{
        simple_pwm::{PwmPin, SimplePwm},
        Channel, CountingMode,
    },
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // let led = Output::new(p.PA5, Level::Low, Speed::VeryHigh);
    // let button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    let pwm_pin = PwmPin::new_ch1(p.PA6, OutputType::PushPull);
    let pwm: SimplePwm<'static, TIM3> = SimplePwm::new(
        p.TIM3,
        Some(pwm_pin),
        None,
        None,
        None,
        hz(50),
        CountingMode::EdgeAlignedUp,
    );

    // let motor = StepperMotor::new(
    //     Output::new(p.PA0, Level::Low, Speed::Medium),
    //     Output::new(p.PA1, Level::Low, Speed::Medium),
    //     Output::new(p.PA4, Level::Low, Speed::Medium),
    //     Output::new(p.PB0, Level::Low, Speed::Medium),
    //     Duration::from_millis(2),
    // );

    // unwrap!(spawner.spawn(motor_driver(motor, button)));
    unwrap!(spawner.spawn(servo_driver(pwm)));
    // loop {
    //     Timer::after_millis(1000).await;
    // }
}

// #[embassy_executor::task]
// async fn motor_driver(mut motor: StepperMotor, mut button: ExtiInput<'static, PC13>) {
//     loop {
//         button.wait_for_low().await;

//         motor.next_step().await
//     }
// }

#[embassy_executor::task]
async fn servo_driver(mut pwm: SimplePwm<'static, TIM3>) {
    let max_duty = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);

    loop {
        pwm.set_duty(Channel::Ch1, max_duty / 15);
        Timer::after_millis(500).await;

        pwm.set_duty(Channel::Ch1, max_duty / 10);
        Timer::after_millis(500).await;
    }
}
