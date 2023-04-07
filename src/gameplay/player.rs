use crate::{loading::AudioHandles, states::AppState};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};
use instant::{Duration, Instant};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(pause.in_set(OnUpdate(AppState::InGame)));
        app.add_system(rotate_player.in_set(OnUpdate(AppState::InGame)));
        app.add_system(
            player_click
                .after(move_player)
                .in_set(OnUpdate(AppState::InGame)),
        );
        app.add_system(move_player.in_set(OnUpdate(AppState::InGame)));
        app.add_system(despawn_collided_bullets.in_set(OnUpdate(AppState::InGame)));
        app.add_system(animate_player.in_set(OnUpdate(AppState::InGame)));
        app.add_system(process_sounds.in_set(OnUpdate(AppState::InGame)));
        app.add_system(detect_player_death.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub shells: i32,
}

#[derive(Component, Default)]
pub struct Bullet {
    pub collided: bool,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
pub struct PlayerSound {
    pub time_to_play: Instant,
    pub sound_type: PlayerSoundType,
}

#[derive(PartialEq)]
pub enum PlayerSoundType {
    Cock,
    Gunshot,
    Shell,
    Bite,
    Empty,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    player_query: Query<&Player>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for _ in player_query.iter() {
        return;
    }

    let texture_handle = asset_server.load("sprites/hotwife.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 6, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 5 };

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Collider::ball(10.0))
        .insert(ColliderMassProperties::Density(0.8))
        .insert(CollisionGroups::new(Group::ALL, Group::ALL))
        .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 1.0,
        })
        .insert(Player {
            health: 100,
            shells: 6,
        })
        .insert((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 2.0)));
    info!("spawned player")
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &Player,
        &ExternalForce,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, _, force, mut timer, mut sprite) in &mut query {
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
                sprite.index = 0;
            }
        }
    }
}

fn pause(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

fn rotate_player(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut player_query: Query<(&Player, &mut Transform)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();

    if let Some(event) = cursor_moved_events.iter().next() {
        for (_, mut t) in player_query.iter_mut() {
            if let Some(position) = camera
                .viewport_to_world(camera_transform, event.position)
                .map(|ray| ray.origin.truncate())
            {
                let mut rads = (t.translation.y - position.y).atan2(t.translation.x - position.x);
                rads = rads + (std::f32::consts::PI / 2.);
                t.rotation = Quat::from_rotation_z(rads);
                // info!("{:?}", position)
            }
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalForce, &mut Player)>,
) {
    for (mut external_force, mut player) in player_query.iter_mut() {
        external_force.force.x = 0.0;
        external_force.force.y = 0.0;

        if keyboard_input.pressed(KeyCode::W) {
            external_force.force.y = 15.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            external_force.force.x = -15.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            external_force.force.y = -15.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            external_force.force.x = 15.0;
        }

        if keyboard_input.pressed(KeyCode::R) {
            player.shells = 6;
        }
    }
}

fn player_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    rng: ResMut<GlobalRng>,
    mut player_query: Query<(&mut Player, &Transform, &mut ExternalForce)>,
    mut commands: Commands,
) {
    if let Some((mut player, transform, mut force)) = player_query.iter_mut().next() {
        if let Some(click_event) = mouse_button_input_events.iter().next() {
            if click_event.state == ButtonState::Pressed {
                if player.shells > 0 {
                    commands.spawn(PlayerSound {
                        time_to_play: Instant::now(),
                        sound_type: PlayerSoundType::Gunshot,
                    });

                    commands.spawn(PlayerSound {
                        time_to_play: Instant::now() + Duration::from_millis(300),
                        sound_type: PlayerSoundType::Cock,
                    });

                    commands.spawn(PlayerSound {
                        time_to_play: Instant::now() + Duration::from_millis(600),
                        sound_type: PlayerSoundType::Shell,
                    });

                    spawn_shotgun_blast(commands, transform, rng, force);

                    player.shells -= 1;
                } else {
                    commands.spawn(PlayerSound {
                        time_to_play: Instant::now(),
                        sound_type: PlayerSoundType::Empty,
                    });
                }
            }
        }
    }
}

fn process_sounds(
    mut commands: Commands,
    sound_query: Query<(&PlayerSound, Entity)>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
) {
    let now = Instant::now();
    for (player_sound, entity) in sound_query.iter() {
        if player_sound.time_to_play < now {
            match player_sound.sound_type {
                PlayerSoundType::Cock => {
                    audio.play(Handle::weak(audio_handles.cock.id()));
                }
                PlayerSoundType::Gunshot => {
                    audio.play(Handle::weak(audio_handles.gunshot.id()));
                }
                PlayerSoundType::Shell => {
                    audio.play(Handle::weak(audio_handles.shell.id()));
                }
                PlayerSoundType::Bite => {
                    audio.play(Handle::weak(audio_handles.bite.id()));
                }
                PlayerSoundType::Empty => {
                    audio.play(Handle::weak(audio_handles.empty.id()));
                }
            }

            commands.entity(entity).despawn();
        }
    }
}

fn spawn_shotgun_blast(
    mut commands: Commands,
    player_transform: &Transform,
    mut rng: ResMut<GlobalRng>,
    mut force: Mut<ExternalForce>,
) {
    const BULLET_SPAWN_DISTANCE_MULTIPLIER: f32 = 15.0;

    let player_facing = Vec2::from_angle(
        player_transform.rotation.to_euler(EulerRot::XYZ).2 + (std::f32::consts::PI / 2.),
    );

    let recoil_direction = Vec2::new(player_facing.x * -1.0, player_facing.y * -1.0);
    force.force = recoil_direction * 300.0;

    let initial_bullet_location = Vec2::new(
        player_transform.translation.x + (player_facing.x * BULLET_SPAWN_DISTANCE_MULTIPLIER),
        player_transform.translation.y + (player_facing.y * BULLET_SPAWN_DISTANCE_MULTIPLIER),
    );

    spawn_bullet(
        &mut commands,
        initial_bullet_location,
        player_facing,
        get_shot_randomness(&mut rng),
        get_shot_randomness(&mut rng),
    );
    spawn_bullet(
        &mut commands,
        initial_bullet_location,
        player_facing,
        get_shot_randomness(&mut rng),
        get_shot_randomness(&mut rng),
    );
    spawn_bullet(
        &mut commands,
        initial_bullet_location,
        player_facing,
        get_shot_randomness(&mut rng),
        get_shot_randomness(&mut rng),
    );
    spawn_bullet(
        &mut commands,
        initial_bullet_location,
        player_facing,
        get_shot_randomness(&mut rng),
        get_shot_randomness(&mut rng),
    );
    spawn_bullet(
        &mut commands,
        initial_bullet_location,
        player_facing,
        get_shot_randomness(&mut rng),
        get_shot_randomness(&mut rng),
    );
}

fn get_shot_randomness(rng: &mut ResMut<GlobalRng>) -> f32 {
    rng.f32_normalized() * 0.2
}

fn spawn_bullet(
    commands: &mut Commands,
    initial_bullet_location: Vec2,
    player_facing: Vec2,
    shot_offset_x: f32,
    shot_offset_y: f32,
) {
    const BULLET_FORCE_MULTIPLIER: f32 = 10.0;

    let bullet_force = Vec2::new(
        (player_facing.x + shot_offset_x) * BULLET_FORCE_MULTIPLIER,
        (player_facing.y + shot_offset_y) * BULLET_FORCE_MULTIPLIER,
    );

    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            initial_bullet_location.x,
            initial_bullet_location.y,
            5.0,
        )))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(ExternalForce {
            force: bullet_force,
            torque: 0.0,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::ALL.difference(Group::GROUP_1),
        ))
        .insert(Collider::cuboid(1.0, 1.0))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(Bullet {
            ..Default::default()
        });
}

fn despawn_collided_bullets(mut commands: Commands, bullet_query: Query<(&Bullet, Entity)>) {
    for (bullet, entity) in bullet_query.iter() {
        if bullet.collided == true {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn detect_player_death(mut next_state: ResMut<NextState<AppState>>, player_query: Query<&Player>) {
    for player in player_query.iter() {
        if player.health <= 0 {
            next_state.set(AppState::Scoreboard);
        }
    }
}
