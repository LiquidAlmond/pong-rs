use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    participant::{Ball, Player, ResetBallEvent, Side},
    physics::PIXELS_PER_METER,
};

pub const SCORE_FONT_SIZE: f32 = 40.;
pub const SCORE_PADDING: Val = Val::Px(5.);
pub const SCORE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component, Clone)]
pub struct Score(pub i8);

#[derive(Component)]
pub struct Goal;

#[derive(Event)]
pub struct ScoreGoalEvent(pub Side);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreGoalEvent>()
            .add_systems(Startup, (create_scoreboard, create_goals))
            .add_systems(
                Update,
                (update_scores, cheat_score, process_goal, score_goal),
            );
    }
}

fn create_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: SCORE_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCORE_PADDING,
            left: SCORE_PADDING,
            ..default()
        }),
        Player(Side::Left),
        Score(0),
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: SCORE_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCORE_PADDING,
            right: SCORE_PADDING,
            ..default()
        }),
        Player(Side::Right),
        Score(0),
    ));
}

fn create_goals(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(2.5 * PIXELS_PER_METER, 0., 0.),
                scale: Vec3::new(0.1 * PIXELS_PER_METER, 1.5 * PIXELS_PER_METER, 0.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        Collider::cuboid(0.4, 3.),
        Goal,
        Player(Side::Left),
        Sensor,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-2.5 * PIXELS_PER_METER, 0., 0.),
                scale: Vec3::new(0.1 * PIXELS_PER_METER, 1.5 * PIXELS_PER_METER, 0.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        Collider::cuboid(0.4, 3.),
        Goal,
        Player(Side::Right),
        Sensor,
    ));
}

fn process_goal(
    mut events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    goals: Query<(Entity, &Player), With<Goal>>,
    mut score: EventWriter<ScoreGoalEvent>,
    mut reset_ball: EventWriter<ResetBallEvent>,
) {
    if events.is_empty() {
        return;
    }

    let ball = ball_query.single();

    for event in events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if entity1.ne(&ball) && entity2.ne(&ball) {
                    continue;
                }

                for (goal, player) in &goals {
                    if entity1.ne(&goal) && entity2.ne(&goal) {
                        continue;
                    }

                    score.send(ScoreGoalEvent(player.0));
                    reset_ball.send(ResetBallEvent);
                }
            }
            _ => {}
        }
    }
}

fn score_goal(
    mut events: EventReader<ScoreGoalEvent>,
    mut score_query: Query<(&mut Score, &Player)>,
) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        for (mut score, player) in &mut score_query {
            if player.0 == event.0 {
                score.0 += 1;
            }
        }
    }
}

fn update_scores(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in &mut query {
        text.sections[0].value = score.0.to_string();
    }
}

fn cheat_score(input: Res<Input<KeyCode>>, mut query: Query<(&mut Score, &Player)>) {
    for (mut score, player) in &mut query {
        if player.0 == Side::Left {
            if input.just_pressed(KeyCode::BracketLeft) {
                score.0 += 1;
            }
        } else {
            if input.just_pressed(KeyCode::BracketRight) {
                score.0 += 1;
            }
        }
    }
}
