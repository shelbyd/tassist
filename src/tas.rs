#[derive(Deserialize, Serialize)]
pub struct Tas {
    start_state: Vec<u8>,
}

impl Tas {
    pub fn from_state(start_state: Vec<u8>) -> Tas {
        Tas {
            start_state,
        }
    }
    
    pub fn start_state(&self) -> &[u8] {
        &self.start_state
    }

    pub fn as_lua(&self, start_state_path: &str) -> String {
        format!(
            include_str!("tas_template.tpl.lua"),
            initial_save_state_file=start_state_path)
    }
}