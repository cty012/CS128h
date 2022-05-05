use std::collections::HashMap;
use amethyst::{
    ecs::{
        World, WorldExt, Entity, Component, Storage,
        DenseVecStorage, storage::MaskedStorage, Join,
    },
    shred::{ Fetch, FetchMut },
    ui::{ UiTransform, UiImage },
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
    pub on_ground: bool,
    pub jump_count: i32,
    pub speed: (i32, i32),
    pub last_pos: Option<(i32, i32)>,
}

impl Component for PlayerComp {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComp {
    pub fn new(name: String) -> Self {
        PlayerComp {
            name, can_jump: false, on_ground: false,
            jump_count: 0, speed: (0, 0), last_pos: None
        }
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
                let x_before_scale = (uitrans.local_x / utils::dpi()) as i32;
                let y_before_scale = (uitrans.local_y / utils::dpi()) as i32;
                uitrans.local_x = (x_before_scale + self.speed.0) as f32 * utils::dpi();
                uitrans.local_y = (y_before_scale + self.speed.1) as f32 * utils::dpi();

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
                let x_before_scale = (uitrans.local_x / utils::dpi()) as i32;
                let y_before_scale = (uitrans.local_y / utils::dpi()) as i32;

                // update speed
                for tr in self.track.iter() {
                    if tr.pos.0 == x_before_scale && tr.pos.1 == y_before_scale {
                        self.speed = tr.speed;
                        break;
                    }
                }

                // move
                uitrans.local_x = (x_before_scale + self.speed.0) as f32 * utils::dpi();
                uitrans.local_y = (y_before_scale + self.speed.1) as f32 * utils::dpi();
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

#[derive(Clone)]
pub struct InteractableComp {
    pub name: String,
    pub state: String,
    pub command: HashMap<String, Vec<Vec<String>>>,
}

impl Component for InteractableComp {
    type Storage = DenseVecStorage<Self>;
}

impl InteractableComp {
    pub fn new(name: String, command: HashMap<String, Vec<Vec<String>>>) -> Self {
        InteractableComp { name, state: "close".to_string(), command }
    }

    pub fn exec(&mut self, world: &mut World) {
        if !self.command.contains_key(&self.state) { return; }
        for command in self.command[&self.state].clone().iter() {
            self.exec_command(world, command);
        }
    }

    fn exec_command(&mut self, world: &mut World, command: &Vec<String>) {
        if command.len() == 0 { return; }
        match command[0].as_str() {
            "state" => {
                self.state = command[1].clone();
            }
            "color" => {
                let obj_store = world.read_storage::<ObjectComp>();
                let mut img_store = world.write_storage::<UiImage>();
                let new_color = [
                    command[1].parse::<u32>().unwrap(),
                    command[2].parse::<u32>().unwrap(),
                    command[3].parse::<u32>().unwrap(),
                    255 as u32,
                ];
                for (obj, img) in (&obj_store, &mut img_store).join() {
                    if obj.name == self.name {
                        if let UiImage::SolidColor(ref mut color) = img {
                            *color = utils::get_color(new_color);
                        }
                        break;
                    }
                }
            }
            "remove" => {
                let obj_store = world.read_storage::<ObjectComp>();
                let mut entities_to_be_removed: Vec<Entity> = vec![];
                for entity in (world.entities()).join() {
                    match obj_store.get(entity) {
                        Some(obj) => {
                            if obj.name == command[1] {
                                entities_to_be_removed.push(entity.clone());
                            }
                        }
                        None => {}
                    }
                }
                drop(obj_store);
                for ent in entities_to_be_removed.iter() {
                    world.delete_entity(*ent).expect("Entity does not exist");
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct DescriptionComp;

impl Component for DescriptionComp {
    type Storage = DenseVecStorage<Self>;
}
