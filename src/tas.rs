use failure::Error;
use std::io::Read;

pub struct Tas {
}

impl Tas {
    pub fn as_lua(&self) -> String {
        format!(
            include_str!("tas_template.tpl.lua"))
    }

    pub fn parse<R: Read>(_: R) -> Result<Tas, Error> {
        Ok(Tas {})
    }
}