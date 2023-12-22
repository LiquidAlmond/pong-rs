use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::random;

use crate::physics::PIXELS_PER_METER;

const PADDLE_WIDTH: f32 = 0.1;
const PADDLE_LENGTH: f32 = 0.5;
const PADDLE_SIZE: Vec3 = Vec3::new(PADDLE_WIDTH, PADDLE_LENGTH, 0.);
const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SPEED: f32 = 0.2;

const BALL_COLOR: Color = Color::WHITE;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player(pub Side);

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Event)]
pub struct ResetBallEvent;

pub struct ParticipantPlugin;

impl Plugin for ParticipantPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetBallEvent>()
            .add_systems(Startup, (create_paddles, create_ball))
            .add_systems(Update, (player_input, computer_input, process_ball_reset));
    }
}

fn create_paddles(mut commands: Commands) {
    create_paddle(&mut commands, Side::Left);
    create_paddle(&mut commands, Side::Right);
}

fn create_paddle(commands: &mut Commands, side: Side) {
    commands.spawn((
        Player(side),
        Paddle,
        Collider::cuboid(4. * PADDLE_SIZE.x, PADDLE_SIZE.y),
        Velocity::zero(),
        RigidBody::Dynamic,
        LockedAxes::from_bits_retain(
            LockedAxes::TRANSLATION_LOCKED_X.bits() | LockedAxes::ROTATION_LOCKED.bits(),
        ),
        Damping {
            linear_damping: 1.,
            angular_damping: 1.,
        },
        ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.,
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    if side == Side::Right { 1. } else { -1. } * 2.3 * PIXELS_PER_METER,
                    0.,
                    0.,
                ),
                scale: PADDLE_SIZE * PIXELS_PER_METER,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
    ));
}

fn create_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ball,
        RigidBody::Dynamic,
        Collider::ball(0.05),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        Restitution {
            coefficient: 1.1,
            combine_rule: CoefficientCombineRule::Max,
        },
        Velocity {
            linvel: Vec2::from_angle((random::<f32>() - 0.5) * 30.) * 100.,
            angvel: 0.,
        },
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(0.05 * PIXELS_PER_METER).into())
                .into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            ..default()
        },
    ));
}

fn player_input(
    mut query: Query<(&mut ExternalImpulse, &Player), With<Paddle>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut impulse, player) in &mut query {
        if player.0 != Side::Left {
            continue;
        }

        let mut direction = 0.0;

        if input.pressed(KeyCode::Up) {
            direction += 1.0;
        }

        if input.pressed(KeyCode::Down) {
            direction -= 1.0;
        }

        impulse.impulse.y = direction * PADDLE_SPEED;
    }
}

fn computer_input(
    mut query: Query<(&mut ExternalImpulse, &Transform, &Player), With<Paddle>>,
    ball_query: Query<(&Velocity, &Transform), (With<Ball>, Without<Paddle>)>,
) {
    let (ball_velocity, ball) = ball_query.single();

    for (mut impulse, paddle, player) in &mut query {
        if player.0 != Side::Right {
            continue;
        }

        let mut direction = 0.0;

        if ball.translation.y > paddle.translation.y && ball_velocity.linvel.x > 0. {
            direction += 1.;
        }
        if ball.translation.y < paddle.translation.y && ball_velocity.linvel.x > 0. {
            direction -= 1.;
        }

        impulse.impulse.y = direction * PADDLE_SPEED;
    }
}

fn process_ball_reset(
    mut events: EventReader<ResetBallEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    if events.is_empty() {
        return;
    }

    let (mut transform, mut velocity) = ball_query.single_mut();

    velocity.linvel = Vec2::from_angle((random::<f32>() % 1. - 0.5) * PI / 6.) * 100.;
    transform.translation.x = 0.;
    transform.translation.y = 0.;

    events.clear();
}
