#![no_std]
#![no_main]

mod models {
    pub mod character;
    pub mod meal;
    pub mod pedometer;
    pub mod rice_ball;
}

mod character_image;
mod helper;
mod navigation;
mod router;
mod screen;

use crate::models::{
    character::Character,
    meal::Meal, pedometer::Pedometer,
    rice_ball::RiceBall
};

use crate::helper::beep;
use crate::navigation::{Direction, Navigation};
use crate::router::{Route, Router};
use crate::screen::Screen;

use accelerometer::Accelerometer;
use panic_halt as _;
use wio_terminal::{
    entry,
    hal::{clock::GenericClockController, delay::Delay, pwm::Channel},
    pac::{CorePeripherals, Peripherals},
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

    // Screenの初期化
    let screen = Screen::new(320, 240);
    Screen::draw_background(&screen, &mut display).unwrap();

    // Routerの初期化
    let mut router = Router::new(Route::Home);

    // ナビゲーションの初期化
    let mut navigation = Navigation::new(Route::Home);
    Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();

    // 歩数計の初期化
    let mut pedometer = Pedometer::new();
    Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

    // キャラクターの初期化
    let mut character = Character::new();

    // おにぎりの初期化
    let mut rice_ball = RiceBall::new();

    // 食事の初期化
    let mut meal = Meal::new();

    // ページの初期化
    Screen::draw_home_page(
        &screen,
        &mut display,
    )
    .unwrap();

    loop {
        // 下
        if switch_x.is_low().unwrap() {
            match router.route {
                Route::Home => {},
                Route::Meal => {
                    beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
                    // 食事の量を減らす
                    Meal::decrease(&mut meal);
                    Screen::draw_meal_page(
                        &screen,
                        &mut display,
                        &rice_ball,
                        &meal,
                    )
                    .unwrap();
                },
                Route::Play => {},
            }
        }

        // 右
        if switch_y.is_low().unwrap() {
            // ナビゲーションを右に移動する
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            Navigation::update(&mut navigation, Direction::Right);
            Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();
        }

        // 上
        if switch_u.is_low().unwrap() {
            match router.route {
                Route::Home => {},
                Route::Meal => {
                    beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
                    // 食事の量を増やす
                    Meal::increase(&mut meal, rice_ball.amount);
                    Screen::draw_meal_page(
                        &screen,
                        &mut display,
                        &rice_ball,
                        &meal,
                    )
                    .unwrap();
                },
                Route::Play => {},
            }
        }

        // 左
        if switch_b.is_low().unwrap() {
            // ナビゲーションを左に移動する
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            Navigation::update(&mut navigation, Direction::Left);
            Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();
        }

        // 押し込み
        if switch_z.is_low().unwrap() {
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            // 違うページを選択した状態で押し込んだとき
            if router.route != navigation.focus {
                Router::update(&mut router, navigation.focus);
                match router.route {
                    Route::Home => {
                        Screen::draw_home_page(
                            &screen,
                            &mut display,
                        )
                        .unwrap();
                    },
                    Route::Meal => {
                        Screen::draw_meal_page(
                            &screen,
                            &mut display,
                            &rice_ball,
                            &meal,
                        )
                        .unwrap();
                    },
                    Route::Play => {
                        Screen::draw_play_page(
                            &screen,
                            &mut display,
                        )
                        .unwrap();
                    },
                }
            // 同一のページを選択した状態で押し込んだとき
            } else {
                match router.route {
                    Route::Home => {
                        // TODO: ふれあい
                    },
                    Route::Meal => {
                        if meal.amount > 0 {
                            // 食事の量を決定して食べる
                            Character::eat(&mut character, &mut meal, &mut rice_ball);
                            // 3秒間食事の様子を描画する
                            Screen::draw_eat_page(&screen, &mut display).unwrap();
                            delay.delay_ms(3000u16);
                            // Homeに遷移する
                            Navigation::update(&mut navigation, Direction::Left);
                            Router::update(&mut router, Route::Home);
                            // 画面を更新する
                            Screen::draw_navigation(&screen, &mut display, Route::Home).unwrap();
                            Screen::draw_home_page(&screen, &mut display).unwrap();
                        }
                    },
                    Route::Play => {
                        // TODO: 遊ぶ
                    },
                }
            }
        }

        pedometer.update(accel.accel_norm().unwrap());
        Character::find_rice_ball(&pedometer, &mut rice_ball);
        Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

        delay.delay_ms(100u16);
    }
}
