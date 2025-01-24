use bevy::prelude::*;

// Component examples
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// A simple system
fn hello_world()
{
    println!("hello world!");
}

// Component initialization example
fn add_people(mut commands: Commands)
{
    commands.spawn((Person,Name("Elaina Proctor".to_string())));
    commands.spawn((Person,Name("Renzh Bow".to_string())));
    commands.spawn((Person,Name("Zang Renzh".to_string())));
}

// Query example
fn greet_people(query: Query<&Name,With<Person>>)
{
    for name in &query
    {
        println!("hello {}!", name.0);
    }
}

// Mutable query example
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}


fn main()
{
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

