use embassy_stm32::{
    peripherals::TIM1,
    timer::{simple_pwm::SimplePwm, Channel},
};

pub struct ServoMotor {
    pwm: SimplePwm<'static, TIM1>,
}

impl ServoMotor {
    pub fn new(pwm: SimplePwm<'static, TIM1>) -> Self {
        Self { pwm }
    }

    pub async fn next_step(&mut self) {
        self.pwm.enable(Channel::Ch2)
    }
}
