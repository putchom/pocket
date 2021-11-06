#![no_std]
#![no_main]

mod character;
mod navigation;
mod pedometer;
mod screen;

use crate::character::{Character, CharacterState};
use crate::navigation::{Navigation, NavigationFocus};
use crate::pedometer::Pedometer;
use crate::screen::Screen;

use accelerometer::Accelerometer;
use panic_halt as _;
use wio_terminal::{
    entry,
    hal::{clock::GenericClockController, delay::Delay},
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

    // ナビゲーションの初期化
    let navigation = Navigation::new(NavigationFocus::Home);
    Screen::draw_navigation(&screen, &mut display, navigation.focus).unwrap();

    // 歩数計の初期化
    let mut pedometer = Pedometer::new();
    Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

    // キャラクターの初期化
    let character = Character::new(CharacterState::Sleep);
    Screen::draw_character(
        &screen,
        &mut display,
        Character::get_image_data(&character),
        Character::get_point(&character),
    )
    .unwrap();

    loop {
        pedometer.update(accel.accel_norm().unwrap());
        Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

        delay.delay_ms(100u16);
    }
}
