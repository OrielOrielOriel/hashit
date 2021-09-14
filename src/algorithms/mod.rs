pub mod md5;

pub trait Task {
    fn new() -> Self;
    fn update_algorithm(&mut self, algorithm: String) -> ();
    fn update_payload(&mut self, payload: Payload) -> ();
    fn update_hash(&mut self, hash: String) -> ();
    fn update_is_file(&mut self, is_file: bool) -> ();
    fn update_debug(&mut self, debug: bool) -> ();
    fn update_quiet(&mut self, quiet: bool) -> ();
    fn update_verbosity(&mut self, verbosity: u32) -> ();
}

pub enum Payload {
    InputDataString(String),
    InputDataFile(String),
    InputDataVec(Vec<String>)
}

pub struct TerminalTask {
    pub id: u32,
    pub payload: Option<Payload>,
    pub algorithm: Option<&str>,
    pub hash: Option<String>,
    pub is_file: Option<bool>,
    pub debug: Option<bool>,
    pub quiet: Option<bool>,
    pub verbosity: Option<u32>,
}

fn generate_uuid() -> u32 { 123456789 }

impl Task for TerminalTask {
    fn new() -> TerminalTask {
        TerminalTask { 
            id: generate_uuid(),
            algorithm: None,
            payload: None,
            hash: None, 
            is_file: None, 
            debug: None,
            quiet: None,
            verbosity: None 
    }}

    fn update_payload(&mut self, payload: Payload) -> () {
        self.payload = Some(payload);
    }

    fn update_hash(&mut self, hash: String) -> () {
        self.hash = Some(hash);
    }

    fn update_algorithm(&mut self, algorithm: String) -> () {
        self.algorithm = Some(algorithm);
    }

    fn update_is_file(&mut self, is_file: bool) -> () {
        self.is_file = Some(is_file);
    }

    fn update_debug(&mut self, debug: bool) -> () {
        self.debug = Some(debug);
    }

    fn update_quiet(&mut self, quiet: bool) -> () {
        self.quiet = Some(quiet);
    }

    fn update_verbosity(&mut self, verbosity: u32) -> () {
        self.verbosity = Some(verbosity);
    }
}


