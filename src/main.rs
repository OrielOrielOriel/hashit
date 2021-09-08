mod cli_input_parser;

fn main() {
    let (opts, is_payload_file) = cli_input_parser::get_opts();
}