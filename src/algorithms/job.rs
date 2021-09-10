use crate::output::Hashed;

pub struct Job {
    id: u32,
    algorithm: String,
    input: String
    destination: String,
}

pub fn generate_job(algorithm: String, input: String) -> Result<Job, Error> {}

pub fn control_job(job: Job) -> Result<Hashed, Error> {}

fn generate_id() -> u32 {
    1234567890
}

fn filter_algorithm(algorithm: String) -> bool {
    true
}

fn determine_destination() -> String {}