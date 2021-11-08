use crate::router::Route;

use core::{convert::TryInto, fmt::Write};
use embedded_graphics::{
    egrectangle, egtext,
    fonts::Font24x32,
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::{raw::LittleEndian, Rgb565},
    prelude::*,
    primitive_style, text_style,
};
use heapless::{consts::*, String};

pub struct Screen {
    pub width: i32,
    pub height: i32,
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen { width, height }
    }
    pub fn draw_background<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = Rgb565::WHITE)
        )
        .draw(display)?;
        Ok(())
    }
    pub fn draw_navigation<T>(&self, display: &mut T, focus: Route) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        const ICON_SIZE: i32 = 32;

        // ナビゲーション表示エリアをクリアする
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width / 2, ICON_SIZE),
            style = primitive_style!(fill_color = Rgb565::WHITE)
        )
        .draw(display)?;

        match focus {
            Route::Home => {}
            Route::Clock => {
                let clock_image_data = ImageRawLE::new(
                    include_bytes!("./assets/navigation/clock.raw"),
                    ICON_SIZE.try_into().unwrap(),
                    ICON_SIZE.try_into().unwrap(),
                );
                Image::new(&clock_image_data, Point::new(0, 0)).draw(display)?;
            }
            Route::Eat => {
                let eat_image_data = ImageRawLE::new(
                    include_bytes!("./assets/navigation/eat.raw"),
                    ICON_SIZE.try_into().unwrap(),
                    ICON_SIZE.try_into().unwrap(),
                );
                Image::new(&eat_image_data, Point::new(36, 0)).draw(display)?;
            }
        }
        Ok(())
    }
    pub fn draw_pedometer<T>(&self, display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // カウント表示エリアをクリアする
        const FONT_WIDTH: i32 = 24;
        const FONT_HEIGHT: i32 = 32;
        egrectangle!(
            top_left = (self.width / 2, 0),
            bottom_right = (self.width - 1, FONT_HEIGHT),
            style = primitive_style!(fill_color = Rgb565::WHITE)
        )
        .draw(display)?;

        let mut textbuffer = String::<U256>::new();
        write!(&mut textbuffer, "{:.2}", step_count).unwrap();

        // 座標計算用に文字列の長さを取得
        let length = textbuffer.len();

        // 右詰描画用に左の座標計算
        let left = self.width - (length as i32) * FONT_WIDTH;

        // 歩数を描画する
        egtext!(
            text = textbuffer.as_str(),
            top_left = (left, 0),
            style = text_style!(font = Font24x32, text_color = Rgb565::BLACK)
        )
        .draw(display)?;
        Ok(())
    }
    pub fn draw_character<T>(
        &self,
        display: &mut T,
        image_data: ImageRaw<Rgb565, LittleEndian>,
        position: Point,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        Image::new(&image_data, position).draw(display)?;
        Ok(())
    }
}
