@echo off
REM PeRust Server Startup Script (Windows)
REM
REM This script checks for the PeRust server binary and runs it
REM with the proper settings.
REM

setlocal enabledelayedexpansion

REM --- Configuration ---
set "SERVER_DIR=%~dp0"
set "BINARY_NAME=perust.exe"
REM ---------------------

echo PeRust Server Launcher
echo.

REM Check if the binary exists
if exist "%SERVER_DIR%\%BINARY_NAME%" (
    set "BINARY=%SERVER_DIR%\%BINARY_NAME%"
) else if exist "%SERVER_DIR%\target\debug\%BINARY_NAME%" (
    set "BINARY=%SERVER_DIR%\target\debug\%BINARY_NAME%"
) else if exist "%SERVER_DIR%\target\release\%BINARY_NAME%" (
    set "BINARY=%SERVER_DIR%\target\release\%BINARY_NAME%"
) else (
    echo Binary not found. Building PeRust server...
    cd /d "%SERVER_DIR%\.."

    REM Build the server
    where cargo >nul 2>&1
    if %ERRORLEVEL% equ 0 (
        cargo build --bin perust
        if %ERRORLEVEL% neq 0 (
            echo Failed to build PeRust server!
            exit /b 1
        )
        set "BINARY=%SERVER_DIR%\..\target\debug\%BINARY_NAME%"
    ) else (
        echo Cargo not found! Please install Rust: https://rustup.rs/
        exit /b 1
    )
)

echo Using binary: %BINARY%
echo.

REM Create data directory if it doesn't exist
if not exist "%SERVER_DIR%\data" mkdir "%SERVER_DIR%\data"
if not exist "%SERVER_DIR%\data\worlds" mkdir "%SERVER_DIR%\data\worlds"
if not exist "%SERVER_DIR%\data\plugins" mkdir "%SERVER_DIR%\data\plugins"

REM Run the server
echo Starting PeRust server...
echo.

"%BINARY%" %*

endlocal
