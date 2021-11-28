use crate::helpers::screen;
use crate::models::router::Route;

use core::convert::TryInto;
use embedded_graphics::{
    egrectangle,
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style,
};

const ICON_SIZE: i32 = 32;

pub struct NavigationView;

impl NavigationView {
    pub fn render<T>(
        display: &mut T,
        focus: Route
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        // ナビゲーション表示エリアをクリアする
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (screen::SCREEN_WIDTH / 2, screen::STATUS_BAR_HEIGHT),
            style = primitive_style!(fill_color = screen::BACKGROUND_COLOR)
        )
        .draw(display)?;

        let data = match focus {
            Route::Home => include_bytes!("../assets/navigation/home.raw"),
            Route::Meal => include_bytes!("../assets/navigation/meal.raw"),
            Route::Play => include_bytes!("../assets/navigation/play.raw"),
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
}