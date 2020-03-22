#[derive(Debug)]
pub enum InfectionStatus {
    Infected(i64),
    Susceptible,
    Recovered
}

#[derive(Debug)]
pub struct Person {
    pub status: InfectionStatus,
    pub mobility: f64
}

impl Person {
    pub fn is_infected(&self) -> bool {
        match self.status {
            InfectionStatus::Infected(_) => true,
            _ => false
        }
    }

    pub fn is_susceptible(&self) -> bool {
        match self.status {
            InfectionStatus::Susceptible => true,
            _ => false
        }
    }

    pub fn is_recovered(&self) -> bool {
        match self.status {
            InfectionStatus::Recovered => true,
            _ => false
        }
    }
}
