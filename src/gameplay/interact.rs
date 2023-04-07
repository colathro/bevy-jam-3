use crate::states::AppState;
use bevy::prelude::*;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause.in_set(OnUpdate(AppState::InGame)));
    }
}

fn pause(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}
