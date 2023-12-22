use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::physics::PIXELS_PER_METER;

#[derive(Eq, PartialEq)]
enum Side {
    Top,
    Bottom,
}

#[derive(Component)]
pub struct Wall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_walls);
    }
}

fn create_walls(mut commands: Commands) {
    create_wall(&mut commands, Side::Top);
    create_wall(&mut commands, Side::Bottom);
}

fn create_wall(commands: &mut Commands, side: Side) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    0.,
                    if side == Side::Bottom { -1. } else { 1. } * 1.5 * PIXELS_PER_METER,
                    0.,
                ),
                scale: Vec3::new(5. * PIXELS_PER_METER, 0.1 * PIXELS_PER_METER, 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::all(),
        Collider::cuboid(5., 0.4),
    ));
}
