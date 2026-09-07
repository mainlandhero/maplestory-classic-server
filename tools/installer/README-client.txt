MapleCW - client
================

This machine PLAYS. It does not run the server, and it holds no accounts and no
characters - those live on the server, and this connects to it over the network.

Testing only. This runs a MapleStory client against a server written from scratch, on
machines you own. It is not connected to Nexon and must never be pointed at their servers.
The client and its WZ data in client\ are Nexon's; this payload exists so a machine you own
can run files you own, not as a way to hand them to anyone else.


BEFORE YOU START
----------------

This machine needs "Microsoft Visual C++ 2015-2022 Redistributable (x64)" -
vc_redist.x64.exe from Microsoft. The launcher and the GameGuard stub both import
VCRUNTIME140.dll from it. MapleStory itself does not, so having the game installed is
not evidence that it is here; check for C:\Windows\System32\vcruntime140.dll. install.ps1
stops with this message if it is missing rather than letting the launcher die at startup
with a missing-DLL dialog.

You also need the server to be running somewhere you can reach, and its address.

Nothing else. No Rust, no .NET, no Python.


INSTALL
-------

From an ELEVATED PowerShell window (PowerShell 5.1 - the one Windows ships):

    powershell -ExecutionPolicy Bypass -File "<this folder>\install.ps1" -ServerIp <server>

An elevated window opens in C:\Windows\System32, not here, so write the path in full.

-ServerIp is the one thing this machine cannot work out for itself. Everything else has a
default, and NO ACCOUNT IS CREATED HERE - you register in the launcher, below.

Options worth knowing:

    -ServerIp   192.168.1.20         the machine running the server. An IP, or a DNS NAME
                                     (a CNAME to the box is fine): the launcher resolves it
                                     each time and hands the client the address; the
                                     firewall rule below pins the address the name had NOW
                                     - re-run this if the name later moves.
                                     THIS ALSO SHAPES THE FIREWALL RULE: a LAN address
                                     leaves private addresses reachable, a public address
                                     leaves exactly that address reachable. Get it wrong
                                     and the client sits on "Connecting..."
    -InstallDir "D:\Games\MapleCW"   somewhere other than C:\MapleCW
    -AuthPort   8480                 the port the SERVER's sign-in service listens on, if
                                     it was moved off 8080. The two sides must agree or
                                     sign-in never connects
    -NoFirewall                      skip the outbound block rule (read the warning first)
    -SkipRuntimeCheck                install without the Visual C++ runtime present. The
                                     launcher will not start until you install it

You should NOT need -AuthFingerprint. This launcher has the server's certificate
fingerprint compiled in, so it already knows which certificate to trust. Pass one only if
the server was rebuilt with a new certificate, in which case its window prints the value.


RUN
---

One double-click.

    Double-click  C:\MapleCW\maplecw-launcher.exe   (or the desktop shortcut)

Windows asks for administrator. Say yes: the game client requires it, and accepting here
means the client does not ask a second time.

There is no server to start on this machine. If the launcher cannot reach one, check that
whoever runs it has it up, and that the address you passed to -ServerIp is right.


YOUR ACCOUNT
------------

You make it yourself, in the launcher - not with this installer, and not on this machine.

    REGISTER tab       username, email, password, and a single-use REGISTRATION CODE that
                       the administrator gives you. They mint it in game with
                       !registrationcode. Passwords must be at least 8 characters and
                       contain both letters and digits.

    FORGOT PASSWORD    your email or username, a recovery code from the administrator
                       (!recoverycode <your email>), and the new password.

Then type the account name (or its email) and the password, check the server address, and
press Login. When it succeeds, Start Game becomes available. Start Game installs the
GameGuard stub into client\ and launches the client.

THE LAUNCHER REMEMBERS. Once Start Game has worked, it saves the server address, the port,
the game folder and your account name into maplecw-launcher.remembered.toml beside
maplecw-launcher.exe, and fills them in for you next time. Only the password is not saved,
and never will be - you type that each time. Delete that file to go back to the defaults.

If your MapleStory.exe is somewhere other than client\, press Browse... beside "Game
folder" and pick it once.

Windows will raise a UAC prompt when the client starts. That is the client's own elevation
manifest, not something the launcher asks for.


WHAT IS AND IS NOT AUTHENTICATED
--------------------------------

The launcher really does check the password - argon2id, and a wrong one is refused. It also
refuses to send it to a server whose certificate it does not recognise.

The GAME SOCKET carries no credentials. The client never sends a username, so the server
cannot tell one connection from another; it serves whatever account the launcher last
claimed. Anything that can reach the login port gets that account. This is a test server on
a network you control, and it should stay that way.


ABOUT THE HITCH EVERY FEW MINUTES
---------------------------------

The client has a memory bug of its own that damages one small block roughly every three
minutes and would eventually crash it. The launcher arms a watcher inside the client that
repairs the damage instead. It is not a cure - the bug is still there - and the watcher is
configured to be as quiet as it can be, but a very short pause every few minutes is it.

If the client does crash, client\maplecw-hook.log says what happened and is worth keeping.


WHERE THINGS LAND
-----------------

    C:\MapleCW\maplecw-launcher.exe   what you run
    C:\MapleCW\grap64.dll             the GameGuard stub the launcher installs into client\
    C:\MapleCW\client\                the game client and its WZ data
    C:\MapleCW\client\maplecw-hook.log
                                      what the client did with each packet, and any crash

There is no database and no packet log on this machine. Those are on the server.


UNINSTALL
---------

Delete the install directory, remove the desktop shortcut, and drop the firewall rule:

    netsh advfirewall firewall delete rule name="MapleCW - block patched client outbound"
