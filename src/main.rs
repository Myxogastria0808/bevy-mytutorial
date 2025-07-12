use bevy::prelude::*;

// ECS (Entity Component System)

fn main() {
    App::new()
        // Runs once at the start of the application
        .add_systems(Startup, add_people)
        // Runs every frame
        .add_systems(Update, hello_world)
        .run();
}

// System
fn hello_world() {
    println!("Hello, world!");
}

// Component
// Component should be small and simple
/*
- Not Good
#[derive(Component)]
Person {
    name: String,
    age: u32
};

- Good
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u32);
*/
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Startup System
// This system runs once at the start of the application
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Tom".to_string())));
}
