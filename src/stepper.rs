use embassy_stm32::{
    gpio::Output,
    peripherals::{PA0, PA1, PA4, PB0},
};
use embassy_time::{Duration, Timer};

#[derive(Default)]
pub enum Step {
    #[default]
    A,
    B,
    C,
    D,
}

pub struct StepperMotor {
    p0: Output<'static, PA0>,
    p1: Output<'static, PA1>,
    p2: Output<'static, PA4>,
    p3: Output<'static, PB0>,
    delay: Duration,
    step: Step,
}

impl StepperMotor {
    pub fn new(
        p0: Output<'static, PA0>,
        p1: Output<'static, PA1>,
        p2: Output<'static, PA4>,
        p3: Output<'static, PB0>,
        delay: Duration,
    ) -> Self {
        Self {
            p0,
            p1,
            p2,
            p3,
            delay,
            step: Step::default(),
        }
    }

    pub async fn next_step(&mut self) {
        match self.step {
            Step::A => {
                self.p0.set_high();
                self.p1.set_low();
                self.p2.set_low();
                self.p3.set_low();
                Timer::after(self.delay).await;
                self.step = Step::B;
            }
            Step::B => {
                self.p0.set_low();
                self.p1.set_high();
                self.p2.set_low();
                self.p3.set_low();
                Timer::after(self.delay).await;
                self.step = Step::C;
            }
            Step::C => {
                self.p0.set_low();
                self.p1.set_low();
                self.p2.set_high();
                self.p3.set_low();
                Timer::after(self.delay).await;
                self.step = Step::D;
            }
            Step::D => {
                self.p0.set_low();
                self.p1.set_low();
                self.p2.set_low();
                self.p3.set_high();
                Timer::after(self.delay).await;
                self.step = Step::A;
            }
        }
    }
}
