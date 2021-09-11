use clap::Clap;
pub mod md5;

trait Task {
    fn new(id: u32) -> Self;
    fn update_payload(&mut self, payload: Payload) -> ();
    fn update_hash(&mut self, hash: String) -> ();
    fn update_is_file(&mut self, is_file: bool) -> ();
}

pub enum Payload {
    InputDataString(String),
    InputDataFile(String),
    InputDataVec(vec![String, String])
}

pub struct TerminalTask {
    id: u32,
    payload: Option<Payload>,
    hash: Option<String>,
    is_file: Option<bool>,
}

impl Task for TerminalTask {
    fn new(id: u32) -> Self {
        TerminalTask { id: id, payload: None, hash: None, is_file: None }
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

#[derive(Clap, Debug)]
pub enum AlgorithmsList {
    md5(crate::algorithms::md5::Md5Opts),
}


