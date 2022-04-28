use amethyst::{
    core::Transform,
    ecs::{WorldExt, Join},
    input::{InputHandler, StringBindings, InputEvent},
    prelude::*,
    renderer::Camera,
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::MouseButton,
};

#[path = "components.rs"] mod comp;
#[path = "entities.rs"] mod entities;
#[path = "map.rs"] mod map;

// Init
#[derive(Default)]
pub struct InitState;

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<comp::CameraComp>();
        data.world.register::<comp::PlayerComp>();
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Replace(Box::new(MenuState::default()))
    }
}

// Menu
#[derive(Default)]
pub struct MenuState {
    b_level: Option<entities::Button>,  // new game button data (not the actual entity)
    b_exit: Option<entities::Button>  // exit button data
}

impl SimpleState for MenuState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // reinitialize the camera whenever a new state starts
        // entities::init_camera(&mut data.world, 0.3);  // TODO: find out why this won't work (print resources)
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);
        data.world.create_entity()
            .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
            .with(transform)
            .with(comp::CameraComp::new(0.3))
            .build();

        // instantiate the background and the title
        entities::Background::default().instantiate("background".to_string(), data.world);
        entities::Label::default(
            "Platformer with RUST".to_string(), 600., 150., "cambria.ttf".to_string(), 60.)
            .instantiate("title1".to_string(), data.world, 0., 200., 1.);

        // instantiate the buttons
        self.b_level = Some(entities::Button::default(
            "New Game".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_level.as_mut().unwrap().instantiate("level".to_string(), data.world, 0., 0., 1.);
        self.b_exit = Some(entities::Button::default(
            "Exit".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_exit.as_mut().unwrap().instantiate("exit".to_string(), data.world, 0., -120., 1.);
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

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let cam_store = data.world.read_storage::<comp::CameraComp>();
        let mut trans_store = data.world.write_storage::<Transform>();
        for (trans, cam) in (&mut trans_store, &cam_store).join() {
            println!("found!");
        }
        println!(".");

        Trans::None
    }
}

// Level
#[derive(Default)]
pub struct LevelState {
    num_levels: usize,  // total number of levels (which is 8)
    b_menu: Option<entities::Button>,  // back button data (not the actual entity)
    b_games: Vec<Option<entities::Button>>,  // all 8 level selection buttons
}

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // reinitialize the camera whenever a new state starts
        self.num_levels = 8;
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);
        data.world.create_entity()
            .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
            .with(transform)
            .with(comp::CameraComp::new(0.3))
            .build();

        // instantiate the background and the title
        entities::Background::default().instantiate("background".to_string(), data.world);
        entities::Label::default(
            "Platformer with RUST".to_string(), 600., 150., "cambria.ttf".to_string(), 60., )
            .instantiate("title".to_string(), data.world, 0., 200., 1.);

        // instantiate the buttons
        self.b_menu = Some(entities::Button::default(
            "Back".to_string(), 300., 60., "merriweather.ttf".to_string(), 30.));
        self.b_menu.as_mut().unwrap().instantiate("menu".to_string(), data.world, 0., -200., 1.);
        let center = [0., 0.];
        let dist = [200., 160.];
        for level in 1..(self.num_levels + 1) {
            let x = center[0] + (((level - 1) % 4) as f32 - 1.5) * dist[0];
            let y = center[1] - (((level - 1) / 4) as f32 - 0.5) * dist[1];
            let mut b_game = Some(entities::Button::default(
                level.to_string(), 70., 100., "merriweather-b.ttf".to_string(), 30.));
            b_game.as_mut().unwrap().instantiate("exit".to_string(), data.world, x, y, 1.);
            self.b_games.push(b_game);
        }
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
    map: Option<map::Map>,
}

impl GameState {
    fn new(level: u32) -> Self {
        GameState { level, map: None }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // load map
        let app_root = application_root_dir().unwrap();
        let level_path = app_root.join("assets").join("levels").join(self.level.to_string() + ".ron")
            .into_os_string().into_string().unwrap();
        self.map = Some(map::from_file(level_path).unwrap());

        // initialize objects
        let map = self.map.as_ref().unwrap();
        map.initialize(&mut data.world);

        // center camera
        todo!();  // make a camera component that follows the player
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        // check pause events
        todo!();
        
        Trans::None
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // physics system
        todo!();

        // center camera
        todo!();

        // let cam_store = data.world.read_storage::<comp::CameraComp>();
        // let mut trans_store = data.world.write_storage::<Transform>();
        // for (trans, cam) in (&mut trans_store, &cam_store).join() {
        //     println!("found!");
        // }
        // println!(".");

        Trans::None
    }
}

// TODO: Add a new pause state (also deal with win/lose situation)
