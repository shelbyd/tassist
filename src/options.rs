#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "play")]
    Play(Play),
    #[structopt(name = "create_from_state")]
    CreateFromState(CreateFromState),
}

#[derive(StructOpt, Debug)]
pub struct Play {
    pub tas_file: String,
    
    #[structopt(subcommand)]
    pub strategy: PlayStrategy,
}

#[derive(StructOpt, Debug)]
pub enum PlayStrategy {
    #[structopt(name = "watched_file")]
    WatchedFile(WatchedFile),
}

#[derive(StructOpt, Debug)]
pub struct WatchedFile {
    pub file: String,
}

#[derive(StructOpt, Debug)]
pub struct CreateFromState {
    pub state_file: String,
    
    pub tas_file: String,
}