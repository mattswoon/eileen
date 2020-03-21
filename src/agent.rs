

pub enum InfectionStatus {
    Infected(i64),
    Susceptible,
    Recovered
}

pub struct Person {
    pub status: InfectionStatus,
    pub mobility: f64
}
