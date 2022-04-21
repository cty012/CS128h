use amethyst::{
    prelude::*,
    input::{InputHandler, VirtualKeyCode, StringBindings, InputEvent},
    winit::MouseButton,
    derive::SystemDesc,
    ecs::{Read, System, SystemData},
    window::ScreenDimensions,
};

#[path = "component.rs"] mod comp;
mod fonts;
mod entity;
mod utils;

// System
// TODO: Add a physics system and a scoreboard system

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
    b_level: Option<entity::Button>,  // new game button data (not the actual entity)
    b_exit: Option<entity::Button>  // exit button data
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // reinitialize the camera whenever a new state starts
        let world = data.world;
        world.delete_all();
        entity::init_camera(world);

        // instantiate the background and the title
        entity::Background::default().instantiate("background".to_string(), world);
        entity::Label::default(
            "Platformer with RUST".to_string(), 600., 150., "cambria.ttf".to_string(), 60.)
            .instantiate("title1".to_string(), world, 0., 200., 1.);

        // instantiate the buttons
        self.b_level = Some(entity::Button::default(
            "New Game".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_level.as_mut().unwrap().instantiate("level".to_string(), world, 0., 0., 1.);
        self.b_exit = Some(entity::Button::default(
            "Exit".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_exit.as_mut().unwrap().instantiate("exit".to_string(), world, 0., -120., 1.);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        // extract necessary input and dimensions data
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let (w, h) = (dimensions.width(), dimensions.height());
        let mouse_pos = data.world.read_resource::<InputHandler<StringBindings>>().mouse_position();

        match event {
            // if the user clicks the left mouse button
            StateEvent::Input(InputEvent::MouseButtonPressed(MouseButton::Left)) => {
                if let Some((mut x, mut y)) = mouse_pos {
                    // converts to game coords
                    x -= w * 0.5;
                    y = h * 0.5 - y;

                    // if user clicks new game button
                    if let Some(btn) = &self.b_level {
                        if btn.in_range(x, y) {
                            data.world.delete_all();
                            return Trans::Replace(Box::new(LevelState::default()));
                        }
                    }

                    // if user clicks exit button
                    if let Some(btn) = &self.b_exit {
                        if btn.in_range(x, y) {
                            return Trans::Quit;
                        }
                    }
                }
                Trans::None
            }
           _ => Trans::None
        }
    }
}

// Level
#[derive(Default)]
pub struct LevelState {
    num_levels: usize,  // total number of levels (which is 8)
    b_menu: Option<entity::Button>,  // back button data (not the actual entity)
    b_games: Vec<Option<entity::Button>>,  // all 8 level selection buttons
}

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // reinitialize the camera whenever a new state starts
        self.num_levels = 8;
        let world = data.world;
        world.delete_all();
        entity::init_camera(world);

        // instantiate the background and the title
        entity::Background::default().instantiate("background".to_string(), world);
        entity::Label::default(
            "Platformer with RUST".to_string(), 600., 150., "cambria.ttf".to_string(), 60., )
            .instantiate("title".to_string(), world, 0., 200., 1.);

        // instantiate the buttons
        self.b_menu = Some(entity::Button::default(
            "Back".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_menu.as_mut().unwrap().instantiate("menu".to_string(), world, 0., -200., 1.);
        let center = [0., 0.];
        let dist = [200., 160.];
        for level in 1..(self.num_levels + 1) {
            let x = center[0] + (((level - 1) % 4) as f32 - 1.5) * dist[0];
            let y = center[1] - (((level - 1) / 4) as f32 - 0.5) * dist[1];
            let mut b_game = Some(entity::Button::default(
                level.to_string(), 70., 100., "merriweather-b.ttf".to_string(), 30.));
            b_game.as_mut().unwrap().instantiate("exit".to_string(), world, x, y, 1.);
            self.b_games.push(b_game);
        }
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        // extract necessary input and dimensions data
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let (w, h) = (dimensions.width(), dimensions.height());
        let mouse_pos = data.world.read_resource::<InputHandler<StringBindings>>().mouse_position();

        match event {
            StateEvent::Input(InputEvent::MouseButtonPressed(MouseButton::Left)) => {
                if let Some((mut x, mut y)) = mouse_pos {
                    // converts to game coords
                    x -= w * 0.5;
                    y = h * 0.5 - y;

                    // if user clicks back button
                    if let Some(btn) = &self.b_menu {
                        if btn.in_range(x, y) {
                            data.world.delete_all();  // clear entities before switching to new state
                            return Trans::Replace(Box::new(MenuState::default()));
                        }
                    }

                    // if user selects a level
                    for level in 1..(self.num_levels + 1) {
                        if let Some(btn) = &self.b_games[level - 1] {
                            if btn.in_range(x, y) {
                                data.world.delete_all();  // clear entities before switching to new state
                                return Trans::Switch(Box::new(GameState::new(level as u32)));
                            }
                        }
                    }
                }
                Trans::None
            }
           _ => Trans::None
        }
    }
}

// Game
#[derive(Default)]
pub struct GameState {
    level: u32,
}

impl GameState {
    fn new(level: u32) -> Self {
        GameState { level }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // load map
        todo!();

        // initialize objects
        todo!();

        // center camera
        todo!();  // make a camera component that follows the player
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        // check pause events
        todo!(); // make a
        
        Trans::None
    }
}
