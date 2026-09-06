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
rem  The sign-in service's port. 8080 is a popular port and something else on
rem  this machine may already hold it - a proxy, a dev server, IIS Express. When
rem  that happens maplecw-auth cannot bind, exits a few seconds after starting,
rem  and the window says so in red with the bind error from auth.log.err.
rem
rem  Change the number here, and then BOTH of these must match it:
rem    - auth_port = "<port>" in maplecw-launcher.toml on every client machine
rem      (or install.ps1 -AuthPort <port>)
rem    - the inbound firewall rule on this machine
set "AUTHPORT=8080"
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
