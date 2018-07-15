use extensions::str::WindowsPaths;
use failure::Error;
use options::{PlayStrategy as PlayStrategyOpt};
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tas::Tas;

pub fn from_option(option: PlayStrategyOpt) -> Box<PlayStrategy> {
    match option {
        PlayStrategyOpt::WatchedFile(watched_file) => {
            Box::new(WatchedFile::new(watched_file.directory.into()))
        }
    }
}

pub trait PlayStrategy {
    fn play(&self, tas: &Tas) -> Result<(), Error>;
}

struct WatchedFile {
    path: OsString,
}

impl WatchedFile {
    fn new(path: OsString) -> WatchedFile {
        WatchedFile {
            path,
        }
    }
    
    fn lua_file_path(&self) -> PathBuf {
        Path::new(&self.path).join("tassist.watched.lua")
    }
}

impl PlayStrategy for WatchedFile {
    fn play(&self, tas: &Tas) -> Result<(), Error> {
        let lua = tas.as_lua();

        let mut lua_tempfile = File::create(self.lua_file_path())?;
        write!(lua_tempfile, "{}", lua)?;
        println!("Point the Lua Console at ({})", self.lua_file_path().canonicalize()?.to_string_lossy().strip_windows_unc());
        
        Ok(())
    }
}