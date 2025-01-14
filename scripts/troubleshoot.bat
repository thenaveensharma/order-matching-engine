@echo off
setlocal enabledelayedexpansion

echo Running troubleshooting checks...
echo.

:: Check Docker
echo Checking Docker...
docker version >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%i in ('docker version --format "{{.Server.Version}}"') do set DOCKER_VERSION=%%i
    echo [32m√ Docker is installed and running (Version: %DOCKER_VERSION%^)[0m
) else (
    echo [31m× Docker is not running or not installed![0m
    echo Please ensure Docker Desktop is running.
)

:: Check Docker Compose
echo.
echo Checking Docker Compose...
docker-compose version >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%i in ('docker-compose version --short') do set COMPOSE_VERSION=%%i
    echo [32m√ Docker Compose is installed (Version: %COMPOSE_VERSION%^)[0m
) else (
    echo [31m× Docker Compose is not installed![0m
)

:: Check Rust
echo.
echo Checking Rust...
rustc --version >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
    echo [32m√ Rust is installed (%RUST_VERSION%^)[0m
) else (
    echo [31m× Rust is not installed![0m
)

:: Check WSL (Windows only)
echo.
echo Checking WSL...
wsl --status >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [32m√ WSL is installed and configured[0m
) else (
    echo [31m× WSL is not installed or not configured![0m
    echo Please enable WSL 2 in Docker Desktop settings.
)

:: Check Database Connection
echo.
echo Checking database connection...
echo Container Status:
docker-compose -f docker/docker-compose.yml ps

:: Check Database Logs
echo.
echo Recent database logs:
docker-compose -f docker/docker-compose.yml logs --tail=10 timescaledb

echo.
echo Troubleshooting complete!
echo.
echo If you're experiencing issues:
echo 1. Ensure Docker Desktop is running
echo 2. Ensure WSL 2 is enabled in Docker Desktop settings
echo 3. Try resetting the database: scripts\reset-db.bat
echo 4. Check the logs: docker-compose -f docker/docker-compose.yml logs

endlocal