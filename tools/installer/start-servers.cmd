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

set "HERE=%~dp0"
echo Starting the MapleCW server from %HERE%
echo Close this window to stop it.
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%start-server.ps1"

rem Reached only if the script returned on its own - a server exiting early, or
rem Ctrl+C. On a window close nothing here runs, and nothing here needs to.
echo.
echo The server has stopped. This window can be closed.
pause
