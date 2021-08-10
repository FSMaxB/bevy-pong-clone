use crate::Collider;
use bevy::app::EventReader;
use bevy::core::Time;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::math::{Vec2, Vec3};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use bevy::sprite::entity::SpriteBundle;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use bevy::window::WindowResized;

pub struct Ball {
	speed: f32,
	direction: Vec2,
}

impl Ball {
	pub fn velocity(&self) -> Vec2 {
		self.speed * self.direction.normalize()
	}
}

impl Default for Ball {
	fn default() -> Self {
		Self {
			speed: Default::default(),
			direction: Vec2::new(1.0, 1.0).normalize(),
		}
	}
}

pub fn spawn_ball(commands: &mut Commands) {
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(Ball::default());
}

pub fn ball_resize_system(
	mut resize_reader: EventReader<WindowResized>,
	mut query: Query<(&mut Sprite, &mut Transform, &mut Ball)>,
) {
	let resize_event = match resize_reader.iter().last() {
		Some(event) => event,
		None => return,
	};

	for (mut sprite, mut transform, mut ball) in query.iter_mut() {
		let window_height = resize_event.height as f32;
		ball.speed = window_height / 1.5;

		let ball_width = 0.05 * window_height;
		sprite.size = Vec2::new(ball_width, ball_width);

		transform.translation = Vec3::default();
	}
}

pub fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
	let time_delta = time.delta_seconds();
	for (ball, mut transform) in query.iter_mut() {
		transform.translation += time_delta * ball.velocity().extend(0.0);
	}
}

pub fn ball_collision_system(
	mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
	collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
	for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
		for (_collider, collider_transform, collider_sprite) in collider_query.iter() {
			let collision = collide(
				ball_transform.translation,
				ball_sprite.size,
				collider_transform.translation,
				collider_sprite.size,
			);

			let collision = match collision {
				Some(collision) => collision,
				None => continue,
			};

			use Collision::*;
			let (reflect_x, reflect_y) = match collision {
				Left => (ball.direction.x > 0.0, false),
				Right => (ball.direction.x < 0.0, false),
				Top => (false, ball.direction.y < 0.0),
				Bottom => (false, ball.direction.y > 0.0),
			};

			if reflect_x {
				ball.direction.x = -ball.direction.x;
			}

			if reflect_y {
				ball.direction.y = -ball.direction.y;
			}
		}
	}
}
