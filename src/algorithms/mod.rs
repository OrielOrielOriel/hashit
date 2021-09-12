use clap::Clap;
pub mod md5;

trait Task {
    fn new() -> Self;
    fn update_payload(&mut self, payload: Payload) -> ();
    fn update_hash(&mut self, hash: String) -> ();
    fn update_is_file(&mut self, is_file: bool) -> ();
}

pub enum Payload {
    InputDataString(String),
    InputDataFile(String),
    InputDataVec(Vec<String>)
}

pub struct TerminalTask {
    pub id: u32,
    pub payload: Option<Payload>,
    pub hash: Option<String>,
    pub is_file: Option<bool>,
}

fn generate_uuid() -> u32 { 123456789 }

impl Task for TerminalTask {
    fn new() -> Self {
        TerminalTask { id: generate_uuid(), payload: None, hash: None, is_file: None }
    }

    fn update_payload(&mut self, payload: Payload) -> () {
        self.payload = Some(payload);
    }

    fn update_hash(&mut self, hash: String) -> () {
        self.hash = Some(hash);
    }

    fn update_is_file(&mut self, is_file: bool) -> () {
        self.is_file = Some(is_file);
    }
}


