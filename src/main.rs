use crate::ball::{ball_collision_system, ball_movement_system};
use crate::ball::{spawn_ball, Ball};
use crate::goal::{spawn_goals, Goal};
use crate::paddle::paddle_movement_system;
use crate::paddle::{spawn_paddles, Paddle};
use crate::wall::{spawn_walls, Wall};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowResized;

mod ball;
mod goal;
mod paddle;
mod wall;

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
	spawn_paddles(&mut commands);
	spawn_walls(&mut commands);
	spawn_goals(&mut commands);
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

#[derive(Default)]
struct WindowResizeListenerState {
	event_reader: EventReader<WindowResized>,
}

fn window_resize_listener(
	mut resize_listener: ResMut<WindowResizeListenerState>,
	resize_events: Res<Events<WindowResized>>,
	mut paddles: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
	mut walls: Query<(&mut Sprite, &mut Transform, &Wall)>,
	mut goals: Query<(&mut Sprite, &mut Transform, &Goal, &Player)>,
	mut ball: Query<(&mut Ball, &mut Sprite, &mut Transform)>,
) {
	if let Some(resize_event) = resize_listener.event_reader.latest(&resize_events) {
		let width = resize_event.width;
		let height = resize_event.height;
		println!("Window resized to {}x{}", width, height);

		for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
			paddle.update_after_window_resize(resize_event, *player, &mut sprite.size, &mut transform.translation);
		}

		for (mut sprite, mut transform, wall) in walls.iter_mut() {
			wall.update_after_window_resize(resize_event, &mut sprite.size, &mut transform.translation);
		}

		for (mut sprite, mut transform, goal, player) in goals.iter_mut() {
			goal.update_after_window_resize(resize_event, *player, &mut sprite.size, &mut transform.translation);
		}

		for (mut ball, mut sprite, mut transform) in ball.iter_mut() {
			ball.update_after_window_resize(resize_event, &mut sprite.size, &mut transform.translation);
		}
	}
}
