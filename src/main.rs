#![no_std]
#![no_main]

mod character;
mod navigation;
mod pedometer;
mod router;
mod screen;

use crate::character::{Character, CharacterState};
use crate::navigation::{Direction, Navigation};
use crate::pedometer::Pedometer;
use crate::router::{Route, Router};
use crate::screen::Screen;

use accelerometer::Accelerometer;
use panic_halt as _;
use wio_terminal::{
    entry,
    hal::{clock::GenericClockController, delay::Delay, pwm::Channel, pwm::Tcc0Pwm, time::Hertz},
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

    fn beep<P: Into<Hertz>>(
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

    // ボタンのGPIOを初期化
    let switch_y = sets.buttons.switch_y.into_floating_input(&mut sets.port);
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
    let mut character = Character::new(CharacterState::Sleep);

    // ページの初期化
    Screen::draw_page(
        &screen,
        &mut display,
        router.route,
        &mut character
    )
    .unwrap();

    loop {
        if switch_y.is_low().unwrap() {
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            Navigation::update(&mut navigation, Direction::Right);
            Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();
        }

        if switch_b.is_low().unwrap() {
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            Navigation::update(&mut navigation, Direction::Left);
            Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();
        }

        if switch_z.is_low().unwrap() {
            beep(&mut buzzer, &mut delay, 800.hz(), 200u16);
            Router::update(&mut router, navigation.focus);
            Screen::draw_page(
                &screen,
                &mut display,
                router.route,
                &mut character
            )
            .unwrap();
        }

        pedometer.update(accel.accel_norm().unwrap());
        Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

        delay.delay_ms(100u16);
    }
}
