use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
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
		.run();
}

struct Ball {
	velocity: Vec2,
}

struct Paddle;

impl Paddle {
	const SPEED: f32 = 200.0;
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
			transform: Transform {
				rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
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
		.with(player);
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

#[derive(Default)]
struct WindowResizeListenerState {
	event_reader: EventReader<WindowResized>,
}

fn window_resize_listener(
	mut resize_listener: ResMut<WindowResizeListenerState>,
	resize_events: Res<Events<WindowResized>>,
) {
	if let Some(resize_event) = resize_listener.event_reader.latest(&resize_events) {
		println!("Window resized to {}x{}", resize_event.width, resize_event.height);
	}
}
