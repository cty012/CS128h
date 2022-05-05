## How to run the project
1. Make sure you are using Windows/MacOS and your computer has rust and cargo installed.
2. Select a place where you want to clone the project and execute `git clone https://github.com/cty012/CS128h/new/main`.
3. In the project, open the file `Cargo.toml` and change line 16 to:
   - If you are using Windows: `default = ["vulkan"]`
   - If you are using MacOS: `default = ["metal"]`
5. In the project root folder, execute `cargo run`.
6. If a window pops up then you may enjoy the game. If not (or if it appears for an instant and disappears) check if your computer supports vulkan/metal (I do not have a lot of computers to test this, but I assume all Windows systems support vulkan). Also, the project may not work with M1 due to one of the dependencies (`winit`) requires a version (`0.19.5`) that does not work on M1 MacOS.
