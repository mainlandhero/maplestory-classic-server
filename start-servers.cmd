@echo off
setlocal
rem ===========================================================================
rem  Start the MapleCW servers by double-clicking this file.
rem
rem  The owner, 2026-08-28: "I'm happy to start the server by double clicking a
rem  powershell script, but the client I would also like to start by double
rem  clicking the new login client as administrator."
rem
rem  Windows will not RUN a .ps1 on double-click - it opens it in an editor -
rem  so this .cmd is the double-clickable half. It re-launches itself elevated
rem  (tools\test-server.ps1 expects an elevated window; CLAUDE.md says so and
rem  stopping a server started elevated needs it), then hands over.
rem
rem  The client is NOT started here. Run maplecw-launcher afterwards - it has
rem  its own requireAdministrator manifest, so double-clicking it raises UAC
rem  once and the client inherits that elevation instead of asking again.
rem ===========================================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Asking for administrator...
    powershell -NoProfile -ExecutionPolicy Bypass -Command "Start-Process -Verb RunAs -FilePath '%~f0'"
    exit /b
)

rem %~dp0 ends with a backslash, so there is deliberately none before tools.
set "REPO=%~dp0"
echo Starting the MapleCW servers from %REPO%
echo.

rem -SetFieldProbe is NOT optional. Without it Session::handle returns nothing
rem for every packet and the client sits on "Connecting..." looking exactly
rem like a server that is not running. That has already cost a manual launch,
rem so it is not exposed as a choice here.
powershell -NoProfile -ExecutionPolicy Bypass -File "%REPO%tools\test-server.ps1" -SetFieldProbe -ServersOnly

echo.
echo ---------------------------------------------------------------------
echo  Servers are up. Now double-click maplecw-launcher.exe:
echo    %REPO%target\release\maplecw-launcher.exe
echo.
echo  Stop the servers with:
echo    "%REPO%stop-servers.cmd"
echo ---------------------------------------------------------------------
pause
