use amethyst::{
    ecs::{
        Component, DenseVecStorage, Storage, storage::MaskedStorage, Join,
    },
    shred::{ Fetch, FetchMut },
    ui::UiTransform,
};

use crate::lib::map;
use crate::lib::utils;

// TODO: add a camera component that
// 1) if can find player: follows the player while within the bounds of the screen
// 2) otherwise: reset to origin
#[derive(Default)]
pub struct CameraComp;

impl Component for CameraComp {
    type Storage = DenseVecStorage<Self>;
}

// TODO: add game object components
// mainly used for identifying entities
#[derive(Default)]
pub struct MapComp;

impl Component for MapComp {
    type Storage = DenseVecStorage<Self>;
}

// Player is special
pub struct PlayerComp {
    pub name: String,
    pub can_jump: bool,
    pub jump_count: i32,
    pub speed: (i32, i32),
    pub last_pos: Option<(i32, i32)>,
}

impl Component for PlayerComp {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComp {
    pub fn new(name: String) -> Self {
        PlayerComp { name, can_jump: false, jump_count: 0, speed: (0, 0), last_pos: None }
    }

    pub fn move_(&mut self,
        obj_store: &Storage<ObjectComp, Fetch<MaskedStorage<ObjectComp>>>,
        uitrans_store: &mut Storage<UiTransform, FetchMut<MaskedStorage<UiTransform>>>) {

        // gravity
        self.speed.1 -= utils::GRAVITY;
        // find data to change
        for (uitrans, obj) in (uitrans_store, obj_store).join() {
            if obj.name == self.name {
                // assign new position
                let x_before_scale = (uitrans.local_x / utils::DPI) as i32;
                let y_before_scale = (uitrans.local_y / utils::DPI) as i32;
                uitrans.local_x = (x_before_scale + self.speed.0) as f32 * utils::DPI;
                uitrans.local_y = (y_before_scale + self.speed.1) as f32 * utils::DPI;

                // record last position
                self.last_pos = Some((x_before_scale, y_before_scale));
                return;
            }
        } 
    }
}

// All game objects
pub enum ObjectType {
    Player, Target, Coin, Switch, Monster, Elevator, Obstacle
}

pub struct ObjectComp {
    pub name: String,
    pub type_: ObjectType,
}

impl Component for ObjectComp {
    type Storage = DenseVecStorage<Self>;
}

impl ObjectComp {
    pub fn new(name: String, type_: ObjectType) -> Self {
        return ObjectComp { name, type_ }
    }
}

// All movables (monsters, elevators)
pub struct MovableComp {
    pub name: String,
    pub track: Vec<map::Track>,
    pub speed: (i32, i32),
}

impl Component for MovableComp {
    type Storage = DenseVecStorage<Self>;
}

impl MovableComp {
    pub fn new(name: String, track: Vec<map::Track>) -> Self {
        return MovableComp { name, track, speed: (0, 0) }
    }

    pub fn move_(&mut self,
        obj_store: &Storage<ObjectComp, Fetch<MaskedStorage<ObjectComp>>>,
        uitrans_store: &mut Storage<UiTransform, FetchMut<MaskedStorage<UiTransform>>>) {
        for (uitrans, obj) in (uitrans_store, obj_store).join() {
            if obj.name == self.name {
                // find unscaled position
                let x_before_scale = (uitrans.local_x / utils::DPI) as i32;
                let y_before_scale = (uitrans.local_y / utils::DPI) as i32;

                // update speed
                for tr in self.track.iter() {
                    if tr.pos.0 == x_before_scale && tr.pos.1 == y_before_scale {
                        self.speed = tr.speed;
                        break;
                    }
                }

                // move
                uitrans.local_x = (x_before_scale + self.speed.0) as f32 * utils::DPI;
                uitrans.local_y = (y_before_scale + self.speed.1) as f32 * utils::DPI;
                break;
            }
        } 
    }
}

pub struct CollidableComp {
    pub name: String,
}

impl Component for CollidableComp {
    type Storage = DenseVecStorage<Self>;
}

impl CollidableComp {
    pub fn new(name: String) -> Self {
        CollidableComp { name }
    }
}

pub struct InteractableComp {
    name: String,
}

impl Component for InteractableComp {
    type Storage = DenseVecStorage<Self>;
}

impl InteractableComp {
    pub fn new(name: String) -> Self {
        InteractableComp { name }
    }
}

#[derive(Default)]
pub struct DescriptionComp;

impl Component for DescriptionComp {
    type Storage = DenseVecStorage<Self>;
}
