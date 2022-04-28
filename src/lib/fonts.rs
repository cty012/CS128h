#![allow(dead_code)]

use std::collections::HashMap;
use amethyst::{
    prelude::*,
    assets::Loader,
    ui::{ FontHandle, TtfFormat },
};

static mut INSTANCE: Option<Fonts> = None;

pub struct Fonts {
    fonts: HashMap<String, FontHandle>
}

impl Fonts {
    fn new() -> Self {
        Fonts{ fonts: HashMap::default() }
    }

    pub fn instance() -> &'static mut Self {
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Fonts{ fonts: HashMap::default() });
            }
            INSTANCE.as_mut().unwrap()
        }
    }

    pub fn get(&mut self, font_family: String, world: &World) -> &FontHandle {
        // add if not already loaded
        if !self.fonts.contains_key(&font_family) {
            let mut font_address = "fonts/".to_string();
            font_address.push_str(&font_family.clone());
            let font = world.read_resource::<Loader>().load(
                font_address, TtfFormat, (), &world.read_resource(),
            );
            self.fonts.insert(font_family.to_string(), font);
        }
        self.fonts.get(&font_family).unwrap()
    }
}
