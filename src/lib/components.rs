use amethyst::{
    ecs::{
        Component, DenseVecStorage, World, WorldExt, Entity
    },
};

#[path = "utils.rs"] mod utils;

// TODO: add a camera component that
// 1) if can find player: follows the player while within the bounds of the screen
// 2) otherwise: reset to origin
#[derive(Default)]
pub struct CameraComp {
    pub alpha: f32
}

impl Component for CameraComp {
    type Storage = DenseVecStorage<Self>;
}

impl CameraComp {
    pub fn new(alpha: f32) -> Self {
        CameraComp { alpha }
    }
}

// TODO: add game object components
// mainly used for identifying entities
pub struct PlayerComp {
    pos: (f32, f32),
    size: (f32, f32),
}

impl Component for PlayerComp {
    type Storage = DenseVecStorage<Self>;
}
