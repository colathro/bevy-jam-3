use crate::{loading::AudioHandles, states::AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_turborand::prelude::*;
use instant::{Duration, Instant};

use super::player::{Bullet, Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy_spawner.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(spawn_enemies.in_set(OnUpdate(AppState::InGame)));
        app.add_system(start_stage.in_set(OnUpdate(AppState::InGame)));
        app.add_system(
            display_collision_events
                .after(move_enemies_toward_player)
                .in_set(OnUpdate(AppState::InGame)),
        );
        app.add_system(handle_enemy_death.in_set(OnUpdate(AppState::InGame)));
        app.add_system(check_for_stage_end.in_set(OnUpdate(AppState::InGame)));
        app.add_system(
            animate_enemies
                .after(handle_enemy_death)
                .in_set(OnUpdate(AppState::InGame)),
        );
        app.add_system(move_enemies_toward_player.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct EnemySpawner {
    pub stage: u32,
    started: bool,
    start_time: Instant,
    pub score: i32,
}

#[derive(Component)]
pub struct Enemy {
    health: i32,
    state: EnemyState,
}

#[derive(PartialEq)]
enum EnemyState {
    Zombie,
    Destroyed,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_enemy_spawner(mut commands: Commands, enemy_spawner_query: Query<&EnemySpawner>) {
    for _ in enemy_spawner_query.iter() {
        return;
    }

    commands.spawn(EnemySpawner {
        stage: 1,
        started: false,
        start_time: instant::Instant::now(),
        score: 0,
    });
}

fn start_stage(mut enemy_spawner_query: Query<&mut EnemySpawner>) {
    for mut spawner in enemy_spawner_query.iter_mut() {
        if !spawner.started {
            if instant::Instant::now() > spawner.start_time {
                spawner.started = true;
            }
        }
    }
}

fn get_current_ideal_enemy_count(stage: u32, time_since_start: Duration) -> usize {
    const MAX_DURATION_SECS: f32 = 30.0;
    let max_enemies_for_state = get_max_enemy_count(stage);
    let percent_duration_remaining = (time_since_start.as_secs() as f32) / MAX_DURATION_SECS;

    let ideal_cound_float = max_enemies_for_state as f32 * percent_duration_remaining;

    ideal_cound_float.ceil() as usize
}

fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &Enemy,
        &ExternalForce,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, enemy, force, mut timer, mut sprite) in &mut query {
        if enemy.state == EnemyState::Destroyed {
            sprite.index = 0;
            continue;
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            if (force.force.x > 0.1 || force.force.x < -0.1)
                || (force.force.y > 0.1 || force.force.y < -0.1)
            {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            } else {
                sprite.index = 1;
            }
        }
    }
}

fn get_max_enemy_count(stage: u32) -> usize {
    stage as usize * 10
}

fn spawn_enemies(
    commands: Commands,
    enemy_query: Query<&Enemy>,
    enemy_spawner_query: Query<&EnemySpawner>,
    global_rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Some(spawner) = enemy_spawner_query.iter().next() {
        if !spawner.started {
            return;
        }

        let time_since_stage_start = instant::Instant::now().duration_since(spawner.start_time);

        let ideal_enemy_count =
            get_current_ideal_enemy_count(spawner.stage, time_since_stage_start);
        let current_enemy_count = enemy_query.iter().len();

        if current_enemy_count >= get_max_enemy_count(spawner.stage) {
            return;
        }

        if current_enemy_count < ideal_enemy_count {
            spawn_enemy(commands, global_rng, asset_server, texture_atlases);
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/zombiebaby.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 2 };

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Enemy {
            health: 100,
            state: EnemyState::Zombie,
        })
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Collider::ball(10.0))
        .insert(ColliderMassProperties::Density(0.8))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(Group::ALL, Group::ALL))
        .insert((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            rng.i32(-390..=390) as f32,
            rng.i32(-290..=290) as f32,
            1.0,
        )));
}

fn display_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    //mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut enemy_query: Query<&mut Enemy>,
    mut bullet_query: Query<&mut Bullet>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first, second, _) => {
                let player_entity_opt = get_collided_player(first, second, &player_query);
                let enemy_entity_opt = get_collided_enemy(first, second, &enemy_query);
                let bullet_entity_opt = get_collided_bullet(first, second, &bullet_query);

                if let (Some(player_entity), Some(_)) = (player_entity_opt, enemy_entity_opt) {
                    // Something is confirmed started colliding with player.
                    let player_res = player_query.get_mut(player_entity);

                    if let Ok(mut player) = player_res {
                        player.health -= 10;
                        audio.play(Handle::weak(audio_handles.bite.id()));
                    }
                }

                if let (Some(_), Some(enemy_entity)) = (bullet_entity_opt, enemy_entity_opt) {
                    // Something is confirmed started colliding with player.
                    let enemy_res = enemy_query.get_mut(enemy_entity);

                    if let Ok(mut enemy) = enemy_res {
                        enemy.health -= 50;
                        audio.play(Handle::weak(audio_handles.hit.id()));
                    }
                }

                if let Some(bullet_entity) = bullet_entity_opt {
                    let bullet_res = bullet_query.get_mut(bullet_entity);

                    if let Ok(mut bullet) = bullet_res {
                        bullet.collided = true;
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn get_collided_player(
    first_collider: &Entity,
    second_collider: &Entity,
    player_query: &Query<&mut Player>,
) -> Option<Entity> {
    if let Ok(_) = player_query.get(*first_collider) {
        return Some(*first_collider);
    }

    if let Ok(_) = player_query.get(*second_collider) {
        return Some(*second_collider);
    }

    return None;
}

fn get_collided_enemy(
    first_collider: &Entity,
    second_collider: &Entity,
    enemy_query: &Query<&mut Enemy>,
) -> Option<Entity> {
    if let Ok(_) = enemy_query.get(*first_collider) {
        return Some(*first_collider);
    }

    if let Ok(_) = enemy_query.get(*second_collider) {
        return Some(*second_collider);
    }

    return None;
}

fn get_collided_bullet(
    first_collider: &Entity,
    second_collider: &Entity,
    bullet_query: &Query<&mut Bullet>,
) -> Option<Entity> {
    if let Ok(_) = bullet_query.get(*first_collider) {
        return Some(*first_collider);
    }

    if let Ok(_) = bullet_query.get(*second_collider) {
        return Some(*second_collider);
    }

    return None;
}

fn handle_enemy_death(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, Entity)>,
    mut enemy_spawner_query: Query<&mut EnemySpawner>,
) {
    if let Ok(mut enemy_spawner) = enemy_spawner_query.get_single_mut() {
        for (mut enemy, entity) in enemy_query.iter_mut() {
            if enemy.health <= 0 && enemy.state == EnemyState::Zombie {
                commands.entity(entity).remove::<Collider>();
                commands.entity(entity).remove::<RigidBody>();
                enemy.state = EnemyState::Destroyed;
                enemy_spawner.score += 50;
            }
        }
    }
}

fn check_for_stage_end(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity)>,
    mut enemy_spawner_query: Query<&mut EnemySpawner>,
) {
    let mut enemy_spawner: Mut<EnemySpawner>;
    if let Ok(spawner) = enemy_spawner_query.get_single_mut() {
        enemy_spawner = spawner;
    } else {
        return;
    }

    let mut dead_enemies: i32 = 0;
    for (enemy, _) in enemy_query.iter() {
        if enemy.state == EnemyState::Destroyed {
            dead_enemies += 1;
        }
    }

    if dead_enemies as usize >= get_max_enemy_count(enemy_spawner.stage) {
        enemy_spawner.started = false;
        enemy_spawner.stage += 1;
        enemy_spawner.start_time = Instant::now() + Duration::from_secs(3);
        info!("Starting new stage: {:?}", enemy_spawner.stage);

        for (_, entity) in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn move_enemies_toward_player(
    mut enemy_query: Query<(&mut Enemy, &mut ExternalForce, &mut Transform)>,
    player_query: Query<(&Player, &Transform), Without<Enemy>>,
) {
    if let Ok((_, player_transform)) = player_query.get_single() {
        for (enemy, mut external_force, mut enemy_transform) in enemy_query.iter_mut() {
            if enemy.state == EnemyState::Destroyed {
                return;
            }
            let mut rads = (enemy_transform.translation.y - player_transform.translation.y)
                .atan2(enemy_transform.translation.x - player_transform.translation.x);
            rads = rads + (std::f32::consts::PI / 2.);
            enemy_transform.rotation = Quat::from_rotation_z(rads);

            let enemy_direction_vec = Vec2::from_angle(
                enemy_transform.rotation.to_euler(EulerRot::XYZ).2 + (std::f32::consts::PI / 2.),
            );

            external_force.force = enemy_direction_vec * 2.0;
        }
    }
}
