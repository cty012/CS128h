impl<'s> System<'s> for ExampleSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {
        // Gets mouse coordinates
        if let Some((x, y)) = input.mouse_position() {
            //..
        }
        
        // Gets all connected controllers
        let controllers = input.connected_controllers();
        for controller in controllers {
            // Checks if the A button is down on each connected controller
            let buttonA = input.controller_button_is_down(controller, ControllerButton::A);
            //..
        }

        // Checks if the A button is down on the keyboard
        let buttonA = input.key_is_down(VirtualKeyCode::A);
        //..
    }
}
