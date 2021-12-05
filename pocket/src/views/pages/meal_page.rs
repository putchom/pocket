use crate::helpers::screen;
use crate::models::{meal::Meal, rice_ball::RiceBall};
use core::fmt::Write;
use embedded_graphics::{
    egtext,
    fonts::Font24x32,
    pixelcolor::Rgb565,
    prelude::*,
    text_style,
};
use heapless::{consts::*, String};

pub struct MealPage;

impl MealPage {
    pub fn render<T>(
        display: &mut T,
        rice_ball: &RiceBall,
        meal: &Meal,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        egtext!(
            text = "How many ?",
            top_left = (0, screen::STATUS_BAR_HEIGHT),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        ).draw(display)?;


        // 個数を描画する
        let mut meal_amount_textbuffer = String::<U256>::new();
        write!(&mut meal_amount_textbuffer, "{:.2}", meal.amount).unwrap();

        egtext!(
            text = meal_amount_textbuffer.as_str(),
            top_left = (0, screen::STATUS_BAR_HEIGHT + screen::FONT_HEIGHT),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        )
        .draw(display)?;

        // 持っているおにぎりの個数を描画する
        let mut rice_ball_amount_textbuffer = String::<U256>::new();
        write!(&mut rice_ball_amount_textbuffer, "/{:.2}", rice_ball.amount).unwrap();

        egtext!(
            text = rice_ball_amount_textbuffer.as_str(),
            top_left = (0, screen::STATUS_BAR_HEIGHT + screen::FONT_HEIGHT * 2),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        )
        .draw(display)?;

        Ok(())
    }
}