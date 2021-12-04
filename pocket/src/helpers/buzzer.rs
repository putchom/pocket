use panic_halt as _;
use wio_terminal::{
    hal::{delay::Delay, pwm::Channel, pwm::Tcc0Pwm, time::Hertz},
    prelude::*,
};

pub fn beep<P: Into<Hertz>>(
    buzzer_pwm: &mut Tcc0Pwm,
    delay: &mut Delay,
    frequency: P,
    duration_ms: u16,
) {
    buzzer_pwm.set_period(frequency.into());
    buzzer_pwm.enable(Channel::_4);
    delay.delay_ms(duration_ms);
    buzzer_pwm.disable(Channel::_4);
}