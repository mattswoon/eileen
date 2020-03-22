pub mod agent;
pub mod model; 
pub mod errors;
pub mod change;

use std::{
    collections::HashMap,
    time::Instant
};
use crate::{
    agent::{Person, InfectionStatus},
    model::{Model, step}
};

fn main() {
    let now = Instant::now();
    let num_agents = 10000;
    let mut people = HashMap::new();
    for i in 0..num_agents {
        let person = Person { 
            status: InfectionStatus::Susceptible,
            mobility: 0.1
        };
        people.insert(i, person);
    }
    people.get_mut(&0).unwrap().status = InfectionStatus::Infected(0);

    let mut model = Model {
        people: people,
        infection_probability: 0.1,
        recovery_probability: 0.02
    };

    for _s in 0..10 {
        step(&mut model).unwrap();
    }
    println!("{:?}", model.num_infected());
    println!("Main completed in {}s", now.elapsed().as_secs());
}
