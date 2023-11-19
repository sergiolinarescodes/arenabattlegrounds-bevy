use bevy::prelude::*;

pub struct Spaceship;

#[derive(Component, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Plugin for Spaceship {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
        app.add_systems(Update, update_position);
        app.add_systems(Update, print_position);
    }
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at {:?}", entity, position);
    }
}
