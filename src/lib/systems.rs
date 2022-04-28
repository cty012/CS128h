use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{System, SystemData, ReadStorage, WriteStorage, Join, World, WorldExt, storage::MaskedStorage},
    input::{InputHandler, VirtualKeyCode, StringBindings, InputEvent},
    ui::UiTransform,
    window::ScreenDimensions,
    winit::MouseButton,
};

#[path = "components.rs"] mod comp;

// System #OBSOLETE#
// TODO: Add a physics system and a scoreboard system

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, comp::CameraComp>,
        ReadStorage<'s, comp::PlayerComp>
    );

    fn setup(&mut self, world: &mut World) {
        world.register::<comp::CameraComp>();
        world.register::<comp::PlayerComp>();
        println!("Registered");
    }

    fn run(&mut self, (mut trans_store, mut uitrans_store, camera_store, player_store): Self::SystemData) {
        // Test code
        for trans in (&mut uitrans_store).join() {
        }
        return;

        // Actual code
        // Checks if the A button is down on the keyboard
        // if input.key_is_down(VirtualKeyCode::W)
        // The camera only moves if an entity with Player component is found
        // let (mut target_x, mut target_y) = (0., 0.);
        // let found = false;
        // for (uitrans, _player) in (&uitrans_store, &player_store).join() {
        //     target_x = uitrans.local_x;
        //     target_y = uitrans.local_y;
        //     found = true;
        //     break;
        // }
        // if !found { return; }
        // for (trans, camera) in (&mut trans_store, &camera_store).join() {
        //     trans.local_x -= (target_x - trans.local_x) * camera.alpha;
        //     trans.local_y -= (target_y - trans.local_y) * camera.alpha;
        // }
    }
}
