use crate::models::target::TargetPosition;
use crate::helpers::screen;
use core::convert::TryInto;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};

const ICON_SIZE: i32 = 32;

pub struct GamePage;

impl GamePage {
    pub fn render<T>(
        display: &mut T,
        target_position: &TargetPosition
    ) -> Result<(), T::Error>
    where
        T: DrawTarget<Rgb565>,
    {
        screen::clear_page(display)?;

        // 左の的を描画
        let left_data = match target_position {
            TargetPosition::Left => include_bytes!("../../assets/navigation/play.raw"),
            TargetPosition::Center => include_bytes!("../../assets/navigation/home.raw"),
            TargetPosition::Right => include_bytes!("../../assets/navigation/home.raw")
        };
        let left_image_data = ImageRawLE::new(
            left_data,
            ICON_SIZE.try_into().unwrap(),
            ICON_SIZE.try_into().unwrap(),
        );
        let left_point = Point::new(0, ICON_SIZE);
        Image::new(&left_image_data, left_point).draw(display)?;

        // 中央の的を描画
        let center_data = match target_position {
            TargetPosition::Left => include_bytes!("../../assets/navigation/home.raw"),
            TargetPosition::Center => include_bytes!("../../assets/navigation/play.raw"),
            TargetPosition::Right => include_bytes!("../../assets/navigation/home.raw")
        };
        let center_image_data = ImageRawLE::new(
            center_data,
            ICON_SIZE.try_into().unwrap(),
            ICON_SIZE.try_into().unwrap(),
        );
        let center_point = Point::new(ICON_SIZE, ICON_SIZE);
        Image::new(&center_image_data, center_point).draw(display)?;

        // 右の的を描画
        let right_data = match target_position {
            TargetPosition::Left => include_bytes!("../../assets/navigation/home.raw"),
            TargetPosition::Center => include_bytes!("../../assets/navigation/home.raw"),
            TargetPosition::Right => include_bytes!("../../assets/navigation/play.raw")
        };
        let right_image_data = ImageRawLE::new(
            right_data,
            ICON_SIZE.try_into().unwrap(),
            ICON_SIZE.try_into().unwrap(),
        );
        let right_point = Point::new(ICON_SIZE * 2, ICON_SIZE);
        Image::new(&right_image_data, right_point).draw(display)?;

        Ok(())
    }
}