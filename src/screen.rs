use crate::router::Route;
use crate::character::Character;

use core::{convert::TryInto, fmt::Write};
use embedded_graphics::{
    egrectangle, egtext,
    fonts::Font24x32,
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style, text_style,
};
use heapless::{consts::*, String};

pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub status_bar_height: i32,
    pub background_color: Rgb565,
    pub foreground_color: Rgb565
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen {
            width,
            height,
            status_bar_height: 32,
            background_color: Rgb565::WHITE,
            foreground_color: Rgb565::BLACK
        }
    }
    pub fn draw_background<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = self.background_color)
        )
        .draw(display)?;
        Ok(())
    }
    pub fn draw_navigation<T>(&self, display: &mut T, focus: Route) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        const NAVIGATION_ICON_SIZE: i32 = 32;

        // ナビゲーション表示エリアをクリアする
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width / 2, self.status_bar_height),
            style = primitive_style!(fill_color = self.background_color)
        )
        .draw(display)?;

        let data = match focus {
            Route::Home => include_bytes!("./assets/navigation/home.raw"),
            Route::Eat => include_bytes!("./assets/navigation/eat.raw"),
        };

        let image_data = ImageRawLE::new(
            data,
            NAVIGATION_ICON_SIZE.try_into().unwrap(),
            NAVIGATION_ICON_SIZE.try_into().unwrap(),
        );

        let point = match focus {
            Route::Home => Point::new(0, 0),
            Route::Eat => Point::new(36, 0),
        };

        Image::new(&image_data, point).draw(display)?;
        Ok(())
    }
    pub fn draw_pedometer<T>(&self, display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // カウント表示エリアをクリアする
        const FONT_WIDTH: i32 = 24;
        egrectangle!(
            top_left = (self.width / 2, 0),
            bottom_right = (self.width - 1, self.status_bar_height),
            style = primitive_style!(fill_color = self.background_color)
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
            style = text_style!(font = Font24x32, text_color = self.foreground_color)
        )
        .draw(display)?;
        Ok(())
    }
    fn draw_home_page<T>(
        &self,
        display: &mut T,
        character: &mut Character
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        let image_data = Character::get_image_data(&character);
        let position = Character::get_point(&character);

        Image::new(&image_data, position).draw(display)?;
        Ok(())
    }
    fn draw_eat_page<T>(
        &self,
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        egtext!(
            text = "Eat",
            top_left = (0, self.status_bar_height),
            style = text_style!(font = Font24x32, text_color = self.foreground_color)
        ).draw(display)?;
        Ok(())
    }
    pub fn draw_page<T>(
        &self,
        display: &mut T,
        route: Route,
        character: &mut Character
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // ページエリアをクリアする
        egrectangle!(
            top_left = (0, self.status_bar_height),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = self.background_color)
        )
        .draw(display)?;

        match route {
            Route::Home => {
                self.draw_home_page(display, character)
            },
            Route::Eat => {
                self.draw_eat_page(display)
            }
        }
    }
}
