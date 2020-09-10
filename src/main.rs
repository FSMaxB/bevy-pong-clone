use bevy::prelude::*;

fn main() {
	App::build()
		.add_default_plugins()
		.add_startup_system(setup.system())
		.run();
}

fn setup(mut commands: Commands) {
	spawn_ball(&mut commands);
}

fn spawn_ball(commands: &mut Commands) {
	const SIZE: f32 = 50.0;

	commands.spawn(Camera2dComponents::default()).spawn(SpriteComponents {
		sprite: Sprite {
			size: Vec2::new(SIZE, SIZE),
		},
		..Default::default()
	});
}
