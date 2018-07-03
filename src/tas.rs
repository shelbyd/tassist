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

    pub fn as_lua(&self) -> String {
        String::from(r#"
            console.clear()
            client.reboot_core()
            client.unpause()
            
            while true do
                emu.frameadvance()
            end
        "#)
    }
}