@echo off
setlocal
rem ===========================================================================
rem  Start the MapleCW servers. Double-click this.
rem
rem  Windows will not RUN a .ps1 on double-click - it opens it in an editor -
rem  so this is the double-clickable half of start-server.ps1.
rem
rem  The CLIENT is not started here. Double-click maplecw-launcher.exe after
rem  this; it asks for administrator once, and the client inherits that
rem  elevation instead of raising a second prompt of its own.
rem ===========================================================================

set "HERE=%~dp0"
echo Starting the MapleCW servers from %HERE%
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%start-server.ps1"

echo.
echo ---------------------------------------------------------------------
echo  Now double-click:  %HERE%maplecw-launcher.exe
echo  Stop the servers:  %HERE%stop-servers.cmd
echo ---------------------------------------------------------------------
pause
