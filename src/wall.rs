use crate::{Collider, Reset};
use bevy::app::EventReader;
use bevy::ecs::prelude::{Query, Res};
use bevy::ecs::system::Commands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Sprite, Transform, Windows};
use bevy::sprite::entity::SpriteBundle;

pub enum Wall {
	Top,
	Bottom,
}

impl Wall {
	pub const THICKNESS: f32 = 20.0;
}

pub fn wall_reset_system(
	mut reset_reader: EventReader<Reset>,
	windows: Res<Windows>,
	mut query: Query<(&mut Sprite, &mut Transform, &Wall)>,
) {
	if reset_reader.iter().last().is_none() {
		return;
	}

	let window = windows.get_primary().unwrap();

	for (mut sprite, mut transform, wall) in query.iter_mut() {
		sprite.size = Vec2::new(window.width(), Wall::THICKNESS);

		use Wall::*;
		let y_offset = (window.height() - Wall::THICKNESS) / 2.0;
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
