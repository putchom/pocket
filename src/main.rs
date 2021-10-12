#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use accelerometer::{vector::F32x3, Accelerometer};
use core::fmt::Write;
use eg::{fonts::*, pixelcolor::*, prelude::*, style::*};
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    // クロックを初期化する
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // ディスプレイドライバを初期化する
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

    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 加速度センサドライバオブジェクトを初期化する
    let mut accel = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 画面に「Hello world!」と表示する
    Text::new("Hello world!", Point::new(30, 30))
        .into_styled(TextStyle::new(Font12x16, Rgb565::BLACK))
        .draw(&mut display)
        .unwrap();

    // 1秒ごとに加速度センサから読み取った値をシリアルに出力する
    loop {
        let F32x3 { x, y, z } = accel.accel_norm().unwrap();
        writeln!(&mut serial, "X:{:.2}, Y:{:.2}, X:{:.2}", x, y, z).unwrap();
        delay.delay_ms(1000u16);
    }
}
