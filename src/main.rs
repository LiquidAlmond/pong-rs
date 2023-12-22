use bevy::prelude::*;

mod graphics;
mod participant;
mod physics;
mod score;
mod wall;

use crate::{graphics::*, participant::*, physics::*, score::*, wall::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Pong".to_string(),
                resolution: (5. * PIXELS_PER_METER, 3. * PIXELS_PER_METER).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            ScorePlugin,
            PhysicsPlugin,
            WallPlugin,
            ParticipantPlugin,
            GraphicsPlugin,
        ))
        .run();
}
