@echo off
setlocal
rem ===========================================================================
rem  The MapleCW server, in one window. Double-click this.
rem
rem  This window IS the server. Closing it stops it, so there is no stop script
rem  to forget. Windows will not RUN a .ps1 on double-click - it opens it in an
rem  editor - which is the only reason this wrapper exists.
rem
rem  The CLIENT is not started here. Double-click maplecw-launcher.exe after
rem  this; it asks for administrator once, and the client inherits that
rem  elevation instead of raising a second prompt of its own.
rem ===========================================================================

rem  ---- SETTINGS -----------------------------------------------------------
rem  The sign-in service's port. 8480 since 2026-09-07, and it was 8080.
rem
rem  8080 is one of the most contended ports on a Windows box - a proxy, a dev
rem  server, IIS Express - and when something holds it maplecw-auth cannot bind,
rem  exits a few seconds after starting, and the window says so in red with the
rem  bind error from auth.log.err. Moving it then cost a change here AND on every
rem  client, so the DEFAULT moved instead: 8480 is now what crates\auth,
rem  start-server.ps1 and the launcher all use with nothing set anywhere.
rem
rem  Leave this alone unless 8480 is taken too. If you do change it, BOTH of
rem  these must match:
rem    - the launcher's "Sign-in port" box on every client (it is remembered
rem      after the first successful Start Game), or auth_port in
rem      maplecw-launcher.toml
rem    - the inbound firewall rule on this machine
set "AUTHPORT=8480"
rem  --------------------------------------------------------------------------

set "HERE=%~dp0"
echo Starting the MapleCW server from %HERE%
echo Close this window to stop it.
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%start-server.ps1" -AuthPort %AUTHPORT%

rem Reached only if the script returned on its own - a server exiting early, or
rem Ctrl+C. On a window close nothing here runs, and nothing here needs to.
echo.
echo The server has stopped. This window can be closed.
pause
