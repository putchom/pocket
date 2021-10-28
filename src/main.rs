#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use accelerometer::{vector::F32x3, Accelerometer};
use core::fmt::Write;
use eg::{
    egrectangle, egtext,
    fonts::Font24x32,
    image::{Image, ImageRawLE},
    pixelcolor::*,
    prelude::*,
    primitive_style, text_style,
};
use heapless::consts::*;
use heapless::String;
use micromath::F32Ext;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 240;

fn draw_step_count<T>(display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
where
    T: embedded_graphics::DrawTarget<Rgb565>,
{
    // カウント表示エリアをクリアする
    const FONT_WIDTH: i32 = 24;
    const FONT_HEIGHT: i32 = 32;
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (SCREEN_WIDTH - 1, FONT_HEIGHT),
        style = primitive_style!(fill_color = Rgb565::WHITE)
    )
    .draw(display)?;

    // 歩数を描画する
    let mut textbuffer = String::<U256>::new();
    write!(&mut textbuffer, "{:.2}", step_count).unwrap();

    // 座標計算用に文字列の長さを取得
    let length = textbuffer.len();
    // 右詰描画用に左の座標計算
    let left = SCREEN_WIDTH - (length as i32) * FONT_WIDTH;

    egtext!(
        text = textbuffer.as_str(),
        top_left = (left, 0),
        style = text_style!(font = Font24x32, text_color = Rgb565::BLACK)
    )
    .draw(display)?;
    Ok(())
}

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

    // ディスプレイの初期化
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        style = primitive_style!(fill_color = Rgb565::WHITE)
    )
    .draw(&mut display)
    .unwrap();
    // 歩数カウントも初期化
    draw_step_count(&mut display, &mut 0).unwrap();

    // キャラクターを描画する
    let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    let image = Image::new(&raw, Point::new(0, 32));
    image.draw(&mut display).unwrap();

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
            draw_step_count(&mut display, &mut step_count).unwrap();
        } else if last_state && !state {
            last_state = state;
        }

        delay.delay_ms(100u16);
    }
}
