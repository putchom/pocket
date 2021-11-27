use crate::helpers::{
    character_image,
    screen,
};
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*,
};

pub struct HomePage;

impl HomePage {
    pub fn render<T>(
        display: &mut T,
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        let state = character_image::State::Sleep;
        let data = character_image::get_data(&state);
        let point = character_image::get_point(&state);

        Image::new(&data, point).draw(display)?;
        Ok(())
    }
}