@echo off
setlocal
rem  Stop the MapleCW servers. Double-clickable, like start-servers.cmd.
set "HERE=%~dp0"
powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%start-server.ps1" -Stop
echo.
pause
