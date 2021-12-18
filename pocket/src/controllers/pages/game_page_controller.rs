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
    target::Target,
};
use crate::views::{
    navigation_view::NavigationView,
    pages::{
        fail_page::FailPage,
        game_page::GamePage,
        home_page::HomePage,
        throw_page::ThrowPage,
        success_page::SuccessPage,
    },
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use models::target::TargetPosition;
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

pub struct GamePageController;

impl GamePageController {
    #![allow(clippy::too_many_arguments)]
    #[allow(unused_must_use)]
    pub fn watch<T>(
        display: &mut T,
        buzzer: &mut Tcc0Pwm,
        delay: &mut Delay,
        switch_z: &Pin<PD10, Input<Floating>>,
        navigation: &mut Navigation,
        router: &mut Router,
        character: &mut Character,
        bet: &mut Bet,
        shuriken: &mut Shuriken,
        target: &mut Target,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        // 的の位置を動かしてGame画面を描画しなおす
        Target::update(target);
        GamePage::render(display, &target.position);

        if switch_z.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);

            // 手裏剣が真ん中の的を射ているかどうかを判定する
            match target.position {
                TargetPosition::Center => {
                    // 手裏剣の投擲画面を描画
                    ThrowPage::render(display);
                    delay.delay_ms(3000u16);
                    // BETした倍の親密度UP
                    Character::intimate(character, bet.amount * 2);
                    // 手裏剣を消費してBETをリセット
                    Character::play(character, bet, shuriken);
                    // 成功画面を描画
                    SuccessPage::render(display);
                    delay.delay_ms(3000u16);
                }
                TargetPosition::Left | TargetPosition::Right => {
                    // 手裏剣の投擲画面を描画
                    ThrowPage::render(display);
                    delay.delay_ms(3000u16);
                    // 親密度をUPせず手裏剣を消費してBETをリセット
                    Character::play(character, bet, shuriken);
                    // 失敗画面を描画
                    FailPage::render(display);
                    delay.delay_ms(3000u16);
                }
            }

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