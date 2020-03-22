use std::{
    collections::{
        HashMap
    },
    time::{Instant}
};
//use shuffle::{
//    shuffler::Shuffler,
//    fy::FisherYates
//};
use rayon::prelude::*;
use conv::prelude::*;
use crate::{
    agent::{
        InfectionStatus,
        Person
    },
    errors::Error,
    change::{
        Change,
        diceroll
    }
};


#[derive(Debug)]
pub struct Model {
    pub people: HashMap<usize, Person>,
    pub transmission_probability: f64,
    pub recovery_probability: f64,
    pub num_contacts_per_step: usize
}

impl Model {
    pub fn num_infected(&self) -> usize {
        self.people.values()
            .filter(|p| p.is_infected())
            .count()
    }

    pub fn num_people(&self) -> usize {
        self.people.iter().count()
    }

    pub fn num_susceptible(&self) -> usize {
        self.people.values()
            .filter(|p| p.is_susceptible())
            .count()
    }

    pub fn num_recovered(&self) -> usize {
        self.people.values()
            .filter(|p| p.is_recovered())
            .count()
    }

    pub fn proportion_infected(&self) -> f64 {
        let f_num_infected = self.num_infected().value_as::<f64>().unwrap();
        let f_num_total = self.num_people().value_as::<f64>().unwrap();
        f_num_infected / f_num_total
    }

    /// The probability of getting infected this step
    pub fn infection_probability(&self) -> f64 {
        let p = self.proportion_infected();
        let n = self.num_contacts_per_step.value_as::<i32>().unwrap();
        (1.0 - (1.0 - p).powi(n)) * self.transmission_probability
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let changes = self.queue_changes();
        self.execute_changes(&changes)?;
        Ok(())
    }

    pub fn run(&mut self, num_steps: usize) -> Result<(), Error> {
        for s in 0..num_steps {
            println!("Running step {}", s);
            let now = Instant::now();
            self.step()?;
            println!(" |- Completed in {}s", now.elapsed().as_secs());
            println!("\tNum Susceptible: {}", self.num_susceptible());
            println!("\tNum Infected   : {}", self.num_infected());
            println!("\tNum Recovered  : {}", self.num_recovered());
        }
        Ok(())
    }

    pub fn queue_changes(&self) -> Vec<Change> {
        let infection_probability = self.infection_probability();
        let changes = self.people
            .par_iter()
            .map(|(pid, person)| {
                let mut changes = Vec::new();
                match person.status {
                    InfectionStatus::Infected(_) => {
                        if diceroll() < self.recovery_probability {
                            changes.push(Change::BecomeRecovered(pid.clone()));
                        } else {
                            changes.push(Change::RemainInfected(pid.clone()));
                        }
                    },
                    InfectionStatus::Susceptible => {
                        if diceroll() < infection_probability {
                            changes.push(Change::BecomeInfected(pid.clone()));
                        }
                    },
                    _ => () 
                }
                changes
            })
            .flatten()
            .collect();
        changes
    }

    pub fn execute_changes(&mut self, changes: &Vec<Change>) -> Result<(), Error> {
        for change in changes.iter() {
            match change {
                Change::RemainInfected(pid) => {
                    let person = self.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
                    match person.status {
                        InfectionStatus::Infected(days) => person.status = InfectionStatus::Infected(days + 1),
                        _ => return Err(Error::BadChange("Change is RemainInfected but person isn't Infected"))
                    }
                },
                Change::BecomeInfected(pid) => {
                    let person = self.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
                    match person.status {
                        InfectionStatus::Susceptible => person.status = InfectionStatus::Infected(0),
                        InfectionStatus::Infected(days) => {
                            if days > 0 {
                                return Err(Error::BadChange("Change is BecomeInfected and person has already been infected for > 0 days"));
                            }
                        },
                        InfectionStatus::Recovered => return Err(Error::BadChange("Change is BecomeInfected but person has already Recovered"))
                    }
                },
                Change::BecomeRecovered(pid) => {
                    let person = self.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
                    match person.status {
                        InfectionStatus::Infected(_) => person.status = InfectionStatus::Recovered,
                        _ => return Err(Error::BadChange("Change is BecomeRecovered but person isn't Infected"))
                    }
                }
            }
        }
        Ok(())
    }
}
