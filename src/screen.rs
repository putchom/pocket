use crate::models::{meal::Meal, rice_ball::RiceBall};
use crate::character_image::{CharacterImage, CharacterState};
use crate::router::Route;

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

const ICON_SIZE: i32 = 32;
const FONT_WIDTH: i32 = 24;
const FONT_HEIGHT: i32 = 32;
const STATUS_BAR_HEIGHT: i32 = 32;
const BACKGROUND_COLOR: Rgb565 = Rgb565::WHITE;
const FOREGROUND_COLOR: Rgb565 = Rgb565::BLACK;

pub struct Screen {
    pub width: i32,
    pub height: i32,
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen {
            width,
            height,
        }
    }
    pub fn draw_background<T>(&self, display: &mut T) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = BACKGROUND_COLOR)
        )
        .draw(display)?;
        Ok(())
    }
    pub fn draw_navigation<T>(&self, display: &mut T, focus: Route) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // ナビゲーション表示エリアをクリアする
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (self.width / 2, STATUS_BAR_HEIGHT),
            style = primitive_style!(fill_color = BACKGROUND_COLOR)
        )
        .draw(display)?;

        let data = match focus {
            Route::Home => include_bytes!("./assets/navigation/home.raw"),
            Route::Meal => include_bytes!("./assets/navigation/meal.raw"),
            Route::Play => include_bytes!("./assets/navigation/play.raw"),
        };

        let image_data = ImageRawLE::new(
            data,
            ICON_SIZE.try_into().unwrap(),
            ICON_SIZE.try_into().unwrap(),
        );

        let point = match focus {
            Route::Home => Point::new(0, 0),
            Route::Meal => Point::new(ICON_SIZE, 0),
            Route::Play => Point::new(ICON_SIZE * 2, 0),
        };

        Image::new(&image_data, point).draw(display)?;
        Ok(())
    }
    pub fn draw_pedometer<T>(&self, display: &mut T, step_count: &mut i32) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // カウント表示エリアをクリアする
        egrectangle!(
            top_left = (self.width / 2, 0),
            bottom_right = (self.width - 1, FONT_HEIGHT),
            style = primitive_style!(fill_color = BACKGROUND_COLOR)
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
            style = text_style!(font = Font24x32, text_color = FOREGROUND_COLOR)
        )
        .draw(display)?;
        Ok(())
    }
    fn clear_page<T>(
        &self,
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // ページエリアをクリアする
        egrectangle!(
            top_left = (0, STATUS_BAR_HEIGHT),
            bottom_right = (self.width - 1, self.height - 1),
            style = primitive_style!(fill_color = BACKGROUND_COLOR)
        )
        .draw(display)?;
        Ok(())
    }
    pub fn draw_home_page<T>(
        &self,
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        self.clear_page(display)?;

        let character_image = CharacterImage::new(CharacterState::Sleep);

        Image::new(&character_image.data, character_image.point).draw(display)?;
        Ok(())
    }
    pub fn draw_meal_page<T>(
        &self,
        display: &mut T,
        rice_ball: &RiceBall,
        meal: &Meal,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        self.clear_page(display)?;

        egtext!(
            text = "How many ?",
            top_left = (0, STATUS_BAR_HEIGHT),
            style = text_style!(font = Font24x32, text_color = FOREGROUND_COLOR)
        ).draw(display)?;


        // 個数を描画する
        let mut meal_amount_textbuffer = String::<U256>::new();
        write!(&mut meal_amount_textbuffer, "{:.2}", meal.amount).unwrap();

        egtext!(
            text = meal_amount_textbuffer.as_str(),
            top_left = (0, STATUS_BAR_HEIGHT + FONT_HEIGHT),
            style = text_style!(font = Font24x32, text_color = FOREGROUND_COLOR)
        )
        .draw(display)?;

        // 持っているおにぎりの個数を描画する
        let mut rice_ball_amount_textbuffer = String::<U256>::new();
        write!(&mut rice_ball_amount_textbuffer, "MAX:{:.2}", rice_ball.amount).unwrap();

        egtext!(
            text = rice_ball_amount_textbuffer.as_str(),
            top_left = (0, STATUS_BAR_HEIGHT + FONT_HEIGHT * 2),
            style = text_style!(font = Font24x32, text_color = FOREGROUND_COLOR)
        )
        .draw(display)?;

        Ok(())
    }
    pub fn draw_eat_page<T>(
        &self,
        display: &mut T
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        self.clear_page(display)?;

        let character_image = CharacterImage::new(CharacterState::Eat);

        Image::new(&character_image.data, character_image.point).draw(display)?;

        Ok(())
    }
    pub fn draw_play_page<T>(
        &self,
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        self.clear_page(display)?;

        egtext!(
            text = "Play",
            top_left = (0, STATUS_BAR_HEIGHT),
            style = text_style!(font = Font24x32, text_color = FOREGROUND_COLOR)
        ).draw(display)?;
        Ok(())
    }
}
