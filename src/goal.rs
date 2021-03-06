use crate::ball::Ball;
use crate::score::Score;
use crate::Player;
use bevy::ecs::system::{Commands, Query, ResMut};
use bevy::math::{Vec2, Vec3};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::entity::SpriteBundle;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use bevy::window::WindowResized;
use std::ops::DerefMut;

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
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(Goal)
		.insert(player);
}

pub fn goal_collision_system(
	ball_query: Query<(&Ball, &Transform, &Sprite)>,
	goal_query: Query<(&Transform, &Sprite, &Goal, &Player)>,
	mut score: ResMut<Score>,
) {
	for (_ball, ball_transform, ball_sprite) in ball_query.iter() {
		for (goal_transform, goal_sprite, _goal, player) in goal_query.iter() {
			let collision = collide(
				ball_transform.translation,
				ball_sprite.size,
				goal_transform.translation,
				goal_sprite.size,
			);

			if collision.is_some() {
				use Player::*;
				match player {
					Left => score.deref_mut().left += 1,
					Right => score.deref_mut().right += 1,
				}
			}
		}
	}
}
