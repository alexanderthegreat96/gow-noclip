GOWR NO-CLIP
============

REQUIREMENTS
------------
- Windows (10 or 11)
- God of War Ragnarok on PC
- Single-player only. Do not use online.


HOW TO USE
----------
1. Launch God of War Ragnarok. Wait until you are in-game (not on
   the main menu).
2. Run gow_tools.exe.
3. If SmartScreen or your antivirus complains, click "More info"
   -> "Run anyway". The tool reads/writes the game's memory, which
   is the kind of thing anti-cheat and AV heuristics flag.
4. Alt-tab back to the game. The tool runs in the background and
   listens for hotkeys globally -- you don't need to focus its
   window.


CONTROLS
--------
  G         toggle no-clip on / off
  W         move forward (in the direction the camera is looking)
  S         move backward
  A         strafe left
  D         strafe right
  LSHIFT    ascend
  LCTRL     descend
  L         lock gravity (freeze -- you stop falling)
  U         unlock gravity (normal falling behaviour)
  X         quit the tool

Note: since the tool listens to your keyboard globally, X quits it
even while you are focused on the game. Just something to be aware
of if you have X bound to a game action.


TROUBLESHOOTING
---------------
"process 'GoWR.exe' not found. is the game running?"
    Launch the game first, wait until you are past the main menu,
    then run gow_tools.exe.

Pressing WASD in-game does nothing.
    Press G first. No-clip is off by default -- G toggles it on.

I fell through the map.
    That happens if gravity is unlocked and you clip into the world.
    Press L to re-lock gravity, or LSHIFT to bump yourself back up.

Terrain / textures aren't loading in new areas.
    The game streams assets as you move. If you fly faster than it
    can load, you'll outrun the streamer. Slow down (stop moving for
    a couple seconds) and things will pop in.

Weird animation glitches on Kratos.
    Toggle no-clip off (G) and let the game settle for a second.
    You may also want to reload a save.

--