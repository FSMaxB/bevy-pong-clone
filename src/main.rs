use bevy::prelude::*;

fn main() {
	App::build()
		.add_default_plugins()
		.add_startup_system(setup.system())
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dComponents::default());
	spawn_ball(&mut commands);
}

fn spawn_ball(commands: &mut Commands) {
	const SIZE: f32 = 50.0;

	commands.spawn(SpriteComponents {
		sprite: Sprite {
			size: Vec2::new(SIZE, SIZE),
		},
		translation: Translation::new(300.0, 200.0, 0.0),
		rotation: Rotation::from_rotation_z(std::f32::consts::PI / 4.0),
		..Default::default()
	});
}
