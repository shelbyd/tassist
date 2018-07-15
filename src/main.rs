#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate structopt;

use failure::Error;
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;

mod extensions;

mod options;
use options::Options;

mod play_strategy;

mod tas;
use tas::Tas;

fn main() {
    run().unwrap_or_else(|e| panic!("{}", e));
    std::process::exit(0);
}

fn run() -> Result<(), Error> {
    let options = Options::from_args();
    
    match options.command {
        options::Command::Play(play) => {
            let mut contents = String::new();
            File::open(&play.tas_file)?.read_to_string(&mut contents)?;
            let tas: Tas = Tas::parse(&contents)?;
            let play_strategy = play_strategy::from_option(play.strategy);
            play_strategy.play(&tas)?;
        }
    }
    
    Ok(())
}
