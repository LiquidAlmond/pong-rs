use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PIXELS_PER_METER: f32 = 100.;

#[derive(Resource)]
pub struct PixelsPerMeter(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelsPerMeter(PIXELS_PER_METER))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                PIXELS_PER_METER,
            ))
            //.add_plugins(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..default()
            });
    }
}
