use crate::models::{
    bet::Bet,
    character::Character,
    meal::Meal,
    navigation::Navigation,
    rice_ball::RiceBall,
    router::{
        Route,
        Router,
    },
    shuriken::Shuriken,
    target::Target,
};
use crate::controllers::pages::{
    home_page_controller::HomePageController,
    meal_page_controller::MealPageController,
    play_page_controller::PlayPageController,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use wio_terminal::hal::{
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
};

use super::pages::game_page_controller::GamePageController;

pub struct PageController;

impl PageController {
    #![allow(clippy::too_many_arguments)]
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
        meal: &mut Meal,
        rice_ball: &mut RiceBall,
        shuriken: &mut Shuriken,
        target: &mut Target,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        match router.route {
            Route::Home => {
                HomePageController::watch(
                    buzzer,
                    delay,
                    switch_z,
                    navigation
                );
            },
            Route::Meal => {
                MealPageController::watch(
                    display,
                    buzzer,
                    delay,
                    switch_x,
                    switch_u,
                    switch_z,
                    navigation,
                    router,
                    character,
                    meal,
                    rice_ball
                );
            },
            Route::Play => {
                PlayPageController::watch(
                    display,
                    buzzer,
                    delay,
                    switch_x,
                    switch_u,
                    switch_z,
                    navigation,
                    router,
                    bet,
                    shuriken
                );
            },
            Route::Game => {
                GamePageController::watch(
                    display,
                    buzzer,
                    delay,
                    switch_z,
                    navigation,
                    router,
                    character,
                    bet,
                    shuriken,
                    target
                )
            }
        }
    }
}