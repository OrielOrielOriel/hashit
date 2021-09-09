mod cli_input_parser;

fn main() {
    let opts: cli_input_parser::Opts = cli_input_parser::get_opts();
    println!("beans")
}