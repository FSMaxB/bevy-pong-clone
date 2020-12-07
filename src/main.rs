use crate::ball::Ball;
use crate::ball::{ball_collision_system, ball_movement_system};
use crate::paddle::paddle_movement_system;
use crate::paddle::Paddle;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowResized;

mod ball;
mod paddle;

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

#[derive(Clone, Copy)]
pub enum Player {
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

pub struct Collider;

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
		.with(Paddle::default())
		.with(player)
		.with(Collider);
}

#[derive(Default)]
struct WindowResizeListenerState {
	event_reader: EventReader<WindowResized>,
}

fn window_resize_listener(
	mut resize_listener: ResMut<WindowResizeListenerState>,
	resize_events: Res<Events<WindowResized>>,
	mut paddles: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
	mut ball: Query<(&mut Ball, &mut Sprite, &mut Transform)>,
) {
	if let Some(resize_event) = resize_listener.event_reader.latest(&resize_events) {
		let width = resize_event.width;
		let height = resize_event.height;
		println!("Window resized to {}x{}", width, height);

		for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
			paddle.update_after_window_resize(resize_event, *player, &mut sprite.size, &mut transform.translation);
		}

		for (mut ball, mut sprite, mut transform) in ball.iter_mut() {
			ball.update_after_window_resize(resize_event, &mut sprite.size, &mut transform.translation);
		}
	}
}
