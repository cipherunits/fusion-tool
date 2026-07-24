@echo off
setlocal

set REPO=cipherunits/fusion-tool
set VERSION=latest

if not "%1"=="" set VERSION=%1

echo Installing fusion-tool v%VERSION%...

set INSTALL_DIR=%LOCALAPPDATA%\Microsoft\WindowsApps

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo Downloading fusion-tool...
powershell -Command "Invoke-WebRequest -Uri 'https://github.com/%REPO%/releases/download/%VERSION%/fusion-x86_64-pc-windows-msvc.exe' -OutFile '%INSTALL_DIR%\fusion.exe'"

echo.
echo ✔ fusion-tool installed successfully!
echo   Location: %INSTALL_DIR%\fusion.exe
echo.
echo Make sure %INSTALL_DIR% is in your PATH.