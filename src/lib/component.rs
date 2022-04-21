use amethyst::ecs::{
    Component, DenseVecStorage,
};

#[path = "utils.rs"] mod utils;

// TODO: add a camera component that
// 1) if can find player: follows the player while within the bounds of the screen
// 2) otherwise: reset to origin

// TODO: add game object components
// mainly used for identifying entities
