use crate::helpers::screen;
use crate::models::{bet::Bet, shuriken::Shuriken};
use core::fmt::Write;
use embedded_graphics::{
    egtext,
    fonts::Font24x32,
    pixelcolor::Rgb565,
    prelude::*,
    text_style,
};
use heapless::{consts::*, String};

pub struct PlayPage;

impl PlayPage {
    pub fn render<T>(
        display: &mut T,
        bet: &Bet,
        shuriken: &Shuriken,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        egtext!(
            text = "How much ?",
            top_left = (0, screen::STATUS_BAR_HEIGHT),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        ).draw(display)?;

        // BETする手裏剣の個数を描画する
        let mut bet_amount_textbuffer = String::<U256>::new();
        write!(&mut bet_amount_textbuffer, "{:.2}", bet.amount).unwrap();

        egtext!(
            text = bet_amount_textbuffer.as_str(),
            top_left = (0, screen::STATUS_BAR_HEIGHT + screen::FONT_HEIGHT),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        )
        .draw(display)?;

        // 持っている手裏剣の個数を描画する
        let mut shuriken_amount_textbuffer = String::<U256>::new();
        write!(&mut shuriken_amount_textbuffer, "/{:.2}", shuriken.amount).unwrap();

        egtext!(
            text = shuriken_amount_textbuffer.as_str(),
            top_left = (0, screen::STATUS_BAR_HEIGHT + screen::FONT_HEIGHT * 2),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        )
        .draw(display)?;

        Ok(())
    }
}