use amethyst::{
    prelude::*,
    input::{InputHandler, VirtualKeyCode, StringBindings, InputEvent},
    winit::MouseButton,
    derive::SystemDesc,
    ecs::{Read, System, SystemData},
    window::ScreenDimensions,
};

mod camera;
mod component;
mod entity;
mod utils;

// System

// #[derive(SystemDesc)]
// struct InputSystem;

// impl<'s> System<'s> for InputSystem {
//     // The same BindingTypes from the InputBundle needs to be inside the InputHandler
//     type SystemData = Read<'s, InputHandler<StringBindings>>;

//     fn run(&mut self, (world, input): Self::SystemData) {
//         // Gets mouse coordinates
//         input.world.write_resource::<UserInput>().current_state = CurrentState::MainMenu;

//         // Checks if the A button is down on the keyboard
//         let buttonA = input.key_is_down(VirtualKeyCode::A);
//         //..
//     }
// }

// Menu
#[derive(Default)]
pub struct MenuState {
    b_new_game: Option<entity::Button>,
    b_exit: Option<entity::Button>
}

impl SimpleState for MenuState {
    fn on_start<'s>(&'s mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        camera::init_camera(world);

        entity::Background::default().instantiate("background".to_string(), world);
        entity::Label::default("Platformer with RUST".to_string(), 800., 200., 80.)
            .instantiate("title".to_string(), world, 0., 300., 1.);
        self.b_new_game = Some(entity::Button::default("New Game".to_string(), 400., 80., 40.,));
        self.b_new_game.as_mut().unwrap()
            .instantiate("new game".to_string(), world, 0., 0., 1.);
        self.b_exit = Some(entity::Button::default("Exit".to_string(), 400., 80., 40.,));
        self.b_exit.as_mut().unwrap()
            .instantiate("exit".to_string(), world, 0., -150., 1.);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let input = data.world.read_resource::<InputHandler<StringBindings>>();

        match event {
            StateEvent::Input(InputEvent::MouseButtonPressed(MouseButton::Left)) => {
                if let Some((mut x, mut y)) = input.mouse_position() {
                    x -= dimensions.width() * 0.5;
                    y = dimensions.height() * 0.5 - y;
                    if let Some(btn) = &self.b_new_game {
                        if btn.in_range(x, y) {
                            return Trans::Push(Box::new(LevelState::default()));
                        }
                    }
                    let btn = self.b_exit.as_ref().unwrap();
                    if btn.in_range(x, y) {
                        return Trans::Quit;
                    }
                }
                Trans::None
            }
           _ => Trans::None
        }
    }
}

// Level
enum LevelStateAction {
    Menu, Game(String),
}

#[derive(Default)]
pub struct LevelState {
    action: Option<LevelStateAction>,
}

impl SimpleState for LevelState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("level selection");
        return match &self.action {
            Some(action) => {
                match action {
                    LevelStateAction::Menu => { Trans::Pop }
                    LevelStateAction::Game(level) => { Trans::Quit }
                }
            }
            None => { Trans::None }
        }
    }
}
