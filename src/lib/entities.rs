use amethyst::{
    core::{ Transform, Parent },
    ecs::Entity,
    prelude::*,
    renderer::Camera,
    ui::{ Anchor, LineMode, UiImage, UiText, UiTransform },
    window::ScreenDimensions,
};

use crate::lib::components;
use crate::lib::fonts;
use crate::lib::utils;

// Camera
pub fn init_camera(world: &mut World) {
    let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 10.0);

    world.create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .with(components::CameraComp::default())
        .build();
}

// Background
pub struct Background {
    color: [f32; 4]
}

impl Background {
    pub fn default() -> Background {
        Background::new(utils::get_color(utils::BACKGROUND_COLOR))
    }

    pub fn new(color: [f32; 4]) -> Background {
        Background { color }
    }

    pub fn instantiate(&self, id: String, world: &mut World) -> Entity {
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                0., 0., 0., dimensions.width(), dimensions.height()))
            .with(UiImage::SolidColor(self.color.clone()))
            .build()
    }

    pub fn instantiate_z(&self, id: String, world: &mut World, z: f32) -> Entity {
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                0., 0., z, dimensions.width(), dimensions.height()))
            .with(UiImage::SolidColor(self.color.clone()))
            .build()
    }
}

// Label
pub struct Label {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub font_family: String,
    pub font_size: f32,
}

impl Label {
    pub fn default(text: String, width: f32, height: f32, font_family: String, font_size: f32) -> Self {
        Self::new(
            text, width, height, utils::get_color(utils::BLACK),
            font_family, font_size
        )
    }

    pub fn new(text: String, width: f32, height: f32, color: [f32; 4],
        font_family: String, font_size: f32) -> Self {
        Label { text, width, height, color, font_size, font_family }
    }

    pub fn instantiate(&self, id: String, world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        let font = fonts::Fonts::instance().get(self.font_family.clone(), world);

        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                x * utils::dpi(), y * utils::dpi(), z,
                self.width * utils::dpi(), self.height * utils::dpi()))
            .with(UiText::new(
                font.clone(), self.text.clone(), self.color.clone(), self.font_size * utils::dpi(),
                LineMode::Single, Anchor::Middle))
            .build()
    }
}

// Scoreboard
#[derive(Default)]
pub struct Scoreboard {
    pub score: i32,
    pub entity: Option<Entity>,
}

impl Scoreboard {
    pub fn instantiate(&mut self, id: String, world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        let font = fonts::Fonts::instance().get("digital-7-mono.ttf".to_string(), world);

        self.entity = Some(world.create_entity()
            .with(UiTransform::new(
                id, Anchor::TopLeft, Anchor::MiddleLeft,
                x * utils::dpi(), y * utils::dpi(), z,
                200. * utils::dpi(), 80. * utils::dpi()))
            .with(UiText::new(
                font.clone(), "Score: 0".to_string(),
                utils::get_color(utils::BLACK), 20. * utils::dpi(),
                LineMode::Single, Anchor::MiddleLeft))
            .build());

        self.entity.unwrap()
    }

    pub fn add_score(&mut self, world: &mut World, num: i32) {
        self.score += num;
        if self.entity.is_none() { return; }
        world.write_storage::<UiText>().get_mut(self.entity.unwrap()).unwrap()
            .text = "Score: ".to_string() + &self.score.to_string();
    }
}

// Button
pub struct Button {
    pos: [f32; 3],
    label: Label,
    bg_color: [f32; 4],
    fr_color: [f32; 4],
}

impl Button {
    pub fn default(text: String, width: f32, height: f32, font_family: String, font_size: f32) -> Self {
        Self::new(
            text, width, height, utils::get_color(utils::BLACK),
            utils::get_color(utils::GRAY_1), utils::get_color(utils::GRAY_2),
            font_family, font_size
        )
    }

    pub fn new(text: String, width: f32, height: f32, color: [f32; 4], bg_color: [f32; 4],
        fr_color: [f32; 4], font_family: String, font_size: f32) -> Self {
        Button {
            pos: [0., 0., 0.],
            label: Label::new(text, width, height, color, font_family, font_size),
            bg_color, fr_color
        }
    }

    pub fn get_pos(&self, pivot: Anchor) -> (f32, f32) {
        let x = match pivot {
            Anchor::TopLeft | Anchor::MiddleLeft | Anchor::BottomLeft => {
                self.pos[0] - self.label.width * utils::dpi() * 0.5
            }
            Anchor::TopMiddle | Anchor::Middle | Anchor::BottomMiddle => {
                self.pos[0]
            }
            Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => {
                self.pos[0] + self.label.width * utils::dpi() * 0.5
            }
        };
        let y = match pivot {
            Anchor::TopLeft | Anchor::TopMiddle | Anchor::TopRight => {
                self.pos[1] + self.label.height * utils::dpi() * 0.5
            }
            Anchor::MiddleLeft | Anchor::Middle | Anchor::MiddleRight => {
                self.pos[1]
            }
            Anchor::BottomLeft | Anchor::BottomMiddle | Anchor::BottomRight => {
                self.pos[1] - self.label.height * utils::dpi() * 0.5
            }
        };
        (x, y)
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        let (x_min, y_min) = self.get_pos(Anchor::BottomLeft);
        let (x_max, y_max) = self.get_pos(Anchor::TopRight);
        return x > x_min && x < x_max && y > y_min && y < y_max;
    }

    pub fn instantiate(&mut self, id: String, world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        self.pos = [x * utils::dpi(), y * utils::dpi(), z];

        let font = fonts::Fonts::instance().get(self.label.font_family.clone(), world);

        // parent
        let button = world.create_entity()
            .with(UiTransform::new(
                id.clone(), Anchor::Middle, Anchor::Middle,
                0., 0., 0., 0., 0.))
            .build();

        // frame
        world.create_entity()
            .with(UiTransform::new(
                id.clone() + "-frame", Anchor::Middle, Anchor::Middle,
                x * utils::dpi(), y * utils::dpi(), z - 0.2,
                (self.label.width + 5.) * utils::dpi(), (self.label.height + 5.) * utils::dpi()))
            .with(UiImage::SolidColor(self.fr_color.clone()))
            .with(Parent::new(button))
            .build();

        // background
        world.create_entity()
            .with(UiTransform::new(
                id.clone() + "-bg", Anchor::Middle, Anchor::Middle,
                x * utils::dpi(), y * utils::dpi(), z - 0.1,
                self.label.width * utils::dpi(), self.label.height * utils::dpi()))
            .with(UiImage::SolidColor(self.bg_color.clone()))
            .with(Parent::new(button))
            .build();
        
        // text
        world.create_entity()
            .with(UiTransform::new(
                id + "-text", Anchor::Middle, Anchor::Middle,
                x * utils::dpi(), y * utils::dpi(), z,
                self.label.width * utils::dpi(), self.label.height * utils::dpi()))
            .with(UiText::new(
                font.clone(), self.label.text.clone(), self.label.color.clone(),
                self.label.font_size * utils::dpi(), LineMode::Single, Anchor::Middle))
            .with(Parent::new(button))
            .build();

        button
    }
}
