MapleCW for Mac
===============

MapleCW.app runs the same launcher, GameGuard stub and game client as the Windows download,
in Wine - the way Nexon's own Mac version of Classic World runs - with a Wine of its own inside
it. Sign in, press Start Game, play. Everything the Windows launcher does (registering, password recovery, remembering your
server, patching the client, updating itself) works the same way.

NOTHING AUTHENTICATES THE GAME CONNECTION. The launcher signs you in over an encrypted,
pinned connection, but the game protocol itself carries no credentials. Play on servers and
networks you trust.


WHAT YOU NEED
-------------

* A Mac with macOS 10.15 or later. Apple Silicon and Intel both work the same way; on Apple
  Silicon, Rosetta 2 is needed (macOS offers to install it the first time).
* Nothing else. MapleCW.app carries its own Wine (open source, see
  MapleCW.app/Contents/Resources/wine/README-WINE.txt). You do not need Nexon's launcher or
  any other copy of MapleStory.


INSTALLING
----------

1. Unzip MapleCW-setup-mac.zip and drag MapleCW.app into Applications.

2. Open Terminal (Applications -> Utilities -> Terminal), paste this ONE line and press Return:

       xattr -dr com.apple.quarantine /Applications/MapleCW.app && codesign --force --deep --sign - /Applications/MapleCW.app

   Both tools come with macOS - nothing to install. MapleCW.app is not signed by Apple: the
   first half lets macOS open it at all, the second gives it a local signature, without which
   macOS silently refuses it your local network (sign-in fails with "OS Error 10065"). It takes
   a few seconds and prints a line per part of the app; that is normal. Do it again only if you
   replace MapleCW.app with a newer download.

3. Open MapleCW.app. If macOS asks whether it may find devices on your local network, allow
   it. The FIRST start copies the game (about 500 MB) and sets up MapleCW's
   Wine prefix, which takes a minute or two with nothing on screen. Later starts take a few
   seconds.

4. The launcher window opens. Enter the server address you were given, sign in (or use the
   Register tab with a code from a GM), and press Start Game.


WHERE THINGS ARE
----------------

    ~/Library/Application Support/MapleCW/
        MapleCW/              the launcher, the stub and the client (client/)
        MapleCW/client/maplecw-hook.log    the game's hook log - send this with any problem
        mac-launch.log        what MapleCW.app did, and Wine's own messages
        prefix/               MapleCW's Wine prefix (its own C: drive)

To start completely fresh, quit everything and delete that folder.


SOMETHING WENT WRONG?
---------------------

Send mac-launch.log, MapleCW/client/maplecw-hook.log and the text in the launcher's log pane.
The launcher's log starts with a line saying "running under Wine ... the Mac client"; if it
does not, the log is not from the Mac client.

One difference from Windows: on Windows the launcher adds a firewall rule that stops the game
from reaching anything but your server. There is no such rule on a Mac, and the launcher's
log says so at Start Game.
