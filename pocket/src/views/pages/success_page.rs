use crate::helpers::{
    image,
    screen,
};
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*,
};

pub struct SuccessPage;

impl SuccessPage {
    pub fn render<T>(
        display: &mut T
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        let state = image::CharacterState::Happy;
        let data = image::get_character_data(&state);
        let point = image::get_character_point(&state);

        Image::new(&data, point).draw(display)?;

        Ok(())
    }
}