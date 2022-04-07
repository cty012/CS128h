use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
mod state;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    amethyst::start_logger(Default::default());
    let game_data = GameDataBuilder::default();

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, state::MenuState, game_data)?;
    game.run();

    Ok(())
}
