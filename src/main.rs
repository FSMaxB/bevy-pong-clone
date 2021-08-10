use crate::ball::spawn_ball;
use crate::ball::{ball_collision_system, ball_movement_system, ball_resize_system};
use crate::goal::{goal_collision_system, goal_resize_system, spawn_goals};
use crate::paddle::spawn_paddles;
use crate::paddle::{paddle_movement_system, paddle_resize_system};
use crate::score::{scoreboard_update_system, spawn_score_board, Score, Scored};
use crate::wall::{spawn_walls, wall_resize_system};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

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
		.add_event::<Scored>()
		.insert_resource(Score::default())
		.add_startup_system(setup.system())
		.add_system(ball_movement_system.system())
		.add_system(paddle_movement_system.system())
		.add_system(paddle_resize_system.system())
		.add_system(ball_collision_system.system())
		.add_system(ball_resize_system.system())
		.add_system(goal_collision_system.system())
		.add_system(goal_resize_system.system())
		.add_system(wall_resize_system.system())
		.add_system(scoreboard_update_system.system())
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
