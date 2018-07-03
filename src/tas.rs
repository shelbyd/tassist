#[derive(Deserialize)]
pub struct Tas {
}

impl Tas {
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