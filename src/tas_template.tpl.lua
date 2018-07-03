savestate.load("{initial_save_state_file}")

client.reboot_core()
client.unpause()
console.clear()

while true do
  emu.frameadvance()
end