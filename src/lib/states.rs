use std::collections::HashMap;
use amethyst::{
    ecs::{ WorldExt, Join, Entity },
    input::{ InputHandler, StringBindings, InputEvent, VirtualKeyCode, is_key_down },
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
            .instantiate("title".to_string(), data.world, 0., 200., 1.);

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
pub enum GameStatus {
    Win, Lose, None,
}

impl Default for GameStatus {
    fn default() -> Self { GameStatus::None }
}

#[derive(Default)]
pub struct GameState {
    level: u32,
    scoreboard: entities::Scoreboard,
    map: Option<map::Map>,
}

impl GameState {
    fn new(level: u32) -> Self {
        GameState { level, scoreboard: entities::Scoreboard::default(), map: None }
    }

    // delete an object with the name
    fn remove_objs(&mut self, world: &mut World, names: Vec<String>) {
        let obj_store = world.read_storage::<components::ObjectComp>();
        let mut entities_to_be_removed: Vec<Entity> = vec![];
        for entity in (world.entities()).join() {
            match obj_store.get(entity) {
                Some(obj) => {
                    if names.contains(&obj.name) {
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

    // this function allows the camera to follow the player
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
            // lerp the camera position to get a smooth camera movement
            // the target position is truncated AFTER the lerp to maintain the speed of the camera movement
            // which helps remind the player that the end of the map is reached
            uitrans.local_x = (uitrans.local_x - (uitrans.local_x - target_x) * alpha)
                .min(0.).max(dimensions.width() - uitrans.width);
            uitrans.local_y = (uitrans.local_y - (uitrans.local_y - target_y) * alpha)
                .min(0.).max(dimensions.height() - uitrans.height);
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

    fn check_win(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // access the storage
        let uitrans_store = data.world.read_storage::<UiTransform>();
        let player_store = data.world.read_storage::<components::PlayerComp>();
        let obj_store = data.world.read_storage::<components::ObjectComp>();
        let inter_store = data.world.read_storage::<components::InteractableComp>();

        // find the player
        let mut _player_uitrans: Option<&UiTransform> = None;
        for (uitrans, _player) in (&uitrans_store, &player_store).join() {
            _player_uitrans = Some(uitrans);
            break;
        }
        let player_uitrans = _player_uitrans.unwrap();

        // check if player is out of bounds
        if player_uitrans.local_y < utils::LOWER_BOUND as f32 * utils::DPI {
            return Trans::Push(Box::new(PauseState::new(self.level, GameStatus::Lose, self.scoreboard.score)));
        }

        // check if collide with enemy, target, or coin
        for (uitrans, _inter, obj) in (&uitrans_store, &inter_store, &obj_store).join() {
            match obj.type_ {
                components::ObjectType::Monster => {
                    if utils::compare(player_uitrans.clone(), uitrans.clone()) == Anchor::Middle {
                        return Trans::Push(Box::new(PauseState::new(self.level, GameStatus::Lose, self.scoreboard.score)));
                    }
                }
                components::ObjectType::Target => {
                    if utils::compare(player_uitrans.clone(), uitrans.clone()) == Anchor::Middle {
                        return Trans::Push(Box::new(PauseState::new(self.level, GameStatus::Win, self.scoreboard.score)));
                    }
                }
                _ => {}
            }
        }

        let mut coins_to_remove: Vec<String> = vec![];
        // check if collide with enemy, target, or coin
        for (uitrans, _inter, obj) in (&uitrans_store, &inter_store, &obj_store).join() {
            match obj.type_ {
                components::ObjectType::Coin => {
                    if utils::compare(player_uitrans.clone(), uitrans.clone()) == Anchor::Middle {
                        // remove this object
                        coins_to_remove.push(obj.name.clone());
                    }
                }
                _ => {}
            }
        }

        drop(uitrans_store);
        drop(player_store);
        drop(obj_store);
        drop(inter_store);
        self.scoreboard.add_score(data.world, coins_to_remove.len() as i32);
        self.remove_objs(&mut data.world, coins_to_remove);

        Trans::None
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

        // initialize scoreboard
        self.scoreboard.instantiate("scoreboard".to_string(), data.world, 50., -60., 1.5);

        // center camera
        self.follow_player(&data, utils::CAMERA_ALPHA);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(wevent) = &event {
            if is_key_down(&wevent, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState::new(self.level, GameStatus::None, self.scoreboard.score)));
            }
        }

        Trans::None
    }

    fn fixed_update(&mut self, mut data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
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

        // update resolved player position
        for (uitrans, _player) in (&mut uitrans_store, &player_store).join() {
            uitrans.local_x = target.0;
            uitrans.local_y = target.1;
        }

        // center camera
        drop(input);
        drop(uitrans_store);
        drop(player_store);
        drop(movable_store);
        drop(obj_store);
        drop(collidable_store);
        self.follow_player(&data, utils::CAMERA_ALPHA);

        // check win or lose
        self.check_win(&mut data)
    }
}

// TODO: Add a new pause state (also deal with win/lose situation)
// Game
#[derive(Default)]
pub struct PauseState {
    level: u32,
    status: GameStatus,
    score: i32,
    b_game: Option<entities::Button>,  // resume/replay button data
    b_menu: Option<entities::Button>,  // main menu button data
    ent_bg: Option<Entity>,  // the corresponding entities
    ent_title: Option<Entity>,
    ent_status: Option<Entity>,
    ent_b_game: Option<Entity>,
    ent_b_menu: Option<Entity>,
}

impl PauseState {
    pub fn new(level: u32, status: GameStatus, score: i32) -> Self {
        PauseState {
            level, status, score, b_game: None, b_menu: None,
            ent_bg: None, ent_title: None, ent_status: None, ent_b_game: None, ent_b_menu: None
        }
    }
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // decide displayed messages
        let title_msg = "Level ".to_string() + &self.level.to_string();
        let score_msg = "Score: ".to_string() + &self.score.to_string();
        let (status_msg, b_game_msg, bg_color) = match self.status {
            GameStatus::Win => { ("You Win!", "Replay", [192, 128, 128, 192]) }
            GameStatus::Lose => { ("You Lost!", "Replay", [128, 128, 192, 192]) }
            GameStatus::None => { ("Paused", "Resume", [192, 192, 192, 128]) }
        };

        // instantiate the background and the title
        self.ent_bg = Some(entities::Background::new(utils::get_color(bg_color))
            .instantiate_z("background".to_string(), data.world, 2.));
        self.ent_title = Some(entities::Label::default(
            title_msg, 600., 200., "cambria.ttf".to_string(), 60.)
            .instantiate("title".to_string(), data.world, 0., 150., 3.));
        self.ent_status = Some(entities::Label::default(
            status_msg.to_string(), 600., 200., "cambria.ttf".to_string(), 25.)
            .instantiate("title".to_string(), data.world, 0., 90., 3.));
        self.ent_status = Some(entities::Label::default(
            score_msg.to_string(), 600., 200., "cambria.ttf".to_string(), 25.)
            .instantiate("title".to_string(), data.world, 0., 50., 3.));

        // instantiate the scoreboard TODO

        // instantiate the buttons
        self.b_game = Some(entities::Button::default(
            b_game_msg.to_string(), 260., 50., "merriweather.ttf".to_string(), 25.));
        self.ent_b_game = Some(
            self.b_game.as_mut().unwrap().instantiate("game".to_string(), data.world, 0., -20., 3.));
        self.b_menu = Some(entities::Button::default(
            "Exit to menu".to_string(), 260., 50., "merriweather.ttf".to_string(), 25.));
        self.ent_b_menu = Some(
            self.b_menu.as_mut().unwrap().instantiate("menu".to_string(), data.world, 0., -120., 3.));
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_entity(self.ent_bg.unwrap()).unwrap_or(());
        data.world.delete_entity(self.ent_title.unwrap()).unwrap_or(());
        data.world.delete_entity(self.ent_status.unwrap()).unwrap_or(());
        data.world.delete_entity(self.ent_b_game.unwrap()).unwrap_or(());
        data.world.delete_entity(self.ent_b_menu.unwrap()).unwrap_or(());
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let (w, h) = (dimensions.width(), dimensions.height());
        let mouse_pos = data.world.read_resource::<InputHandler<StringBindings>>().mouse_position();

        // check if unpause
        if let StateEvent::Window(wevent) = &event {
            if is_key_down(&wevent, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        // check if click buttons
        match event {
            // if the user clicks the left mouse button
            StateEvent::Input(InputEvent::MouseButtonPressed(MouseButton::Left)) => {
                if let Some((mut x, mut y)) = mouse_pos {
                    // converts to game coords
                    x -= w * 0.5;
                    y = h * 0.5 - y;

                    // if user clicks game button
                    if let Some(btn) = &self.b_game {
                        if btn.in_range(x, y) {
                            match self.status {
                                GameStatus::None => { return Trans::Pop; }
                                _ => { 
                                    data.world.delete_all();
                                    return Trans::Replace(Box::new(GameState::new(self.level)));
                                }
                            }
                        }
                    }

                    // if user clicks menu button
                    if let Some(btn) = &self.b_menu {
                        if btn.in_range(x, y) {
                            data.world.delete_all();
                            return Trans::Replace(Box::new(MenuState::default()));
                        }
                    }
                }
                Trans::None
            }
            _ => Trans::None
        }
    }
}
