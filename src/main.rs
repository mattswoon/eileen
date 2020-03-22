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
    model::{
        Model, 
//        step
    }
};

fn main() {
    let now = Instant::now();
    let num_agents = 10_000_000;
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
        transmission_probability: 0.1,
        recovery_probability: 0.02,
        num_contacts_per_step: 100
    };

    model.run(10).unwrap();

    println!("Main completed in {}s", now.elapsed().as_secs());
}
