use bevy::prelude::*;

fn main() {
	App::build()
		.add_default_plugins()
		.add_startup_system(setup.system())
		.add_system(ball_movement_system.system())
		.run();
}

struct Ball {
	velocity: Vec2,
}

struct Paddle;

impl Default for Ball {
	fn default() -> Self {
		const DEFAULT_VELOCITY: f32 = 100.0;
		Self {
			velocity: DEFAULT_VELOCITY * Vec2::new(1.0, 1.0).normalize(),
		}
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dComponents::default());
	spawn_ball(&mut commands);
	spawn_paddle(&mut commands, Vec2::new(-300.0, 0.0));
	spawn_paddle(&mut commands, Vec2::new(300.0, 0.0));
}

fn spawn_ball(commands: &mut Commands) {
	const SIZE: f32 = 50.0;

	commands
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(SIZE, SIZE),
			},
			rotation: Rotation::from_rotation_z(std::f32::consts::PI / 4.0),
			..Default::default()
		})
		.with(Ball::default());
}

fn spawn_paddle(commands: &mut Commands, start_position: Vec2) {
	commands
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(20.0, 200.0),
			},
			translation: Translation(start_position.extend(0.0)),
			..Default::default()
		})
		.with(Paddle);
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Translation)>) {
	let time_delta = time.delta_seconds;
	for (ball, mut translation) in &mut query.iter() {
		translation.0 += time_delta * ball.velocity.extend(0.0);
	}
}
