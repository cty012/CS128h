use std::collections::HashMap;
use amethyst::{
    ecs::{ WorldExt, Join },
    input::{ InputHandler, StringBindings, InputEvent, VirtualKeyCode },
    prelude::*,
    ui::{ Anchor, UiTransform },
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::lib::components;
use crate::lib::entities;
use crate::lib::map;
use crate::lib::utils;

// Init
#[derive(Default)]
pub struct InitState;

impl SimpleState for InitState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // register the components
        data.world.register::<components::CameraComp>();
        data.world.register::<components::MapComp>();
        data.world.register::<components::PlayerComp>();
        data.world.register::<components::ObjectComp>();
        data.world.register::<components::MovableComp>();
        data.world.register::<components::CollidableComp>();
        data.world.register::<components::InteractableComp>();
        data.world.register::<components::DescriptionComp>();

        // camera only need to be initialized once
        entities::init_camera(&mut data.world);
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
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
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
                            return Trans::Replace(Box::new(LevelState::new(8)));
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
    b_menu: Option<entities::Button>,  // back button data (not the actual entity)
    b_games: Vec<Option<entities::Button>>,  // all 8 level selection buttons
}

impl LevelState {
    fn new(num_levels: usize) -> Self {
        LevelState { num_levels, b_menu: None, b_games: vec![] }
    }
}

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.num_levels = 8;

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
    alpha: f32,
}

impl GameState {
    fn new(level: u32) -> Self {
        GameState { level, map: None, alpha: 0.1 }
    }

    /// This function allows the camera to follow the player
    fn follow_player(&mut self, data: &StateData<'_, GameData<'_, '_>>, alpha: f32) {
        // access the storage
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let mut uitrans_store = data.world.write_storage::<UiTransform>();
        let map_store = data.world.write_storage::<components::MapComp>();
        let player_store = data.world.write_storage::<components::PlayerComp>();

        // find the player's position
        let mut found = false;
        let mut target_x: f32 = 0.;
        let mut target_y: f32 = 0.;
        for (uitrans, _player) in (&uitrans_store, &player_store).join() {
            // The local position of the player has anchor bottomleft and pivot topleft.
            // The target postion is a vector that shows the required displacement of the map
            // such that the player can be centered.
            target_x = dimensions.width() * 0.5 - uitrans.local_x - uitrans.width * 0.5;
            target_y = dimensions.height() * 0.5 - uitrans.local_y + uitrans.height * 0.5;
            found = true;
            break;
        }

        // if the player is not found then there is some problem
        if !found { println!("WARNING: PLAYER NOT FOUND!"); return; }

        // center the player by moving the map (by a percentage)
        for (uitrans, _map) in (&mut uitrans_store, &map_store).join() {
            // truncate the target position such that it is inside the map
            target_x = target_x.min(0.).max(dimensions.width() - uitrans.width);
            target_y = target_y.min(0.).max(dimensions.height() - uitrans.height);
            uitrans.local_x -= (uitrans.local_x - target_x) * alpha;
            uitrans.local_y -= (uitrans.local_y - target_y) * alpha;
        }
    }

    fn get_rel_pos(&self, data: &StateData<'_, GameData<'_, '_>>) -> HashMap<String, Anchor> {
        let uitrans_store = data.world.read_storage::<UiTransform>();
        let player_store = data.world.read_storage::<components::PlayerComp>();
        let obj_store = data.world.read_storage::<components::ObjectComp>();

        let mut result = HashMap::new();

        let mut player_uitrans: Option<&UiTransform> = None;
        for (uitrans, _player) in (&uitrans_store, &player_store).join() {
            player_uitrans = Some(uitrans);
            break;
        }

        for (uitrans, obj) in (&uitrans_store, &obj_store).join() {
            result.insert(obj.name.clone(),
                utils::compare(player_uitrans.unwrap().clone(), uitrans.clone()));
        }

        result
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
        self.follow_player(&data, self.alpha);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        // check pause events TODO
        
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // get relative position
        let rel_pos = self.get_rel_pos(&data);

        // access the storage
        let input = data.world.read_resource::<InputHandler<StringBindings>>();
        let mut uitrans_store = data.world.write_storage::<UiTransform>();
        let mut player_store = data.world.write_storage::<components::PlayerComp>();
        let obj_store = data.world.read_storage::<components::ObjectComp>();
        let mut movable_store = data.world.write_storage::<components::MovableComp>();
        let collidable_store = data.world.read_storage::<components::CollidableComp>();

        // find the player
        let mut _player_comp: Option<&mut components::PlayerComp> = None;
        for (_uitrans, player) in (&uitrans_store, &mut player_store).join() {
            _player_comp = Some(player);
            break;
        }

        // if the player is not found then there is some problem
        if _player_comp.is_none() { println!("WARNING: PLAYER NOT FOUND!"); return Trans::None; }
        let player_comp = _player_comp.unwrap();

        // update player status
        player_comp.speed.0 = 0;
        if input.key_is_down(VirtualKeyCode::A) {
            player_comp.speed.0 -= utils::PLAYER_SPEED;
        }
        if input.key_is_down(VirtualKeyCode::D) {
            player_comp.speed.0 += utils::PLAYER_SPEED;
        }
        if input.key_is_down(VirtualKeyCode::W) {
            if player_comp.can_jump && player_comp.jump_count > 0 {
                player_comp.speed.1 = utils::PLAYER_JUMP;
                player_comp.can_jump = false;
                if !player_comp.on_ground { player_comp.jump_count -= 1; }
            }
        } else {
            player_comp.can_jump = true;
        }

        // find movables and move them (and player)
        let mut movables: Vec<&mut components::MovableComp> = Vec::new();
        for (_uitrans, movable) in (&mut uitrans_store, &mut movable_store).join() {
            movables.push(movable);
        }
        for movable in movables.iter_mut() { movable.move_(&obj_store, &mut uitrans_store); }
        player_comp.move_(&obj_store, &mut uitrans_store);

        // resolve collision (by changing the position of the player)
        // will not deal with the case that the player is squeezed between two collidables
        // the squeeze feature will be implemented in later versions

        let mut _player_uitrans: Option<UiTransform> = None;
        let mut target = (0., 0.);
        for (uitrans, obj) in (&uitrans_store, &obj_store).join() {
            if obj.name == player_comp.name {
                _player_uitrans = Some(uitrans.clone());
                target = (uitrans.local_x, uitrans.local_y);
                break;
            }
        }
        let player_uitrans = _player_uitrans.unwrap();
        player_comp.on_ground = false;
        for (uitrans, collidable) in (&uitrans_store, &collidable_store).join() {
            if utils::compare(player_uitrans.clone(), uitrans.clone()) == Anchor::Middle {  // TODO: get player moving track
                let direction = utils::anchor_to_tuple(rel_pos[&collidable.name]);
                if direction.1 != 0 {
                    target.1 = uitrans.local_y
                        + 0.5 * (direction.1 - 1) as f32 * uitrans.height
                        + 0.5 * (direction.1 + 1) as f32 * player_uitrans.height;
                    player_comp.speed.1 = 0;
                    player_comp.on_ground = true;
                    player_comp.jump_count = 1;
                }
                else if direction.0 != 0 {
                    target.0 = uitrans.local_x
                        + 0.5 * (direction.0 + 1) as f32 * uitrans.width
                        + 0.5 * (direction.0 - 1) as f32 * player_uitrans.width;  // player modifier
                    player_comp.speed.0 = 0;
                }
            }
        }
        for (uitrans, _player) in (&mut uitrans_store, &player_store).join() {
            uitrans.local_x = target.0;
            uitrans.local_y = target.1;
        }

        // center camera
        drop(input);
        drop(uitrans_store);
        drop(player_store);
        drop(movable_store);
        self.follow_player(&data, self.alpha);

        Trans::None
    }
}

// TODO: Add a new pause state (also deal with win/lose situation)
