mod input;
use input::{get_opts};

mod algorithms;
use algorithms::{md5, Algorithm};

fn main() {
    get_opts();
}