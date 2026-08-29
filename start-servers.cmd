@echo off
setlocal
rem ===========================================================================
rem  The MapleCW server, in one window. Double-click this.
rem
rem  The owner, 2026-08-28: "Instead of having a start and stop script, I would like
rem  a single shell that stays open while the server runs, then if I close that
rem  shell, the server gracefully stops."
rem
rem  So: this window IS the server. Closing it stops it, and there is no stop
rem  script to forget. Windows will not RUN a .ps1 on double-click - it opens
rem  it in an editor - which is the only reason this wrapper exists.
rem
rem  The CLIENT is not started here. Double-click maplecw-launcher.exe after
rem  this; it asks for administrator once, and the client inherits that
rem  elevation instead of raising a second prompt of its own.
rem ===========================================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Asking for administrator...
    rem -Wait so THIS window does not vanish behind the elevated one and leave two
    rem consoles on screen, one of which does nothing.
    powershell -NoProfile -ExecutionPolicy Bypass -Command "Start-Process -Verb RunAs -FilePath '%~f0' -Wait"
    exit /b
)

rem %~dp0 ends with a backslash, so there is deliberately none before tools.
set "REPO=%~dp0"

rem -SetFieldProbe is NOT optional. Without it Session::handle returns nothing
rem for every packet and the client sits on "Connecting..." looking exactly
rem like a server that is not running. That has already cost a manual launch,
rem so it is not exposed as a choice here.
rem
rem -ServersOnly starts the servers ATTACHED to this console and waits. That is
rem what makes closing this window stop them: every process attached to a
rem console gets CTRL_CLOSE_EVENT when it closes.
powershell -NoProfile -ExecutionPolicy Bypass -File "%REPO%tools\test-server.ps1" -SetFieldProbe -ServersOnly

rem Reached only if the script returned on its own - a server exiting early, or
rem Ctrl+C. On a window close nothing here runs, and nothing here needs to.
echo.
echo The server has stopped. This window can be closed.
pause
