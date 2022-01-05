use bevy::app::{AppBuilder, Plugin};
use bevy::ecs::system::IntoSystem;
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_startup_system(hello_world.system())

            .add_system(greet_people.system());
    }
}


fn hello_world()
{
    println!("Hello world!");
}

fn add_people(mut commands: Commands)
{
    commands.spawn().insert(Person).insert(Name("John Wayne".to_string()));
    commands.spawn().insert(Person).insert(Name("Clint Eastwood".to_string()));
    commands.spawn().insert(Person).insert(Name("Tommy Lee Jones".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
{
    if timer.0.tick(time.delta()).just_finished()
    {
        for name in query.iter()
        {
            println!("Greetings {}!", name.0)
        }
    }
}

struct Person;

struct Name(String);

struct GreetTimer(Timer);