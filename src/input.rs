/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Debug bool
- Payload
    - Text or file?

and so on.
*/
use clap::{App, Arg, ArgMatches};
use std::path::Path;
use crate::algorithms::*;

pub fn iniate_clap_app() -> App<'static> {
    let ret = 
        App::new("hashit")
        .author("Oriel <@OrielOrielOriel>")
        .version("1.0")
        .about("Hashing tool")
        .subcommand( md5::md5_clap_app() )
        .arg(
            Arg::new("debug")
            .short('d')
            .long("debug")
            .about("Turns on debugging output."))
        .arg(
            Arg::new("quiet")
            .short('q')
            .long("quiet")
            .about("Turns off all output except for resulting hash."))
    ; 
    ret
}

pub fn generate_task(clap_app: App<'static>) -> TerminalTask {
    let mut ret = TerminalTask::new();
    let matches = clap_app.get_matches();

    ret.update_debug(matches.is_present("debug"));
    ret.update_quiet(matches.is_present("quiet"));
    ret.update_algorithm(matches.subcommand_name().expect("None"));

    ret
}

pub fn test_get_options() -> ArgMatches {
    iniate_clap_app().get_matches()
}

