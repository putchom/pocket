use embedded_graphics as eg;

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

pub struct Screen {
    pub width: i32,
    pub height: i32,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            width: 320,
            height: 240,
        }
    }
    pub fn draw_background<T>(&self, display: &mut T) -> Result<(), T::Error>
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
    pub fn draw_pedometer<T>(&self, display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
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
    pub fn draw_character<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: embedded_graphics::DrawTarget<Rgb565>,
    {
        let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
        let image = Image::new(&raw, Point::new(0, 32));
        image.draw(display)?;
        Ok(())
    }
}
