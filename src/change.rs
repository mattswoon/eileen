use rand::{
    distributions::{
        Uniform,
        Distribution
    }
};

pub enum Change {
    BecomeInfected(usize),
    RemainInfected(usize),
    BecomeRecovered(usize),
}


pub fn diceroll() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    let sample = between.sample(&mut rng);
    sample
}
