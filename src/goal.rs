use crate::Player;
use bevy::ecs::Commands;
use bevy::math::{Vec2, Vec3};
use bevy::sprite::entity::SpriteBundle;
use bevy::window::WindowResized;

pub struct Goal;

impl Goal {
	const THICKNESS: f32 = 20.0;

	pub fn update_after_window_resize(
		&self,
		resize_event: &WindowResized,
		player: Player,
		size: &mut Vec2,
		translation: &mut Vec3,
	) {
		let window_width = resize_event.width as f32;
		let window_height = resize_event.height as f32;
		*size = Vec2::new(Self::THICKNESS, window_height);

		use Player::*;
		let x_offset = (window_width - Self::THICKNESS) / 2.0;
		let x_position = match player {
			Left => x_offset,
			Right => -x_offset,
		};
		*translation = Vec3::new(x_position, 0.0, 0.0);
	}
}

pub fn spawn_goals(commands: &mut Commands) {
	spawn_goal(commands, Player::Left);
	spawn_goal(commands, Player::Right);
}

fn spawn_goal(commands: &mut Commands, player: Player) {
	commands.spawn(SpriteBundle::default()).with(Goal).with(player);
}
