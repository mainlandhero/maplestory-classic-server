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
    api-ms-win-core-synch-l1-2-0    IPHLPAPI (login and world only)

That is checked when the package is built, not assumed - `tools\package-server.ps1`
refuses to produce a package whose binaries import VCRUNTIME140.dll. An ordinary
build DOES import it, which is the trap this avoids: it works on the machine
that built it and fails on a clean one with a missing-DLL dialog.

SQLite is compiled in. There is no database engine to install.


SETUP
-----

1. Unzip anywhere. C:\MapleCW-server is fine.

2. Make YOUR account, and give it GM:

     & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" <name> --email <address>
     & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" --gm <name>

   The leading & is required - PowerShell reads a line starting with a quoted
   string as a string, not a command.

   OTHER PLAYERS register themselves through the launcher, but only with a
   single-use code you mint. In game, as the GM:

     !registrationcode                  a code that lets one person register
                                        (Register tab in the launcher; 7 days)
     !recoverycode <email or username>  a code that lets THAT account set a
                                        new password (Forgot password tab;
                                        24 hours)

   The code appears as a chat notice on your screen and nowhere else - the
   server keeps only a hash and never logs it. The same codes come from this
   console with --registration-code and --recovery-code <name|email>. A lost
   code is replaced, not looked up. Passwords players choose must be at least
   8 characters with a letter and a digit; the launcher says so before sending.

3. Open inbound TCP 8080, 8484, 8485 and 8486 on this machine:

     netsh advfirewall firewall add rule name="MapleCW server" dir=in action=allow protocol=TCP localport=8080,8484,8485,8486

   8080 IS A POPULAR PORT. If something on this box already holds it - a proxy, a
   dev server, IIS Express - maplecw-auth cannot bind and exits on its own a few
   seconds after starting. Either free it, or move the sign-in service:

     powershell -ExecutionPolicy Bypass -File ".\start-server.ps1" -AuthPort 8090

   and put auth_port = "8090" in maplecw-launcher.toml on every client, and open
   8090 instead of 8080 above. To see what holds a port:

     netstat -ano | findstr ":8080 :8484 :8485 :8486"

4. Double-click start-servers.cmd. THAT WINDOW IS THE SERVER - closing it stops
   everything, and there is no stop script to forget.

5. Copy the sign-in certificate's fingerprint to every client machine. The
   server window prints it:

     TLS: fingerprint sha256:<64 hex characters>

   (it is also in auth-cert-fingerprint.txt beside maplecw.db). On each client,
   either install with `install.ps1 -AuthFingerprint <that value>`, or set
   `auth_fingerprint = "sha256:..."` in maplecw-launcher.toml beside the
   launcher, or copy the .txt file beside the launcher. A launcher with no
   fingerprint refuses to sign in rather than sending the password anywhere.

   The certificate is generated on the FIRST start and kept in auth-cert.pem
   and auth-key.pem beside the database. Delete them and the next start makes
   a new pair - and every client's pin stops matching until it is updated.
   That is the same thing a client would see under an impostor server, and it
   is meant to be indistinguishable.


THE TWO THINGS THAT WILL GO WRONG
---------------------------------

Both of these produce the same symptom - the client sits on "Connecting..."
forever - and neither says anything about itself.

1. THE ADDRESS THE CLIENT IS TOLD TO DIAL MUST BE ONE IT CAN REACH.

   When a player enters the world the login server hands the client an address
   to connect to. That address is used BY THE CLIENT MACHINE, so loopback means
   "the client's own computer" and a LAN address means nothing to a client on
   the internet.

   This is decided PER CONNECTION now, and the default needs no configuration:
   a client on your LAN or VPN is told the address it reached this box on, and
   a client on the internet is told this box's PUBLIC address, which the server
   discovers at startup. The first lines of login.log say what was found:

     advertise: AUTO - each client is told the host it can actually reach:
       public address: 203.0.113.5 (from checkip.amazonaws.com, ...)

   If that line says UNKNOWN - no internet at startup, or the echo services
   were unreachable - LAN clients still work and internet clients will not.
   Pin the address by hand:

     powershell -ExecutionPolicy Bypass -File ".\start-server.ps1" -Advertise 203.0.113.5

   Character select works perfectly with this wrong and the world does not,
   which is the confusing half. Every migration line in login.log ends with
   "Advertised as <address>, <why>", so a stuck client can be read back.

2. THE CLIENT MACHINE'S FIREWALL RULE BLOCKS THE SERVER.

   The rule that keeps the patched client away from Nexon blocks outbound to
   RemoteIP=Any. Windows Firewall does not filter loopback, which is why a
   local server has always worked - and why a LAN server silently will not.

   An "allow" rule alongside does not help: Windows evaluates block before
   allow. The block itself has to be narrowed. On each CLIENT machine, from an
   elevated window - for a server on the LAN:

     powershell -ExecutionPolicy Bypass -File "<repo>\tools\firewall.ps1" -Add -AllowLan

   That blocks the public internet and permits private addresses. Nexon is
   public and stays blocked. For a server reached over the INTERNET - this
   box's public address behind forwarded ports - -AllowLan is NOT enough,
   because that address is inside the ranges it blocks:

     powershell -ExecutionPolicy Bypass -File "<repo>\tools\firewall.ps1" -Add -AllowServer <public ip>

   That blocks everything except the server. The shipped install.ps1 derives
   the same shape from its -ServerIp, so a client installed with the right
   address needs neither. -Status prints the rule's RemoteIP line either way.


IF A SERVER EXITS ON ITS OWN
----------------------------

The window says so in red, names which one it was, and prints the last lines of THAT
server's log and its .err file - login.log, auth.log or world.log, whichever died.
Read those lines first; they carry the reason.

The most common one is a port already in use, and it reads as

  server error: could not bind 0.0.0.0:8080: Only one usage of each socket address
  (protocol/network address/port) is normally permitted. (os error 10048)

See the -AuthPort note in step 3. Until 2026-09-06 this message always told you to
read login.log no matter which server had died, and printed a blank exit code.


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


REACHING IT FROM OUTSIDE YOUR NETWORK
-------------------------------------

On the router, forward TCP 8080, 8484, 8485 and 8486 to this machine - one more
port per extra channel (-Channels). Nothing here uses UDP.

The server advertises the right host on its own: an internet client is told the
public address discovered at startup, a LAN client is still told the LAN
address, so the router does not need NAT hairpinning. Discovery asks
checkip.amazonaws.com, api.ipify.org and icanhazip.com in turn over plain HTTP
and refuses any answer that is not a public address; it is re-checked every ten
minutes, and a change is logged.

Read the next section before you do this. Forwarded to the internet, the
second fact in it is the entire security posture of this server.


WHAT IS AND IS NOT PROTECTED
----------------------------

The launcher checks a password with argon2id before it says who is playing.

SIGN-IN IS TLS TO A PINNED CERTIFICATE. The server makes its own certificate
and every launcher is told its fingerprint (step 5 above); the launcher accepts
that certificate and no other, and refuses to send a password at all when it
has no fingerprint. Nobody on the path can read the password or the launch
token, and an impostor server fails the handshake before a byte is sent. Until
2026-09-05 this section said the password crossed in plain text; it does not.

THE GAME SOCKET CARRIES NO CREDENTIALS AT ALL. The client never sends a user
name. A connection the server can tie to a launcher sign-in is served as that
sign-in's account; one it cannot is REFUSED with a login failure and sees no
characters (since 2026-09-05 - it used to be served a fallback account). What
ties a connection to a sign-in is, in order: the one-time token the launcher
gives the client to carry, the process that owns the socket (same machine),
and the address. So on a forwarded port the remaining exposure is narrower:
a stranger is served as you only if their connection is attributed to your
sign-in - by arriving from your address while your claim is live - not merely
by connecting.
