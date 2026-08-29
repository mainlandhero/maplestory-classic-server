MapleCW - private local test server
==================================

Testing only. This runs a MapleStory client against a server written from scratch, on
machines you own. It is not connected to Nexon and must never be pointed at their servers.
The client and its WZ data in client\ are Nexon's; this payload exists so a machine you own
can run files you own, not as a way to hand them to anyone else.


INSTALL
-------

From an ELEVATED PowerShell window (PowerShell 5.1 - the one Windows ships):

    powershell -ExecutionPolicy Bypass -File "<this folder>\install.ps1"

An elevated window opens in C:\Windows\System32, not here, so write the path in full.

It will ask for an account name, an optional email, and a password (twice). The password is
hashed with argon2id and stored salted; nothing keeps it in plain text.

Options worth knowing:

    -InstallDir "D:\Games\MapleCW"   somewhere other than C:\MapleCW
    -ServerIp   192.168.1.20         the machine running the servers, if not this one
    -NoFirewall                      skip the outbound block rule (read the warning first)
    -NoAccount                       install without creating an account


RUN
---

Two double-clicks. No typing.

1. Double-click  C:\MapleCW\start-servers.cmd
   THAT WINDOW IS THE SERVER. Leave it open while you play; close it when you
   are done and the server stops with it. There is no stop script to forget.

2. Double-click  C:\MapleCW\maplecw-launcher.exe   (or the desktop shortcut)
   Windows asks for administrator. Say yes: the game client requires it, and
   accepting here means the client does not ask a second time.

Closing the server window terminates the servers rather than asking them to
finish, and nothing is lost by that: every log line is flushed as it is written,
and the database is in WAL mode, which is crash-safe by design.

If you would rather use a PowerShell window, the same two steps are:

       powershell -ExecutionPolicy Bypass -File "C:\MapleCW\start-server.ps1"
       & "C:\MapleCW\maplecw-launcher.exe"

   The leading & is required. PowerShell reads a line starting with a quoted string as
   a string, not a command, so without it you get "Unexpected token".

3. Type the account name (or its email) and the password, check the server IP, and press
   Login. When it succeeds, Start Game becomes available. Start Game installs the GameGuard
   stub into client\ and launches the client.

Windows will raise a UAC prompt when the client starts. That is the client's own elevation
manifest, not something this launcher asks for.


MORE ACCOUNTS
-------------

    & "C:\MapleCW\bin\maplecw-useradd.exe" --db "C:\MapleCW\maplecw.db" <name> --email <addr>
    & "C:\MapleCW\bin\maplecw-useradd.exe" --db "C:\MapleCW\maplecw.db" --list

Each account has its own characters. The launcher decides which one is playing; sign in
again as someone else and press Start Game to swap. You do not need to restart the servers.


WHAT IS AND IS NOT AUTHENTICATED
--------------------------------

The launcher really does check the password - argon2id, and a wrong one is refused.

The GAME SOCKET carries no credentials. The client never sends a username, so the server
cannot tell one connection from another; it serves whatever account the launcher last
claimed. Anything that can reach the login port gets that account. This is a test server on
a network you control, and it should stay that way.


WHERE THINGS LAND
-----------------

    C:\MapleCW\maplecw-launcher.exe   what you run
    C:\MapleCW\grap64.dll        the GameGuard stub the launcher installs into client\
    C:\MapleCW\bin\              the login and channel servers, and maplecw-useradd
    C:\MapleCW\client\           the game client and its WZ data
    C:\MapleCW\maplecw.db        accounts and characters. Back this up; nothing else here
                                 is irreplaceable
    C:\MapleCW\login.log         every packet on the login connection
    C:\MapleCW\world.log         the same for channel 0
    C:\MapleCW\previous-runs\    earlier logs, archived rather than deleted
    C:\MapleCW\client\maplecw-hook.log
                                 what the client did with each packet, and any crash


UNINSTALL
---------

Delete the install directory, remove the desktop shortcut, and drop the firewall rule:

    netsh advfirewall firewall delete rule name="MapleCW - block patched client outbound"
