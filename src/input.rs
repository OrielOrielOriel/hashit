/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Payload
    - Text or file?

and so on.
*/
use clap::{Clap, App, Arg, ArgMatches, ArgSettings, clap_app};
use std::path::Path;
use crate::algorithms::*;



// fn test_func() -> App<'static> {
//     let newapp: App = clap_app!( hashit =>
//         (version: "1.0")
//         (author: "beansman")
//         (@arg verbose: -v --verbose +multiple_occurrences)
//     );

//     newapp
// }

// pub fn update_app(app: App) -> App {
//     let newnewapp: App = app.arg(Arg::new("test"));
//     newnewapp
// }

// pub fn get_test_options() -> ArgMatches { 
//     let newapp = test_func();
//     let newnewapp = update_app(newapp);

//     newnewapp.get_matches()
// }
