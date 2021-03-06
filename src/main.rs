use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod lib;

fn main() -> amethyst::Result<()> {
    // find the paths to important locations
    let app_root = application_root_dir()?;
    let resources_path = app_root.join("assets");
    let display_path = app_root.join("config/display.ron");

    // create the game data
    amethyst::start_logger(Default::default());
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]))
            .with_plugin(RenderUi::default())
            .with_plugin(RenderFlat2D::default()))?;

    // start the game with InitState
    let mut game = Application::new(resources_path, lib::states::InitState::default(), game_data)?;
    game.run();

    Ok(())
}
