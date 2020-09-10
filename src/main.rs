use bevy::prelude::*;

fn main() {
	App::build()
		.add_startup_system(add_people_to_world.system())
		.add_system(hello_world.system())
		.add_system(greet_people.system())
		.run();
}

struct Person;
struct Name(String);

fn add_people_to_world(mut commands: Commands) {
	commands
		.spawn((Person, Name("Alice".into())))
		.spawn((Person, Name("Bob".into())))
		.spawn((Person, Name("Charlie".into())));
}

fn greet_people(_people: &Person, Name(name): &Name) {
	println!("Hello {}!", name);
}

fn hello_world() {
	println!("hello world!");
}
