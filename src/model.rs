use std::{
    collections::{
        HashMap
    }
};
use rand::{
    distributions::{
        Uniform,
        Distribution
    }
};
use shuffle::{
    shuffler::Shuffler,
    fy::FisherYates
};
use crate::{
    agent::{
        InfectionStatus,
        Person
    },
    errors::Error
};

fn diceroll() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    let sample = between.sample(&mut rng);
    sample
}

pub struct Model {
    people: HashMap<usize, Person>,
    infection_probability: f64,
    recovery_probability: f64
}

pub enum Change {
    BecomeInfected(usize),
    RemainInfected(usize),
    BecomeRecovered(usize),
}


pub fn queue_changes(model: &Model) -> Result<Vec<Change>, Error> {
    let mut changes = Vec::new();
    let mut rng = rand::thread_rng();
    let mut fy = FisherYates::default();
    let people: Vec<usize> = model.people.keys().map(|x| x.clone()).collect();
    let mut contact_people = people.clone();
    fy.shuffle(&mut contact_people, &mut rng).map_err(|_| Error::Error)?;

    for pid in people.iter() {
        let person = model.people.get(pid).ok_or(Error::Error)?;
        match person.status {
            InfectionStatus::Infected(_) => {
                if diceroll() < model.recovery_probability {
                    changes.push(Change::BecomeRecovered(pid.clone()));
                } else {
                    changes.push(Change::RemainInfected(pid.clone()));
                }
            },
            _ => ()
        }
        for contact_pid in contact_people.iter() {
            let contact_person = model.people.get(contact_pid).ok_or(Error::Error)?;
            let cond = diceroll() < person.mobility * contact_person.mobility * model.infection_probability;
            match (&person.status, &contact_person.status) {
                (InfectionStatus::Infected(_), InfectionStatus::Susceptible) => {
                    if cond {
                        changes.push(Change::BecomeInfected(contact_pid.clone()));
                    }
                },
                (InfectionStatus::Susceptible, InfectionStatus::Infected(_)) => {
                    if cond {
                        changes.push(Change::BecomeInfected(pid.clone()));
                    }
                },
                _ => ()
            }
        }
    }
    Ok(changes)
}


pub fn execute_changes(changes: &Vec<Change>, model: &mut Model) -> Result<(), Error> {
    for change in changes.iter() {
        match change {
            Change::RemainInfected(pid) => {
                let person = model.people.get_mut(pid).ok_or(Error::Error)?;
                match person.status {
                    InfectionStatus::Infected(days) => person.status = InfectionStatus::Infected(days + 1),
                    _ => return Err(Error::Error)
                }
            },
            Change::BecomeInfected(pid) => {
                let person = model.people.get_mut(pid).ok_or(Error::Error)?;
                match person.status {
                    InfectionStatus::Susceptible => person.status = InfectionStatus::Infected(0),
                    _ => return Err(Error::Error)
                }
            },
            Change::BecomeRecovered(pid) => {
                let person = model.people.get_mut(pid).ok_or(Error::Error)?;
                match person.status {
                    InfectionStatus::Infected(_) => person.status = InfectionStatus::Recovered,
                    _ => return Err(Error::Error)
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
