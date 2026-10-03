# client-mac/ - Nexon's Mac app, for reference

This folder is empty in the repository on purpose. It holds a copy of what a Mac player of
Classic World installs: Nexon's **`MapleStory Launcher.app`** (about 400 MB). Only this README
is tracked.

One thing is taken from it: `tools/make_mac_client.py` copies the game's own icon (the Orange
Mushroom of the inner `MapleStory Classic World.app`) into `MapleCW.app` at package time, so
Nexon's art reaches the zip without ever being committed. Without this folder the app builds
with macOS's generic icon. Otherwise it is the evidence behind
[`docs/mac-client.md`](../docs/mac-client.md): Nexon's Mac client is a CrossOver (Wine) bottle
running the **Windows** `MapleStory.exe`, with the Mac anti-cheat (NGS-X, `ngsx.framework`) as
a native helper beside it. That is why the MapleCW Mac client is our Windows launcher, stub and
client in Wine rather than a port - and why it borrows the player's own installed copy of this
app for its Wine runtime instead of shipping one.

To inspect it without a Mac: the `.app` here is a zip (stored, not compressed), so
`python -m zipfile -l "client-mac/MapleStory Launcher.app"` lists it.
