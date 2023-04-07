use crate::states::AppState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_assets.in_set(OnUpdate(AppState::Loading)));
    }
}

#[derive(Resource)]
pub struct AudioHandles {
    pub cock: Handle<AudioSource>,
    pub gunshot: Handle<AudioSource>,
    pub shell: Handle<AudioSource>,
    pub bite: Handle<AudioSource>,
    pub empty: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
}

fn load_assets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    let audio_handles = AudioHandles {
        cock: asset_server.load("sounds/cock.ogg"),
        gunshot: asset_server.load("sounds/gunshot.ogg"),
        shell: asset_server.load("sounds/shell.ogg"),
        bite: asset_server.load("sounds/bite.ogg"),
        empty: asset_server.load("sounds/empty.ogg"),
        hit: asset_server.load("sounds/hit.ogg"),
    };
    commands.insert_resource(audio_handles);
    next_state.set(AppState::Menu);
}
