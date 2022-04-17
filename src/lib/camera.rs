use amethyst::{
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions
};
#[path = "utils.rs"] mod utils;

pub fn initialise_camera(world: &mut World) {
    let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    world.create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(utils::get_center())
        .build();
}
