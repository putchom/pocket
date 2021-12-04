use embedded_graphics::{
    egrectangle,
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style,
};

pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 240;
pub const FONT_WIDTH: i32 = 24;
pub const FONT_HEIGHT: i32 = 32;
pub const STATUS_BAR_HEIGHT: i32 = 32;
pub const BACKGROUND_COLOR: Rgb565 = Rgb565::WHITE;
pub const FOREGROUND_COLOR: Rgb565 = Rgb565::BLACK;

pub fn clear_screen<T>(display: &mut T) -> Result<(), T::Error>
where
    T: DrawTarget<Rgb565>,
{
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        style = primitive_style!(fill_color = BACKGROUND_COLOR)
    )
    .draw(display)?;
    Ok(())
}

// ページエリアのみクリアする
pub fn clear_page<T>(
    display: &mut T,
) -> Result<(), T::Error>
where
    T: DrawTarget<Rgb565>,
{
    egrectangle!(
        top_left = (0, STATUS_BAR_HEIGHT),
        bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        style = primitive_style!(fill_color = BACKGROUND_COLOR)
    )
    .draw(display)?;
    Ok(())
}
