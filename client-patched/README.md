# client-patched/ - your copy of the client goes here

This folder is empty in the repository on purpose. Put a **fresh copy of the MapleStory Classic
World client from the second closed online test (COT#2)** here - WZ data version **779**, the
build this server is written against.

Copy the **contents** of the install's `appdata` folder, so that `MapleStory.exe` and `Data\`
sit directly in this folder:

```text
client-patched\
    MapleStory.exe
    grap64.dll
    Data\
        Base\  Character\  Etc\  Item\  Map\  Mob\  Quest\  String\  ...
    ... (the other DLLs beside MapleStory.exe)
```

With the Nexon Launcher's default install location, from an administrator PowerShell window:

```powershell
robocopy "C:\Nexon\Library\maplestorycw\appdata" "C:\MapleCW\client-patched" /E
```

(Substitute your own repository path for `C:\MapleCW`.)

**Why a copy:** everything in this project patches THIS folder - the GameGuard stub, the hook,
the backported items, the quest patch. The original install is never modified, so it stays a
clean reference and can always be copied again.

**Never commit anything from here.** `.gitignore` keeps everything in this folder except this
file out of git: the client and its game data are not this project's to publish.

Next step: [Getting started](../README.md#getting-started-a-server-on-your-own-machine), step 2.
