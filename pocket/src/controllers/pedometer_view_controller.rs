use crate::models::{
    character::Character,
    pedometer::Pedometer,
    rice_ball::RiceBall,
    shuriken::Shuriken,
};
use crate::views::pedometer_view::PedometerView;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use accelerometer::vector::F32x3;

pub struct PedometerViewController;

impl PedometerViewController {
    #[allow(unused_must_use)]
    pub fn watch<T>(
        display: &mut T,
        normalized_accel: F32x3,
        pedometer: &mut Pedometer,
        rice_ball: &mut RiceBall,
        shuriken: &mut Shuriken,
    )
    where
        T: DrawTarget<Rgb565>,
    {
        Pedometer::update(pedometer, normalized_accel);
        Character::walk(pedometer, rice_ball, shuriken);
        PedometerView::render(display, &mut pedometer.step_count);
    }
}