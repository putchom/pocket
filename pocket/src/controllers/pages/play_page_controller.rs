use crate::helpers::buzzer::beep;
use crate::models::{
    bet::Bet,
    navigation::Navigation,
    router::{
        Route,
        Router,
    },
    shuriken::Shuriken
};
use crate::views::{
    pages::play_page::PlayPage,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use wio_terminal::{
    prelude::*,
    hal::{
        delay::*,
        pwm::*,
        gpio::{
            *,
            v2::pin::{
                PD08,
                PD20,
                PD10,
            },
        },
    },
};

pub struct PlayPageController;

impl PlayPageController {
    #![allow(clippy::too_many_arguments)]
    #[allow(unused_must_use)]
    pub fn watch<T>(
        display: &mut T,
        buzzer: &mut Tcc0Pwm,
        delay: &mut Delay,
        switch_x: &Pin<PD08, Input<Floating>>,
        switch_u: &Pin<PD20, Input<Floating>>,
        switch_z: &Pin<PD10, Input<Floating>>,
        navigation: &mut Navigation,
        router: &mut Router,
        bet: &mut Bet,
        shuriken: &mut Shuriken,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        if switch_x.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // 食事の量を減らす
            Bet::decrease(bet);
            PlayPage::render(display, &bet, &shuriken);
        }
        if switch_u.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // 食事の量を増やす
            Bet::increase(bet, shuriken.amount);
            PlayPage::render(display, &bet, &shuriken);
        }
        if switch_z.is_low().unwrap() && navigation.focus == Route::Play && bet.amount > 0 {
            beep(buzzer, delay, 800.hz(), 200u16);
            // ルーティングをGameにアップデート
            Router::update(router, Route::Game);
        }
    }
}