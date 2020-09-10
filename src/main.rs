use bevy::prelude::*;

fn main() {
	App::build().add_default_plugins().add_plugin(HelloPlugin).run();
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
			.add_startup_system(add_people_to_world.system())
			.add_system(greet_people.system());
	}
}

struct Person;
struct Name(String);

fn add_people_to_world(mut commands: Commands) {
	commands
		.spawn((Person, Name("Alice".into())))
		.spawn((Person, Name("Bob".into())))
		.spawn((Person, Name("Charlie".into())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut all_people: Query<(&Person, &Name)>) {
	let delta = time.delta_seconds;
	timer.0.tick(delta);
	if timer.0.finished {
		for (_person, Name(name)) in &mut all_people.iter() {
			println!("Hello {}!", name);
		}
	}
}
