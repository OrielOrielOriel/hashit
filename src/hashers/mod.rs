mod md5;

pub mod subcommand_structures {
    use clap::Clap;

    #[derive(Clap, Debug)]
    pub enum algorithm {
        md5(crate::hashers::md5::Md5Opts),
    }
}

