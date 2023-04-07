use crate::states::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

        let sync_backend_system_set =
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend);
        let sync_backend_flush_system_set =
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackendFlush);
        let step_simulation_system_set =
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation);
        let writeback_system_set =
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback);

        app.add_plugin(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)
                .with_default_system_setup(false),
        );

        app.configure_sets(
            (
                PhysicsSet::SyncBackend,
                PhysicsSet::SyncBackendFlush,
                PhysicsSet::StepSimulation,
                PhysicsSet::Writeback,
            )
                .chain()
                .after(CoreSet::UpdateFlush)
                .before(CoreSet::PostUpdate),
        );

        app.add_systems(
            sync_backend_system_set
                //.distributive_run_if(in_state(AppState::InGame))
                .in_base_set(PhysicsSet::SyncBackend),
        );
        app.add_systems(
            sync_backend_flush_system_set
                .distributive_run_if(in_state(AppState::InGame))
                .in_base_set(PhysicsSet::SyncBackendFlush),
        );
        app.add_systems(
            step_simulation_system_set
                .distributive_run_if(in_state(AppState::InGame))
                .in_base_set(PhysicsSet::StepSimulation),
        );
        app.add_systems(
            writeback_system_set
                .distributive_run_if(in_state(AppState::InGame))
                .in_base_set(PhysicsSet::Writeback),
        );

        //app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
