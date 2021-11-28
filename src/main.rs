#![no_std]
#![no_main]

mod helpers {
    pub mod buzzer;
    pub mod character_image;
    pub mod screen;
}
mod models {
    pub mod character;
    pub mod meal;
    pub mod navigation;
    pub mod pedometer;
    pub mod rice_ball;
    pub mod router;
}
mod views {
    pub mod navigation_view;
    pub mod pedometer_view;
    pub mod pages {
        pub mod eat_page;
        pub mod home_page;
        pub mod meal_page;
        pub mod play_page;
    }
}
mod controllers {
    pub mod navigation_view_controller;
    pub mod pedometer_view_controller;
    pub mod page_controller;
    pub mod pages {
        pub mod home_page_controller;
        pub mod meal_page_controller;
        pub mod play_page_controller;
    }
}

use crate::helpers::screen;
use crate::models::{
    character::Character,
    meal::Meal,
    navigation::Navigation,
    pedometer::Pedometer,
    rice_ball::RiceBall,
    router::{
        Route,
        Router,
    }
};
use crate::views::{
    navigation_view::NavigationView,
    pedometer_view::PedometerView,
    pages::home_page::HomePage,
};
use crate::controllers::{
    navigation_view_controller::NavigationViewController,
    pedometer_view_controller::PedometerViewController,
    page_controller::PageController,
};

use accelerometer::Accelerometer;
use panic_halt as _;
use wio_terminal::{
    entry,
    hal::{
        clock::GenericClockController,
        delay::Delay,
        pwm::Channel
    },
    pac::{
        CorePeripherals,
        Peripherals
    },
    prelude::*,
    Pins,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    // クロックの初期化
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets = Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // ディスプレイドライバの初期化
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    // ブザーの初期化
    let mut buzzer = sets.buzzer.init(
        &mut clocks,
        peripherals.TCC0,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    let max_duty = buzzer.get_max_duty();
    buzzer.set_duty(Channel::_4, max_duty / 2);
    buzzer.disable(Channel::_4);

    // ボタンのGPIOを初期化
    let switch_x = sets.buttons.switch_x.into_floating_input(&mut sets.port);
    let switch_y = sets.buttons.switch_y.into_floating_input(&mut sets.port);
    let switch_u = sets.buttons.switch_u.into_floating_input(&mut sets.port);
    let switch_b = sets.buttons.switch_b.into_floating_input(&mut sets.port);
    let switch_z = sets.buttons.switch_z.into_floating_input(&mut sets.port);

    // UARTドライバオブジェクトの初期化
    let mut _serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 加速度センサドライバオブジェクトの初期化
    let mut accel = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // Routerの初期化
    let mut router = Router::new(Route::Home);

    // ナビゲーションの初期化
    let mut navigation = Navigation::new(Route::Home);

    // 歩数計の初期化
    let mut pedometer = Pedometer::new();

    // キャラクターの初期化
    let mut character = Character::new();

    // おにぎりの初期化
    let mut rice_ball = RiceBall::new();

    // 食事の初期化
    let mut meal = Meal::new();

    // 初期画面の描画
    screen::clear_screen(&mut display).unwrap();
    NavigationView::render(&mut display, navigation.focus).unwrap();
    PedometerView::render(&mut display, &mut pedometer.step_count).unwrap();
    HomePage::render(&mut display).unwrap();

    loop {
        NavigationViewController::watch(
            &mut display,
            &mut buzzer,
            &mut delay,
            &switch_y,
            &switch_b,
            &switch_z,
            &mut navigation,
            &mut router,
            &meal,
            &rice_ball
        );

        PedometerViewController::watch(
            &mut display,
            accel.accel_norm().unwrap(),
            &mut pedometer,
            &mut rice_ball
        );

        PageController::watch(
            &mut display,
            &mut buzzer,
            &mut delay,
            &switch_x,
            &switch_u,
            &switch_z,
            &mut navigation,
            &mut router,
            &mut character,
            &mut meal,
            &mut rice_ball
        );

        delay.delay_ms(100u16);
    }
}
