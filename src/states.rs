use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
    Paused,
    Scoreboard,
    Cutscene,
}
