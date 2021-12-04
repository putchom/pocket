use crate::helpers::buzzer::beep;
use crate::models::{
    bet::Bet,
    character::Character,
    navigation::{
        Navigation,
        Direction,
    },
    router::{
        Route,
        Router,
    },
    shuriken::Shuriken,
};
use crate::views::{
    navigation_view::NavigationView,
    pages::{
        home_page::HomePage,
        play_page::PlayPage,
        throw_page::ThrowPage,
    },
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
        character: &mut Character,
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
            // Betの量を決定して遊ぶ
            Character::play(character, bet, shuriken);
            Character::get_reward(character, 1);
            // 手裏剣の投擲画面に遷移
            ThrowPage::render(display);
            delay.delay_ms(3000u16);
            // Homeに遷移する
            Navigation::update(navigation, Direction::Left);
            Navigation::update(navigation, Direction::Left);
            Router::update(router, Route::Home);
            // 画面を更新する
            NavigationView::render(display, Route::Home);
            HomePage::render(display);
        }
    }
}