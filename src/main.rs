use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_turborand::prelude::*;
use cutscene::CutscenePlugin;
use gameplay::GameplayPlugin;
use loading::LoadingPlugin;
use mainmenu::MainMenuPlugin;
use pausemenu::PauseMenuPlugin;
use scoreboard::ScoreboardPlugin;
use states::AppState;

mod constants;
mod cutscene;
mod gameplay;
mod loading;
mod mainmenu;
mod pausemenu;
mod scoreboard;
mod states;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ligma: The fourth trimester".into(),
                        resolution: (800., 600.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RngPlugin::default())
        .add_plugin(MainMenuPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(CutscenePlugin)
        .add_plugin(ScoreboardPlugin)
        .add_startup_system(setup_camera)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
