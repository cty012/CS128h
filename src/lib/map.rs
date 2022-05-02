use serde::{Deserialize, Serialize};
use amethyst::{
    core::Parent,
    ecs::Entity,
    prelude::*,
    ui::{ Anchor, LineMode, UiImage, UiText, UiTransform },
};

use crate::lib::components;
use crate::lib::fonts;
use crate::lib::utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    pub size: (u32, u32),
    pub player: Player,
    pub targets: Vec<Target>,
    pub coins: Vec<Coin>,
    pub switches: Vec<Switch>,
    pub monsters: Vec<Monster>,
    pub elevators: Vec<Elevator>,
    pub obstacles: Vec<Obstacle>,
    pub descriptions: Vec<Description>,
}

impl Map {
    pub fn initialize(&self, mut world: &mut World) {
        let map_ent = world.create_entity()
            .with(UiTransform::new(
                "map".to_string(), Anchor::BottomLeft, Anchor::BottomLeft,
                0., 0., 0.,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(utils::get_color(utils::BACKGROUND_COLOR)))
            .with(components::MapComp::default())
            .build();
        self.player.initialize(&mut world, &map_ent);
        for target in self.targets.iter() {
            target.initialize(&mut world, &map_ent);
        }
        for coin in self.coins.iter() {
            coin.initialize(&mut world, &map_ent);
        }
        for switch in self.switches.iter() {
            switch.initialize(&mut world, &map_ent);
        }
        for monster in self.monsters.iter() {
            monster.initialize(&mut world, &map_ent);
        }
        for elevator in self.elevators.iter() {
            elevator.initialize(&mut world, &map_ent);
        }
        for obstacle in self.obstacles.iter() {
            obstacle.initialize(&mut world, &map_ent);
        }
        for description in self.descriptions.iter() {
            description.initialize(&mut world, &map_ent);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Player {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                "player".to_string(), Anchor::BottomLeft, Anchor::TopLeft,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.7,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new("player".to_string(), components::ObjectType::Player))
            .with(components::PlayerComp::new("player".to_string()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Target {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.2,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Target))
            .with(components::InteractableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Coin {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.6,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Coin))
            .with(components::InteractableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Switch {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Switch {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.4,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Switch))
            .with(components::InteractableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monster {
    pub name: String,
    pub track: Vec<Track>,
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Monster {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.track[0].pos.0 as f32 * utils::DPI, self.track[0].pos.1 as f32 * utils::DPI, 0.5,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Monster))
            .with(components::MovableComp::new(self.name.clone(), self.track.clone()))
            .with(components::InteractableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Elevator {
    pub name: String,
    pub track: Vec<Track>,
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Elevator {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.track[0].pos.0 as f32 * utils::DPI, self.track[0].pos.1 as f32 * utils::DPI, 0.3,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Elevator))
            .with(components::MovableComp::new(self.name.clone(), self.track.clone()))
            .with(components::CollidableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Obstacle {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub color: (u32, u32, u32),
}

impl Obstacle {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::TopLeft,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.1,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiImage::SolidColor(
                utils::get_color([self.color.0, self.color.1, self.color.2, 255])))
            .with(components::ObjectComp::new(self.name.clone(), components::ObjectType::Obstacle))
            .with(components::CollidableComp::new(self.name.clone()))
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Description {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (i32, i32),
    pub text: String,
    pub font: Font,
    pub color: (u32, u32, u32),
}

impl Description {
    pub fn initialize(&self, world: &mut World, parent: &Entity) {
        let font = fonts::Fonts::instance().get(self.font.family.clone(), world);

        world.create_entity()
            .with(UiTransform::new(
                self.name.clone(), Anchor::BottomLeft, Anchor::Middle,
                self.pos.0 as f32 * utils::DPI, self.pos.1 as f32 * utils::DPI, 0.11,
                self.size.0 as f32 * utils::DPI, self.size.1 as f32 * utils::DPI))
            .with(UiText::new(
                font.clone(), self.text.clone(),
                utils::get_color([self.color.0, self.color.1, self.color.2, 255]),
                self.font.size as f32 * utils::DPI,
                LineMode::Wrap, Anchor::Middle))
            .with(components::DescriptionComp::default())
            .with(Parent::new(*parent))
            .build();
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub pos: (i32, i32),
    pub speed: (i32, i32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Font {
    pub family: String,
    pub size: u32,
}

pub fn from_file(file_name: String) -> Result<Map, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_name)?;
    let obj = ron::from_str(contents.as_str())?;
    Ok(obj)
}
