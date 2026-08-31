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
rem -PoolSentry is a DIAGNOSTIC and it is here on purpose while the heap crash is open.
rem It fixes NOTHING. It watches the client's own allocator every 100 ms and snapshots a
rem slot the moment its header goes bad - about 106 ms after the write instead of tens of
rem thousands of allocations later, which is the whole reason it can settle what a dump
rem cannot. It costs 0.08% of one core and writes a heartbeat every 60 s.
rem
rem DELETE the -PoolSentry below once the crash is understood. Leaving an instrument armed
rem after it has answered its question is how a measurement turns into background noise.
rem
rem Do NOT add -HeapFix. It voids the free-list argument the sentry exists to exploit, and
rem it patches one of three entry points anyway - research/heapfix-did-not-hold.md.
powershell -NoProfile -ExecutionPolicy Bypass -File "%REPO%tools\test-server.ps1" -SetFieldProbe -ServersOnly -PoolSentry

rem Reached only if the script returned on its own - a server exiting early, or
rem Ctrl+C. On a window close nothing here runs, and nothing here needs to.
echo.
echo The server has stopped. This window can be closed.
pause
