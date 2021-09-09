pub struct Item {
    pub payload: Tuple,
    pub algorithm: HashingAlgorithm
}

pub struct HashingAlgorithm {
    pub name: String,
    pub compute: Function,
}