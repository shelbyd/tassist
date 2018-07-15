#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "play")]
    Play(Play),
}

#[derive(StructOpt, Debug)]
pub struct Play {
    /// Which .tas file to play
    pub tas_file: String,
    
    /// How to play the TAS
    #[structopt(subcommand)]
    pub strategy: PlayStrategy,
}

#[derive(StructOpt, Debug)]
pub enum PlayStrategy {
    /// Run a generated lua script
    #[structopt(name = "watched_file")]
    WatchedFile(WatchedFile),
}

#[derive(StructOpt, Debug)]
pub struct WatchedFile {
    /// The directory to place the file to watch
    pub directory: String,
}
