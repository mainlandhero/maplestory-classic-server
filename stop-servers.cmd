@echo off
setlocal
rem ===========================================================================
rem  Stop the MapleCW servers. Double-clickable, like start-servers.cmd.
rem
rem  Elevates first: the servers were started from an elevated window, and an
rem  unelevated Stop-Process against them fails with "Access is denied" - which
rem  reads as "they would not die" rather than as "ask properly".
rem ===========================================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Asking for administrator...
    powershell -NoProfile -ExecutionPolicy Bypass -Command "Start-Process -Verb RunAs -FilePath '%~f0'"
    exit /b
)

set "REPO=%~dp0"
powershell -NoProfile -ExecutionPolicy Bypass -File "%REPO%tools\test-server.ps1" -Stop
echo.
pause
