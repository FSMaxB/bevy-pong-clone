use crate::Collider;
use bevy::ecs::Commands;
use bevy::math::{Vec2, Vec3};
use bevy::sprite::entity::SpriteBundle;
use bevy::window::WindowResized;

pub enum Wall {
	Top,
	Bottom,
}

impl Wall {
	const THICKNESS: f32 = 20.0;

	pub fn update_after_window_resize(&self, resize_event: &WindowResized, size: &mut Vec2, translation: &mut Vec3) {
		let window_width = resize_event.width as f32;
		let window_height = resize_event.height as f32;
		*size = Vec2::new(window_width, Self::THICKNESS);

		use Wall::*;
		let y_offset = (window_height - Self::THICKNESS) / 2.0;
		let y_position = match self {
			Top => y_offset,
			Bottom => -y_offset,
		};
		*translation = Vec3::new(0.0, y_position, 0.0);
	}
}

pub fn spawn_walls(commands: &mut Commands) {
	spawn_wall(commands, Wall::Top);
	spawn_wall(commands, Wall::Bottom);
}

fn spawn_wall(commands: &mut Commands, wall: Wall) {
	commands.spawn(SpriteBundle::default()).with(wall).with(Collider);
}
