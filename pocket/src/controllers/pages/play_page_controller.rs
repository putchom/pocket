use crate::helpers::buzzer::beep;
use crate::models::{
    navigation::Navigation,
    router::Route,
};

use wio_terminal::{
    prelude::*,
    hal::{
        delay::*,
        pwm::*,
        gpio::{
            *,
            v2::pin::PD10,
        },
    },
};

pub struct PlayPageController;

impl PlayPageController {
    pub fn watch(
        buzzer: &mut Tcc0Pwm,
        delay: &mut Delay,
        switch_z: &Pin<PD10, Input<Floating>>,
        navigation: &Navigation,
    ) {
        if switch_z.is_low().unwrap() && navigation.focus == Route::Play {
            beep(buzzer, delay, 800.hz(), 200u16);
        }
    }
}