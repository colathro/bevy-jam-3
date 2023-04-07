use crate::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::states::AppState;
use bevy::prelude::*;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_menu.in_schedule(OnEnter(AppState::Cutscene)));
        app.add_system(spawn_background.in_schedule(OnEnter(AppState::Cutscene)));
        app.add_system(process_menu.in_set(OnUpdate(AppState::Cutscene)));
        app.add_system(cutscene_continue.in_set(OnUpdate(AppState::Cutscene)));
        app.add_system(update_cutscene.in_set(OnUpdate(AppState::Cutscene)));
        app.add_system(destroy_menu.in_schedule(OnExit(AppState::Cutscene)));
    }
}

#[derive(Resource)]
struct CutsceneEntities {
    skip: Entity,
    cutscene: Entity,
}

#[derive(Component)]
struct Cutscene {
    stage: i32,
    next_stage: i32,
    scenes: Vec<CutsceneInfo>,
}

#[derive(Component)]
struct ActiveCutscene;

#[derive(Component)]
struct CutsceneNode;

#[derive(Component)]
struct CutsceneBackground;

struct CutsceneInfo {
    doc_text: String,
    wife_text: String,
    baby_text: String,
    wife_big_boob: bool,
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skip = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(CutsceneNode)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "space to continue",
                    TextStyle {
                        font: asset_server.load("fonts/pixelsplitter.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ))
                .insert(BackgroundColor(Color::NONE));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "Skip",
                            TextStyle {
                                font: asset_server.load("fonts/pixelsplitter.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(BackgroundColor(Color::NONE));
                });
        })
        .id();

    let scenes = vec![
        CutsceneInfo {
            doc_text: "So you understand the possible \n side effects right?".to_string(),
            wife_text: "".to_string(),
            baby_text: "".to_string(),
            wife_big_boob: false,
        },
        CutsceneInfo {
            doc_text: "".to_string(),
            wife_text: "Yeah yeah. \n Just give me the pill already!".to_string(),
            baby_text: "".to_string(),
            wife_big_boob: false,
        },
        CutsceneInfo {
            doc_text: "Here you go....".to_string(),
            wife_text: "".to_string(),
            baby_text: "".to_string(),
            wife_big_boob: false,
        },
        CutsceneInfo {
            doc_text: "".to_string(),
            wife_text: "*gulp*".to_string(),
            baby_text: "".to_string(),
            wife_big_boob: false,
        },
        CutsceneInfo {
            doc_text: "".to_string(),
            wife_text: "Oh my gosh! This is amazing! \n But something isn't right....".to_string(),
            baby_text: "".to_string(),
            wife_big_boob: true,
        },
        CutsceneInfo {
            doc_text: "".to_string(),
            wife_text: "".to_string(),
            baby_text: "*RAWWWWRRRR*".to_string(),
            wife_big_boob: true,
        },
    ];

    spawn_scene(&mut commands, &scenes[0], asset_server, skip);

    let cutscene = commands
        .spawn(Cutscene {
            stage: 0,
            next_stage: 0,
            scenes,
        })
        .id();

    commands.insert_resource(CutsceneEntities { skip, cutscene });
}

fn spawn_scene(
    commands: &mut Commands,
    cutscene_info: &CutsceneInfo,
    asset_server: Res<AssetServer>,
    cutscene_entity: Entity,
) {
    if !cutscene_info.doc_text.is_empty() {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/Doctor.png"),
                ..default()
            })
            .insert(TransformBundle {
                local: Transform {
                    translation: Vec3 {
                        x: -240.0,
                        y: -120.0,
                        z: 2.0,
                    },
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(ActiveCutscene);

        commands.entity(cutscene_entity).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveCutscene)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", cutscene_info.doc_text),
                            TextStyle {
                                font: asset_server.load("fonts/pixelsplitter.ttf"),
                                font_size: 28.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(BackgroundColor(Color::NONE));
                });
        });
    } else if !cutscene_info.wife_text.is_empty() {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/Wife.png"),
                ..default()
            })
            .insert(TransformBundle {
                local: Transform {
                    translation: Vec3 {
                        x: 240.0,
                        y: -120.0,
                        z: 2.0,
                    },
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(ActiveCutscene);

        commands.entity(cutscene_entity).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Start,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveCutscene)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", cutscene_info.wife_text),
                            TextStyle {
                                font: asset_server.load("fonts/pixelsplitter.ttf"),
                                font_size: 28.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(BackgroundColor(Color::NONE));
                });
        });
    } else {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/Baby.png"),
                ..default()
            })
            .insert(TransformBundle {
                local: Transform {
                    translation: Vec3 {
                        x: -240.0,
                        y: -120.0,
                        z: 2.0,
                    },
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(ActiveCutscene);

        commands.entity(cutscene_entity).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveCutscene)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("{}", cutscene_info.baby_text),
                            TextStyle {
                                font: asset_server.load("fonts/pixelsplitter.ttf"),
                                font_size: 28.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(BackgroundColor(Color::NONE));
                });
        });
    }
}

fn process_menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].style.color = PRESSED_BUTTON.into();
                    }
                }
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].style.color = Color::RED;
                    }
                }
            }
            Interaction::None => {
                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].style.color = Color::WHITE;
                    }
                }
            }
        }
    }
}

fn cutscene_continue(
    mut cutscene_query: Query<&mut Cutscene>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut cutscene) = cutscene_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            cutscene.next_stage += 1;
        }
    }
}

fn update_cutscene(
    mut commands: Commands,
    mut cutscene_query: Query<&mut Cutscene>,
    active_cutscene_query: Query<(&ActiveCutscene, Entity)>,
    cutscene_node_query: Query<(&CutsceneNode, Entity)>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((_, entity)) = cutscene_node_query.get_single() {
        if let Ok(mut cutscene) = cutscene_query.get_single_mut() {
            if cutscene.stage != cutscene.next_stage {
                cutscene.stage = cutscene.next_stage;
                if cutscene.stage >= (cutscene.scenes.len()) as i32 {
                    next_state.set(AppState::InGame);
                } else {
                    for (_, entity) in active_cutscene_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    spawn_scene(
                        &mut commands,
                        &cutscene.scenes[cutscene.stage as usize],
                        asset_server,
                        entity,
                    );
                }
            }
        }
    }
}

fn destroy_menu(
    mut commands: Commands,
    menu_data: Res<CutsceneEntities>,
    active_query: Query<(&ActiveCutscene, Entity)>,
    background_query: Query<(&CutsceneBackground, Entity)>,
) {
    for (_, entity) in active_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (_, entity) in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.entity(menu_data.skip).despawn_recursive();
    commands.entity(menu_data.cutscene).despawn_recursive();
}

fn spawn_background(mut commands: Commands, map_query: Query<&CutsceneBackground>) {
    for _ in map_query.iter() {
        return;
    }

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            ..default()
        })
        .insert(CutsceneBackground)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));
}
