use crate::ball::{ball_collision_system, ball_movement_system};
use crate::ball::{spawn_ball, Ball};
use crate::goal::{goal_collision_system, spawn_goals, Goal};
use crate::paddle::paddle_movement_system;
use crate::paddle::{spawn_paddles, Paddle};
use crate::score::{spawn_score_board, Score};
use crate::wall::{spawn_walls, Wall};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowResized;
use std::ops::Deref;

mod ball;
mod goal;
mod paddle;
mod score;
mod wall;

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugin(LogDiagnosticsPlugin::default())
		.insert_resource(Score::default())
		.add_startup_system(setup.system())
		.add_system(ball_movement_system.system())
		.add_system(paddle_movement_system.system())
		.add_system(window_resize_listener.system())
		.add_system(ball_collision_system.system())
		.add_system(goal_collision_system.system())
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn().insert_bundle(UiCameraBundle::default());
	spawn_ball(&mut commands);
	spawn_paddles(&mut commands);
	spawn_walls(&mut commands);
	spawn_goals(&mut commands);
	spawn_score_board(&mut commands, &asset_server);
	commands.insert_resource(ClearColor(Color::BLACK));
	commands.insert_resource(WindowDescriptor {
		width: 1280.0,
		height: 720.0,
		title: "pong clone".to_string(),
		vsync: true,
		resizable: true,
		..Default::default()
	});
}

fn window_resize_listener(
	mut resize_reader: EventReader<WindowResized>,
	mut query_set: QuerySet<(
		Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
		Query<(&mut Sprite, &mut Transform, &Wall)>,
		Query<(&mut Sprite, &mut Transform, &Goal, &Player)>,
		Query<(&mut Sprite, &mut Transform, &mut Ball)>,
	)>,
	score: Res<Score>,
) {
	if let Some(resize_event) = resize_reader.iter().last() {
		println!("Score: {}", score.deref());
		let width = resize_event.width;
		let height = resize_event.height;
		println!("Window resized to {}x{}", width, height);

		let paddles = query_set.q0_mut();
		for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
			paddle.update_after_window_resize(resize_event, *player, &mut sprite.size, &mut transform.translation);
		}

		let walls = query_set.q1_mut();
		for (mut sprite, mut transform, wall) in walls.iter_mut() {
			wall.update_after_window_resize(resize_event, &mut sprite.size, &mut transform.translation);
		}

		let goals = query_set.q2_mut();
		for (mut sprite, mut transform, goal, player) in goals.iter_mut() {
			goal.update_after_window_resize(resize_event, *player, &mut sprite.size, &mut transform.translation);
		}

		let ball = query_set.q3_mut();
		for (mut sprite, mut transform, mut ball) in ball.iter_mut() {
			ball.update_after_window_resize(resize_event, &mut sprite.size, &mut transform.translation);
		}
	}
}
