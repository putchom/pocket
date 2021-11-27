use crate::helpers::screen;
use embedded_graphics::{
    egtext,
    fonts::Font24x32,
    pixelcolor::Rgb565,
    prelude::*,
    text_style,
};

pub struct PlayPage;

impl PlayPage {
    pub fn render<T>(
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        egtext!(
            text = "Play",
            top_left = (0, screen::STATUS_BAR_HEIGHT),
            style = text_style!(font = Font24x32, text_color = screen::FOREGROUND_COLOR)
        ).draw(display)?;
        Ok(())
    }
}