use bevy::prelude::*;

struct Person;
struct Name(String);

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

pub fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
