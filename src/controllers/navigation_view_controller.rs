use crate::helpers::buzzer::beep;
use crate::models::{
    meal::Meal,
    rice_ball::RiceBall,
};
use crate::views::{
    navigation_view::NavigationView,
    pages::{
        home_page::HomePage,
        meal_page::MealPage,
        play_page::PlayPage,
    },
};
use crate::navigation::{Navigation, Direction};
use crate::router::{Router, Route};

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
                PD09,
                PD12,
                PD10,
            }
        },
    },
};

pub struct NavigationViewController;

impl NavigationViewController {
    #![allow(clippy::too_many_arguments)]
    #[allow(unused_must_use)]
    pub fn watch<T>(
        display: &mut T,
        buzzer: &mut Tcc0Pwm,
        delay: &mut Delay,
        switch_y: &Pin<PD09, Input<Floating>>,
        switch_b: &Pin<PD12, Input<Floating>>,
        switch_z: &Pin<PD10, Input<Floating>>,
        navigation: &mut Navigation,
        router: &mut Router,
        meal: &Meal,
        rice_ball: &RiceBall,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        if switch_y.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // ナビゲーションを右に移動する
            Navigation::update(navigation, Direction::Right);
            NavigationView::render(display, navigation.focus);
        }

        if switch_b.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // ナビゲーションを左に移動する
            Navigation::update(navigation, Direction::Left);
            NavigationView::render(display, navigation.focus);
        }

        // 現在のページではないナビゲーションを指し示していてかつ、Zが押されたとき
        if navigation.focus != router.route && switch_z.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            Router::update(router, navigation.focus);
            match router.route {
                Route::Home => {
                    HomePage::render(display);
                },
                Route::Meal => {
                    MealPage::render(display, &rice_ball, &meal);
                },
                Route::Play => {
                    PlayPage::render(display);
                },
            }
        }
    }
}