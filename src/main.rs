use bevy::{
    prelude::*,
    time::FixedTimestep,
};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]

struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(apply_velocity.before(check_for_collisions)),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
}

fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
