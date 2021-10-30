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

struct Pedometer {
    sample_count: i32,
    total_composite_accel: f32,
    threshold: f32,
    hysteresis: f32,
    step_count: i32,
    state: bool,
    last_state: bool,
}

impl Pedometer {
    fn new() -> Pedometer {
        Pedometer {
            sample_count: 0,
            total_composite_accel: 0.0,
            threshold: 1.5,
            hysteresis: 0.15,
            step_count: 0,
            state: false,
            last_state: false,
        }
    }
    fn count(&mut self, normalized_accel: F32x3) {
        let F32x3 { x, y, z } = normalized_accel;

        // XYZ軸の合成値
        let composite_accel = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();

        // XYZ軸の合成値を、50サンプルごとに平均したものを閾値として設定する。閾値近辺の値を誤検出しないようにヒステリシスも設定する。
        if self.sample_count < 50 {
            self.total_composite_accel += composite_accel;
            self.sample_count += 1;
        } else {
            self.threshold = self.total_composite_accel / self.sample_count as f32;
            self.hysteresis = self.threshold / 5.0;
            self.total_composite_accel = 0.0;
            self.sample_count = 0;
        }

        // 閾値の判定
        if composite_accel > (self.threshold + self.hysteresis) {
            self.state = true;
        } else if composite_accel < (self.threshold - self.hysteresis) {
            self.state = false
        }

        // 歩数をカウントする
        if !self.last_state && self.state {
            self.step_count += 1;
            self.last_state = self.state;
        } else if self.last_state && !self.state {
            self.last_state = self.state;
        }
    }
}

enum CharacterState {
    Happy,
    Angry,
}

struct Character {
    state: CharacterState,
}

impl Character {
    fn new() -> Character {
        Character {
            state: CharacterState::Happy,
        }
    }
}

struct Screen {
    width: i32,
    height: i32,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            width: 320,
            height: 240,
        }
    }
    fn draw_background<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: embedded_graphics::DrawTarget<Rgb565>,
    {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = Rgb565::WHITE)
        )
        .draw(display)?;
        Ok(())
    }
    fn draw_pedometer<T>(&self, display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
    where
        T: embedded_graphics::DrawTarget<Rgb565>,
    {
        // カウント表示エリアをクリアする
        const FONT_WIDTH: i32 = 24;
        const FONT_HEIGHT: i32 = 32;
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width - 1, FONT_HEIGHT),
            style = primitive_style!(fill_color = Rgb565::WHITE)
        )
        .draw(display)?;

        // 歩数を描画する
        let mut textbuffer = String::<U256>::new();
        write!(&mut textbuffer, "{:.2}", step_count).unwrap();

        // 座標計算用に文字列の長さを取得
        let length = textbuffer.len();
        // 右詰描画用に左の座標計算
        let left = self.width - (length as i32) * FONT_WIDTH;

        egtext!(
            text = textbuffer.as_str(),
            top_left = (left, 0),
            style = text_style!(font = Font24x32, text_color = Rgb565::BLACK)
        )
        .draw(display)?;
        Ok(())
    }
    fn draw_character<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: embedded_graphics::DrawTarget<Rgb565>,
    {
        let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
        let image = Image::new(&raw, Point::new(0, 32));
        image.draw(display)?;
        Ok(())
    }
}

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

    let mut sets = wio::Pins::new(peripherals.PORT).split();
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
    let screen = Screen::new();
    Screen::draw_background(&screen, &mut display).unwrap();

    // 歩数計の初期化
    let mut pedometer = Pedometer::new();
    Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

    // キャラクターの初期化
    let mut character = Character::new();
    Screen::draw_character(&screen, &mut display).unwrap();

    loop {
        pedometer.count(accel.accel_norm().unwrap());
        Screen::draw_pedometer(&screen, &mut display, &mut pedometer.step_count).unwrap();

        delay.delay_ms(100u16);
    }
}
