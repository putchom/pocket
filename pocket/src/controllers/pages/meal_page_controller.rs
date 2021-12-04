use crate::helpers::buzzer::beep;
use crate::models::{
    character::Character,
    meal::Meal,
    navigation::{
        Direction,
        Navigation,
    },
    rice_ball::RiceBall,
    router::{
        Route,
        Router,
    }
};
use crate::views::{
    navigation_view::NavigationView,
    pages::{
        eat_page::EatPage,
        home_page::HomePage,
        meal_page::MealPage,
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
            }
        },
    },
};

pub struct MealPageController;

impl MealPageController {
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
        meal: &mut Meal,
        rice_ball: &mut RiceBall,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        if switch_x.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // 食事の量を減らす
            Meal::decrease(meal);
            MealPage::render(display, &rice_ball, &meal);
        }
        if switch_u.is_low().unwrap() {
            beep(buzzer, delay, 800.hz(), 200u16);
            // 食事の量を増やす
            Meal::increase(meal, rice_ball.amount);
            MealPage::render(display, &rice_ball, &meal);
        }
        if switch_z.is_low().unwrap() && navigation.focus == Route::Meal && meal.amount > 0 {
            beep(buzzer, delay, 800.hz(), 200u16);
            // 食事量の分だけ親密度UP
            Character::intimate(character, meal.amount);
            // 食べる
            Character::eat(character, meal, rice_ball);
            // 3秒間食事の様子を描画する
            EatPage::render(display);
            delay.delay_ms(3000u16);
            // Homeに遷移する
            Navigation::update(navigation, Direction::Left);
            Router::update(router, Route::Home);
            // 画面を更新する
            NavigationView::render(display, Route::Home);
            HomePage::render(display);
        }
    }
}