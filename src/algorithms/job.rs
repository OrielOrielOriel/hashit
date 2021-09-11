use crate::output::Hashed;
use std::path::Path;

enum Payload {
    Payload(String),
    Payload(Path),
}

pub struct Job {
    id: u32,
    algorithm: String,
    payload: Payload
    destination: String,
}

impl Job {
    pub fn generate_job(algorithm: String, data: String) -> Result<Job, Error> {
        
    }
    fn generate_id() -> u32 {
        1234567890
    }
    
    fn filter_algorithm(algorithm: String) -> bool {}
    fn determine_destination() -> String {}
    fn determine_payload_type(data: String) -> Option<Payload::Payload(String), Payload::Payload(Path)> {
        let path: Path = Path::new(&data);

        if path.is_file() {
            Payload(data)
        } else {
            Payload(path)
        }
    }


}