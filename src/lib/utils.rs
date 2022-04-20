#![allow(dead_code)]

use amethyst::core::transform;
use amethyst::core::math;
use amethyst_rendy::palette::Srgba;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const BACKGROUND_COLOR: [u32; 4] = [60, 179, 113, 255];
pub const BLACK: [u32; 4] = [0, 0, 0, 255];
pub const WHITE: [u32; 4] = [255, 255, 255, 255];
pub const GRAY_1: [u32; 4] = [224, 224, 224, 255];
pub const GRAY_2: [u32; 4] = [192, 192, 192, 255];
pub const GRAY_3: [u32; 4] = [128, 128, 128, 255];

pub fn get_color([r, g, b, a]: [u32; 4]) -> [f32; 4] {
    let (new_r, new_g, new_b, _) = Srgba::new(
        r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.)
            .into_linear()
            .into_components();
    [new_r, new_g, new_b, a as f32 / 255.]
}


pub fn get_center() -> transform::Transform {
    transform::Transform::new(
        math::Translation3::new(WIDTH * 0.5, HEIGHT * 0.5, 1.0),
        math::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        math::Vector3::new(1.0, 1.0, 1.0)
    )
}

pub fn get_bottom_left() -> transform::Transform {
    transform::Transform::new(
        math::Translation3::new(0.0, 0.0, 1.0),
        math::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        math::Vector3::new(1.0, 1.0, 1.0)
    )
}
