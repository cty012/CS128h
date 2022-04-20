use amethyst::{
    assets::Loader,
    prelude::*,
    ui::{
        Anchor, FontHandle, LineMode, TtfFormat,
        UiButtonBuilder, UiImage, UiText, UiTransform,
    },
    window::ScreenDimensions,
};

#[path = "utils.rs"] mod utils;
#[path = "component.rs"] mod comp;

// Background
pub struct Background {
    color: [f32; 4]
}

impl Background {
    pub fn default() -> Background {
        Background { color: utils::get_color(utils::BACKGROUND_COLOR) }
    }

    pub fn new(color: [f32; 4]) -> Background {
        Background { color }
    }

    pub fn instantiate(&self, id: String, world: &mut World) -> &Self {
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                0., 0., 0., dimensions.width(), dimensions.height()))
            .with(UiImage::SolidColor(self.color.clone()))
            .build();
        self
    }
}

// Label
pub struct Label {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub font_size: f32,
    pub font_family: String,
}

impl Label {
    pub fn default(text: String, width: f32, height: f32, font_size: f32) -> Self {
        Self::new(
            text, width, height, utils::get_color(utils::BLACK),
            font_size, "cambria.ttf".to_string()
        )
    }

    pub fn new(text: String, width: f32, height: f32, color: [f32; 4],
        font_size: f32, font_family: String) -> Self {
        Label { text, width, height, color, font_size, font_family }
    }

    pub fn instantiate(&self, id: String, world: &mut World, x: f32, y: f32, z: f32) -> &Self {
        let mut font_address = "fonts/".to_string();
        font_address.push_str(&self.font_family.clone());
        let font: FontHandle = world.read_resource::<Loader>().load(
            font_address, TtfFormat, (), &world.read_resource(),
        );
        
        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                x, y, z, self.width, self.height))
            .with(UiText::new(
                font.clone(), self.text.clone(), self.color.clone(), self.font_size,
                LineMode::Single, Anchor::Middle))
            .build();
        
        self
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
    pub fn default(text: String, width: f32, height: f32, font_size: f32) -> Self {
        Self::new(
            text, width, height, utils::get_color(utils::BLACK),
            utils::get_color(utils::GRAY_1), utils::get_color(utils::GRAY_2),
            font_size, "cambria.ttf".to_string()
        )
    }

    pub fn new(text: String, width: f32, height: f32, color: [f32; 4], bg_color: [f32; 4],
        fr_color: [f32; 4], font_size: f32, font_family: String) -> Self {
        Button {
            pos: [0., 0., 0.],
            label: Label::new(text, width, height, color, font_size, font_family),
            bg_color, fr_color
        }
    }

    pub fn get_pos(&self, anchor: Anchor) -> (f32, f32) {
        let x = match anchor {
            Anchor::TopLeft | Anchor::MiddleLeft | Anchor::BottomLeft => {
                self.pos[0] - self.label.width * 0.5
            }
            Anchor::TopMiddle | Anchor::Middle | Anchor::BottomMiddle => {
                self.pos[0]
            }
            Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => {
                self.pos[0] + self.label.width * 0.5
            }
        };
        let y = match anchor {
            Anchor::TopLeft | Anchor::TopMiddle | Anchor::TopRight => {
                self.pos[1] + self.label.height * 0.5
            }
            Anchor::MiddleLeft | Anchor::Middle | Anchor::MiddleRight => {
                self.pos[1]
            }
            Anchor::BottomLeft | Anchor::BottomMiddle | Anchor::BottomRight => {
                self.pos[1] - self.label.height * 0.5
            }
        };
        (x, y)
    }

    pub fn in_range(&self, x: f32, y: f32) -> bool {
        let (x_min, y_min) = self.get_pos(Anchor::BottomLeft);
        let (x_max, y_max) = self.get_pos(Anchor::TopRight);
        return x > x_min && x < x_max && y > y_min && y < y_max;
    }

    pub fn instantiate(&mut self, id: String, world: &mut World, x: f32, y: f32, z: f32) -> &Self {
        self.pos = [x, y, z];

        let mut font_address = "fonts/".to_string();
        font_address.push_str(&self.label.font_family.clone());
        let font: FontHandle = world.read_resource::<Loader>().load(
            font_address, TtfFormat, (), &world.read_resource(),
        );
        
        // round corners (amethyst does not support drawing circles)
        // let corners = [
        //     [x - 0.5 * self.label.width, y - 0.5 * self.label.height],
        //     [x - 0.5 * self.label.width, y + 0.5 * self.label.height],
        //     [x + 0.5 * self.label.width, y - 0.5 * self.label.height],
        //     [x + 0.5 * self.label.width, y + 0.5 * self.label.height],
        // ];
        // for corner in corners {
        //     world.create_entity()
        //     .with(UiTransform::new(
        //         id.clone(), Anchor::Middle, Anchor::Middle,
        //         corner[0], corner[1], z - 0.2, 5., 5.))
        //     .with(UiImage::SolidColor())
        //     .build();
        // }

        // frame
        world.create_entity()
            .with(UiTransform::new(
                id.clone(), Anchor::Middle, Anchor::Middle,
                x, y, z - 0.2, self.label.width + 5., self.label.height + 5.))
            .with(UiImage::SolidColor(self.fr_color.clone()))
            .build();

        // background
        world.create_entity()
            .with(UiTransform::new(
                id.clone(), Anchor::Middle, Anchor::Middle,
                x, y, z - 0.1, self.label.width, self.label.height))
            .with(UiImage::SolidColor(self.bg_color.clone()))
            .build();
        
        // text
        world.create_entity()
            .with(UiTransform::new(
                id, Anchor::Middle, Anchor::Middle,
                x, y, z, self.label.width, self.label.height))
            .with(UiText::new(
                font.clone(), self.label.text.clone(), self.label.color.clone(),
                self.label.font_size, LineMode::Single, Anchor::Middle))
            .build();

        self
    }
}
