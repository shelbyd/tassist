function advance_frames(frames)
  for i=1,frames do
    emu.frameadvance()
  end
end

function press(keys)
  local table = {{}};
  for i, key in ipairs(keys) do
    table[key] = true;
  end
  joypad.set(table, 1);
  emu.frameadvance();
end

console.clear()

savestate.load("{initial_save_state_file}")

client.reboot_core()
client.unpause()

{gameplay}

while true do
  emu.frameadvance()
end