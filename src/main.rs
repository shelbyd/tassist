extern crate bincode;
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

use bincode::{deserialize_from, serialize_into};
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
    run().unwrap();
    std::process::exit(0);
}

fn run() -> Result<(), Error> {
    let options = Options::from_args();
    
    match options.command {
        options::Command::Play(play) => {
            let tas: Tas = deserialize_from(File::open(&play.tas_file)?)?;
            let play_strategy = play_strategy::from_option(play.strategy);
            play_strategy.play(&tas)?;
        }
        options::Command::CreateFromState(create_from_state) => {
            let mut state_contents = Vec::new();
            File::open(create_from_state.state_file)?.read_to_end(&mut state_contents)?;
            let tas = Tas::from_state(state_contents);
            serialize_into(File::create(create_from_state.tas_file)?, &tas)?;
        }
    }
    
    Ok(())
}
