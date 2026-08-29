MapleCW - the server, on its own machine
========================================

Testing only, on machines you own and a network you control.


WHAT IS IN HERE, AND WHAT IS NOT
--------------------------------

    bin\maplecw-login.exe     login and character select        TCP 8484
    bin\maplecw-world.exe     one channel; one process each     TCP 8485, 8486
    bin\maplecw-auth.exe      sign-in. The launcher POSTs here  TCP 8080
    bin\maplecw-useradd.exe   accounts, GM status, codes
    gm-handbook\              game tables extracted from the client's WZ
    data\                     shops, drops, quest scripts, the EXP curve

Not here, and not needed here: the game client, its 450 MB of WZ data, the
launcher, grap64.dll. Those are the CLIENT machine's payload.

Also not here: maplecw.db. See "BRINGING AN EXISTING DATABASE" below.


IT NEEDS NOTHING INSTALLED
--------------------------

No Rust. No .NET. No Visual C++ Redistributable.

These executables are built with the C runtime linked in, and the only DLLs
they import ship with Windows:

    KERNEL32  ADVAPI32  WS2_32  ntdll  bcrypt  bcryptprimitives
    api-ms-win-core-synch-l1-2-0

That is checked when the package is built, not assumed - `tools\package-server.ps1`
refuses to produce a package whose binaries import VCRUNTIME140.dll. An ordinary
build DOES import it, which is the trap this avoids: it works on the machine
that built it and fails on a clean one with a missing-DLL dialog.

SQLite is compiled in. There is no database engine to install.


SETUP
-----

1. Unzip anywhere. C:\MapleCW-server is fine.

2. Make an account, and give it GM:

     & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" <name> --email <address>
     & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" --gm <name>

   The leading & is required - PowerShell reads a line starting with a quoted
   string as a string, not a command.

3. Open inbound TCP 8080, 8484, 8485 and 8486 on this machine:

     netsh advfirewall firewall add rule name="MapleCW server" dir=in action=allow protocol=TCP localport=8080,8484,8485,8486

4. Double-click start-servers.cmd. THAT WINDOW IS THE SERVER - closing it stops
   everything, and there is no stop script to forget.


THE TWO THINGS THAT WILL GO WRONG
---------------------------------

Both of these produce the same symptom - the client sits on "Connecting..."
forever - and neither says anything about itself.

1. THE CHANNEL ADDRESSES MUST BE THIS MACHINE'S LAN IP, NOT 127.0.0.1.

   When a player enters the world the login server hands the client an address
   to connect to. That address is used BY THE CLIENT MACHINE, so loopback means
   "the client's own computer", which is not where the server is.

   start-server.ps1 binds 0.0.0.0 and advertises whatever -Bind says, so pass
   the real address:

     powershell -ExecutionPolicy Bypass -File ".\start-server.ps1" -Bind 192.168.1.20

   Character select will work perfectly with this wrong and the world will not,
   which is the confusing half.

2. THE CLIENT MACHINE'S FIREWALL RULE BLOCKS THE SERVER.

   The rule that keeps the patched client away from Nexon blocks outbound to
   RemoteIP=Any. Windows Firewall does not filter loopback, which is why a
   local server has always worked - and why a LAN server silently will not.

   An "allow" rule alongside does not help: Windows evaluates block before
   allow. The block itself has to be narrowed. On each CLIENT machine, from an
   elevated window:

     powershell -ExecutionPolicy Bypass -File "<repo>\tools\firewall.ps1" -AllowLan

   That blocks the public internet and permits private addresses. Nexon is
   public and stays blocked.


BRINGING AN EXISTING DATABASE
-----------------------------

Copy maplecw.db AND maplecw.db-wal AND maplecw.db-shm, all three.

SQLite runs in WAL mode here, so recent writes live in the -wal sidecar until a
checkpoint. Copying the .db alone loses them and it does not look like loss -
the database opens, and the last few accounts or characters are simply not in
it. That happened on this project on 2026-08-29 and read as data corruption for
several minutes.

Stop the servers before copying.


WHAT THE LAUNCHER NEEDS TO KNOW
-------------------------------

On each client machine, in the launcher:

    Server IP    this machine's LAN address
    Port         8484   (the game / login server)

The sign-in service is assumed to be on 8080 at the same address. If you moved
it, set auth_port in maplecw-launcher.toml beside the launcher.


WHAT IS AND IS NOT PROTECTED
----------------------------

The launcher checks a password with argon2id before it says who is playing.

THE PASSWORD CROSSES THE NETWORK IN PLAIN TEXT. Sign-in is plain HTTP, so
anything between the client and this machine can read it. That is the price of
being installable and it is fine on a network you control; it is not fine on
the internet.

THE GAME SOCKET CARRIES NO CREDENTIALS AT ALL. The client never sends a user
name, so anything that can reach port 8484 is served as whichever account
signed in last. Do not port-forward any of this.
