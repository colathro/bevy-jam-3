use bevy::prelude::*;
use interact::InteractPlugin;

use self::ui::UIPlugin;
use self::{
    enemies::EnemyPlugin, physics::PhysicsPlugin, player::PlayerPlugin, world::WorldPlugin,
};

mod enemies;
mod events;
mod interact;
mod physics;
mod player;
mod ui;
mod world;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InteractPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(WorldPlugin);
        app.add_plugin(PhysicsPlugin);
        app.add_plugin(EnemyPlugin);
        app.add_plugin(UIPlugin);
    }
}

pub use enemies::{Enemy, EnemySpawner};
pub use player::Bullet;
pub use player::Player;
