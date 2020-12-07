use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::WindowResized;

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugin(PrintDiagnosticsPlugin::default())
		.add_startup_system(setup.system())
		.add_system(ball_movement_system.system())
		.add_system(paddle_movement_system.system())
		.add_system(window_resize_listener.system())
		.add_system(ball_collision_system.system())
		.run();
}

struct Ball {
	velocity: Vec2,
}

struct Paddle;

impl Paddle {
	const SPEED: f32 = 200.0;
	const WIDTH: f32 = 20.0;
	const MARGIN: f32 = 30.0;
}

enum Player {
	Left,
	Right,
}

impl Player {
	fn start_position(&self) -> Vec2 {
		let x_position = match self {
			Player::Left => -300.0,
			Player::Right => 300.0,
		};

		Vec2::new(x_position, 0.0)
	}

	fn paddle_size_and_translation(&self, width: usize, height: usize) -> (Vec2, Vec3) {
		let size = Vec2::new(Paddle::WIDTH, 0.2 * (height as f32));
		let translation = match self {
			Player::Left => Vec2::new(Paddle::MARGIN - ((width as f32) / 2.0), 0.0),
			Player::Right => Vec2::new(((width as f32) / 2.0) - Paddle::MARGIN, 0.0),
		}
		.extend(0.0);
		(size, translation)
	}

	fn movement_keys(&self) -> (KeyCode, KeyCode) {
		match self {
			Player::Left => (KeyCode::W, KeyCode::S),
			Player::Right => (KeyCode::Up, KeyCode::Down),
		}
	}
}

impl Default for Ball {
	fn default() -> Self {
		const DEFAULT_VELOCITY: f32 = 100.0;
		Self {
			velocity: DEFAULT_VELOCITY * Vec2::new(1.0, 1.0).normalize(),
		}
	}
}

struct Collider;

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dComponents::default());
	spawn_ball(&mut commands);
	spawn_paddle(&mut commands, Player::Left);
	spawn_paddle(&mut commands, Player::Right);
	commands.insert_resource(ClearColor(Color::BLACK));
	commands.insert_resource(WindowDescriptor {
		width: 1280,
		height: 720,
		title: "pong clone".to_string(),
		vsync: true,
		resizable: true,
		..Default::default()
	});
	commands.insert_resource(WindowResizeListenerState::default());
}

fn spawn_ball(commands: &mut Commands) {
	const SIZE: f32 = 50.0;

	commands
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(SIZE, SIZE),
				..Default::default()
			},
			..Default::default()
		})
		.with(Ball::default());
}

fn spawn_paddle(commands: &mut Commands, player: Player) {
	commands
		.spawn(SpriteComponents {
			sprite: Sprite {
				size: Vec2::new(20.0, 200.0),
				..Default::default()
			},
			transform: Transform {
				translation: player.start_position().extend(0.0),
				..Default::default()
			},
			..Default::default()
		})
		.with(Paddle)
		.with(player)
		.with(Collider);
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
	let time_delta = time.delta_seconds;
	for (ball, mut transform) in query.iter_mut() {
		transform.translation += time_delta * ball.velocity.extend(0.0);
	}
}

fn paddle_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Paddle, &Player, &mut Transform)>,
) {
	let time_delta = time.delta_seconds;

	for (_paddle, player, mut transform) in query.iter_mut() {
		let (up_keycode, down_keycode) = player.movement_keys();

		if keyboard_input.pressed(up_keycode) {
			transform.translation += time_delta * Vec2::new(0.0, Paddle::SPEED).extend(0.0);
		}

		if keyboard_input.pressed(down_keycode) {
			transform.translation += time_delta * Vec2::new(0.0, -Paddle::SPEED).extend(0.0);
		}
	}
}

fn ball_collision_system(
	mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
	collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
	for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
		for (_collider, collider_transform, collider_sprite) in collider_query.iter() {
			let collision = collide(
				ball_transform.translation,
				ball_sprite.size,
				collider_transform.translation,
				collider_sprite.size,
			);

			let collision = match collision {
				Some(collision) => collision,
				None => continue,
			};

			use Collision::*;
			let (reflect_x, reflect_y) = match collision {
				Left => (ball.velocity.x() > 0.0, false),
				Right => (ball.velocity.x() < 0.0, false),
				Top => (false, ball.velocity.y() < 0.0),
				Bottom => (false, ball.velocity.y() > 0.0),
			};

			if reflect_x {
				*ball.velocity.x_mut() = -ball.velocity.x();
			}

			if reflect_y {
				*ball.velocity.y_mut() = -ball.velocity.y();
			}
		}
	}
}
#[derive(Default)]
struct WindowResizeListenerState {
	event_reader: EventReader<WindowResized>,
}

fn window_resize_listener(
	mut resize_listener: ResMut<WindowResizeListenerState>,
	resize_events: Res<Events<WindowResized>>,
	mut paddles: Query<(&mut Sprite, &mut Transform, &Paddle, &Player)>,
) {
	if let Some(resize_event) = resize_listener.event_reader.latest(&resize_events) {
		println!("Window resized to {}x{}", resize_event.width, resize_event.height);
		for (mut sprite, mut transform, _paddle, player) in paddles.iter_mut() {
			let (size, translation) = player.paddle_size_and_translation(resize_event.width, resize_event.height);
			sprite.size = size;
			transform.translation = translation;
		}
	}
}
