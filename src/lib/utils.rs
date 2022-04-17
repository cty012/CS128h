use amethyst::core::transform;
use amethyst::core::math;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

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
