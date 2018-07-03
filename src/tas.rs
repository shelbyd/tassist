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
        format!(r#"
            savestate.load("{}")
            console.clear()
            client.reboot_core()
            client.unpause()
            
            while true do
                emu.frameadvance()
            end
        "#, start_state_path)
    }
}