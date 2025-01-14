Write-Host "Running troubleshooting checks..." -ForegroundColor Yellow

# Function to check command existence
function Test-Command {
    param ($Command)
    return [bool](Get-Command -Name $Command -ErrorAction SilentlyContinue)
}

# Check Docker
Write-Host "`nChecking Docker..." -ForegroundColor Green
if (Test-Command "docker") {
    $dockerVersion = docker version --format '{{.Server.Version}}'
    Write-Host "Docker is installed (Version: $dockerVersion)" -ForegroundColor Green
    
    $dockerRunning = docker info 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Docker is running" -ForegroundColor Green
    } else {
        Write-Host "Docker is not running! Please start Docker Desktop" -ForegroundColor Red
    }
} else {
    Write-Host "Docker is not installed!" -ForegroundColor Red
}

# Check Docker Compose
Write-Host "`nChecking Docker Compose..." -ForegroundColor Green
if (Test-Command "docker-compose") {
    $composeVersion = docker-compose version --short
    Write-Host "Docker Compose is installed (Version: $composeVersion)" -ForegroundColor Green
} else {
    Write-Host "Docker Compose is not installed!" -ForegroundColor Red
}

# Check Rust
Write-Host "`nChecking Rust..." -ForegroundColor Green
if (Test-Command "rustc") {
    $rustVersion = rustc --version
    Write-Host "Rust is installed ($rustVersion)" -ForegroundColor Green
} else {
    Write-Host "Rust is not installed!" -ForegroundColor Red
}

# Check WSL
Write-Host "`nChecking WSL..." -ForegroundColor Green
if (Test-Command "wsl") {
    $wslVersion = wsl --status
    Write-Host "WSL is installed" -ForegroundColor Green
} else {
    Write-Host "WSL is not installed!" -ForegroundColor Red
}

# Check Database Connection
Write-Host "`nChecking database connection..." -ForegroundColor Green
$containerStatus = docker-compose -f docker/docker-compose.yml ps
Write-Host "Container Status:" -ForegroundColor Yellow
Write-Host $containerStatus

# Check Database Logs
Write-Host "`nRecent database logs:" -ForegroundColor Green
docker-compose -f docker/docker-compose.yml logs --tail=10 timescaledb

Write-Host "`nTroubleshooting complete!" -ForegroundColor Green
Write-Host @"

If you're experiencing issues:
1. Ensure Docker Desktop is running
2. Ensure WSL 2 is enabled in Docker Desktop settings
3. Try resetting the database: .\scripts\reset-db.ps1
4. Check the logs: docker-compose -f docker/docker-compose.yml logs
"@ -ForegroundColor Yellow