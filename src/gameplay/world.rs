use crate::states::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_world.in_schedule(OnEnter(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct WorldParent;

fn spawn_world(mut commands: Commands, world_query: Query<&WorldParent>) {
    for _ in world_query.iter() {
        return;
    }

    commands
        .spawn(TransformBundle { ..default() })
        .insert(WorldParent)
        .with_children(|parent| {
            // left wall
            parent
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(10.0, 600.0))
                .insert(TransformBundle::from(Transform::from_xyz(-400.0, 0.0, 0.0)))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(CollisionGroups::new(Group::ALL, Group::ALL));

            // right wall
            parent
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(10.0, 600.0))
                .insert(TransformBundle::from(Transform::from_xyz(400.0, 0.0, 0.0)))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(CollisionGroups::new(Group::ALL, Group::ALL));

            // top wall
            parent
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(800.0, 10.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(CollisionGroups::new(Group::ALL, Group::ALL));

            // bottom wall
            parent
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(800.0, 10.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(CollisionGroups::new(Group::ALL, Group::ALL));
        });
}
