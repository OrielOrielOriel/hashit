/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Payload
    - Text or file?

and so on.
*/
use clap::{App, Arg, ArgMatches};
use crate::algorithms::*;

fn iniate_clap_app() -> App<'static> {
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

pub fn test_get_options() -> ArgMatches {
    iniate_clap_app().get_matches()
}
