#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use accelerometer::{vector::F32x3, Accelerometer};
use core::fmt::Write;
use eg::{fonts::*, pixelcolor::*, prelude::*, style::*};
use micromath::F32Ext;
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

    let mut count = 0;
    let mut total = 0.0;
    let mut threshold = 1.5;
    let mut hysteresis = 0.15;
    let mut step_count = 0;
    let mut state = false;
    let mut last_state = false;

    loop {
        let F32x3 { x, y, z } = accel.accel_norm().unwrap();

        // XYZ軸の合成値
        let composite_value = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();

        // XYZ軸の合成値を、50サンプルごとに平均したものを閾値として設定する。閾値近辺の値を誤検出しないようにヒステリシスも設定する。
        if count < 50 {
            total += composite_value;
            count += 1;
        } else {
            threshold = total / count as f32;
            hysteresis = threshold / 5.0;
            total = 0.0;
            count = 0;
        }

        // 閾値の判定
        if composite_value > (threshold + hysteresis) {
            state = true;
        } else if composite_value < (threshold - hysteresis) {
            state = false
        }

        // 歩数をカウントする
        if !last_state && state {
            step_count += 1;
            last_state = state;
        } else if last_state && !state {
            last_state = state;
        }

        writeln!(&mut serial, "Step count: {:.2}", step_count).unwrap();
        delay.delay_ms(100u16);
    }
}
