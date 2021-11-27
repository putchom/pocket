use crate::helpers::screen;
use core::fmt::Write;
use embedded_graphics::{
    egrectangle, egtext,
    fonts::Font24x32,
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style, text_style,
};
use heapless::{consts::*, String};

pub struct PedometerView;

impl PedometerView {
    pub fn render<T>(
        display: &mut T,
        step_count: &mut i32
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // カウント表示エリアをクリアする
        egrectangle!(
            top_left = (screen::SCREEN_WIDTH / 2, 0),
            bottom_right = (screen::SCREEN_WIDTH - 1, screen::FONT_HEIGHT),
            style = primitive_style!(fill_color = screen::BACKGROUND_COLOR)
        )
        .draw(display)?;

        let mut textbuffer = String::<U256>::new();
        write!(&mut textbuffer, "{:.2}", step_count).unwrap();

        // 座標計算用に文字列の長さを取得
        let length = textbuffer.len();

        // 右詰描画用に左の座標計算
        let left = screen::SCREEN_WIDTH - (length as i32) * screen::FONT_WIDTH;

        // 歩数を描画する
        egtext!(
            text = textbuffer.as_str(),
            top_left = (left, 0),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        )
        .draw(display)?;
        Ok(())
    }
}