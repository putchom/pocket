use crate::helpers::{
    image,
    screen,
};
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*,
};

pub struct ThrowPage;

impl ThrowPage {
    pub fn render<T>(
        display: &mut T
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        let state = image::CharacterState::Play;
        let data = image::get_character_data(&state);
        let point = image::get_character_point(&state);

        Image::new(&data, point).draw(display)?;

        Ok(())
    }
}