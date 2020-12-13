use crate::Collider;
use bevy::core::Time;
use bevy::ecs::{Commands, Query, Res};
use bevy::math::{Vec2, Vec3};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use bevy::sprite::entity::SpriteComponents;
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

	pub fn update_after_window_resize(
		&mut self,
		resize_event: &WindowResized,
		size: &mut Vec2,
		translation: &mut Vec3,
	) {
		let window_height = resize_event.height as f32;
		self.speed = window_height / 1.5;

		let ball_width = 0.05 * window_height;
		*size = Vec2::new(ball_width, ball_width);

		*translation = Vec3::default();
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
	commands.spawn(SpriteComponents::default()).with(Ball::default());
}

pub fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
	let time_delta = time.delta_seconds;
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
				Left => (ball.direction.x() > 0.0, false),
				Right => (ball.direction.x() < 0.0, false),
				Top => (false, ball.direction.y() < 0.0),
				Bottom => (false, ball.direction.y() > 0.0),
			};

			if reflect_x {
				*ball.direction.x_mut() = -ball.direction.x();
			}

			if reflect_y {
				*ball.direction.y_mut() = -ball.direction.y();
			}
		}
	}
}
