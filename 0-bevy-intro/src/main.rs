use bevy::prelude::*;

fn main() {
    //* Bevy works with a ECS (Entity Component System) architecture
    //* Bevy systems are executed in parallel
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_without_jobs)
            .add_system(people_jobs);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "John".to_string(),
        },
        Employed {
            job: Job::Programmer,
        },
    ));
    commands.spawn((
        Person {
            name: "Jane".to_string(),
        },
        Employed { job: Job::Designer },
    ));
    commands.spawn((Person {
        name: "Bob".to_string(),
    },));
}

//* This system query all entities with the Person component
pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Hello {}", person.name);
    }
}

//* With case is used to filter entities with a specific component
pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} is employed", person.name);
    }
}
pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is unemployed", person.name);
    }
}

//* Query both components
pub fn people_jobs(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Programmer => "Programmer",
            Job::Designer => "Designer",
            Job::Artist => "Artist",
        };
        println!("{} is a {}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Programmer,
    Designer,
    Artist,
}
