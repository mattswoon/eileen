use std::{
    collections::{
        HashMap
    },
    time::{Instant}
};
use shuffle::{
    shuffler::Shuffler,
    fy::FisherYates
};
use rayon::prelude::*;
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
    pub infection_probability: f64,
    pub recovery_probability: f64,
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
}


pub fn queue_changes(model: &Model) -> Result<Vec<Change>, Error> {
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let mut fy = FisherYates::default();
    let people: Vec<usize> = model.people.keys().map(|x| x.clone()).collect();
    let mut contact_people = people.clone();
    fy.shuffle(&mut contact_people, &mut rng).map_err(|_| Error::ShuffleError)?;

    let results: Vec<Result<Vec<Change>, Error>> = people.par_iter()
        .map(|pid| {
            let mut changes = Vec::new();
            let person = model.people.get(pid).ok_or(Error::CouldntFindPerson)?;
            match person.status {
                InfectionStatus::Infected(_) => {
                    if diceroll() < model.recovery_probability {
                        changes.push(Change::BecomeRecovered(pid.clone()));
                    } else {
                        changes.push(Change::RemainInfected(pid.clone()));
                    }
                    ()
                },
                _ => ()
            }
            for contact_pid in contact_people.iter() {
                let contact_person = model.people.get(contact_pid).ok_or(Error::CouldntFindPerson)?;
                let cond = diceroll() < person.mobility * contact_person.mobility * model.infection_probability;
                match (&person.status, &contact_person.status) {
                    (InfectionStatus::Infected(_), InfectionStatus::Susceptible) => {
                        if cond {
                            changes.push(Change::BecomeInfected(contact_pid.clone()));
                        }
                        ()
                    },
                    (InfectionStatus::Susceptible, InfectionStatus::Infected(_)) => {
                        if cond {
                            changes.push(Change::BecomeInfected(pid.clone()));
                        }
                        ()
                    },
                    _ => ()
                }
            }
            Ok(changes)
        })
        .collect();
    let mut changes = Vec::new();
    for r in results.into_iter() {
        let mut v = r?;
        changes.append(&mut v);
    }
    println!("Queue changes took: {}s", now.elapsed().as_secs());
    Ok(changes)
}


pub fn execute_changes(changes: &Vec<Change>, model: &mut Model) -> Result<(), Error> {
    for change in changes.iter() {
        match change {
            Change::RemainInfected(pid) => {
                let person = model.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
                match person.status {
                    InfectionStatus::Infected(days) => person.status = InfectionStatus::Infected(days + 1),
                    _ => return Err(Error::BadChange("Change is RemainInfected but person isn't Infected"))
                }
            },
            Change::BecomeInfected(pid) => {
                let person = model.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
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
                let person = model.people.get_mut(pid).ok_or(Error::CouldntFindPerson)?;
                match person.status {
                    InfectionStatus::Infected(_) => person.status = InfectionStatus::Recovered,
                    _ => return Err(Error::BadChange("Change is BecomeRecovered but person isn't Infected"))
                }
            }
        }
    }
    Ok(())
}

pub fn step(model: &mut Model) -> Result<(), Error> {
    let changes = queue_changes(model)?;
    execute_changes(&changes, model)?;
    Ok(())
}
