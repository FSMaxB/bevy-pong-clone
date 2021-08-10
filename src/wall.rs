use crate::Collider;
use bevy::app::EventReader;
use bevy::ecs::prelude::Query;
use bevy::ecs::system::Commands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Sprite, Transform};
use bevy::sprite::entity::SpriteBundle;
use bevy::window::WindowResized;

pub enum Wall {
	Top,
	Bottom,
}

impl Wall {
	pub const THICKNESS: f32 = 20.0;
}

pub fn wall_resize_system(
	mut resize_reader: EventReader<WindowResized>,
	mut query: Query<(&mut Sprite, &mut Transform, &Wall)>,
) {
	let resize_event = match resize_reader.iter().last() {
		Some(event) => event,
		None => return,
	};

	for (mut sprite, mut transform, wall) in query.iter_mut() {
		let window_width = resize_event.width as f32;
		let window_height = resize_event.height as f32;
		sprite.size = Vec2::new(window_width, Wall::THICKNESS);

		use Wall::*;
		let y_offset = (window_height - Wall::THICKNESS) / 2.0;
		let y_position = match wall {
			Top => y_offset,
			Bottom => -y_offset,
		};
		transform.translation = Vec3::new(0.0, y_position, 0.0);
	}
}

pub fn spawn_walls(commands: &mut Commands) {
	spawn_wall(commands, Wall::Top);
	spawn_wall(commands, Wall::Bottom);
}

fn spawn_wall(commands: &mut Commands, wall: Wall) {
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(wall)
		.insert(Collider);
}
