Write-Host "Starting development environment..." -ForegroundColor Green

# Check if Docker is running
$dockerStatus = docker info 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Docker is not running. Please start Docker Desktop first." -ForegroundColor Red
    exit 1
}

# Start the database
Write-Host "Starting database containers..." -ForegroundColor Yellow
docker-compose -f docker/docker-compose.yml up -d

# Wait for database to be ready
Write-Host "Waiting for database to be ready..." -ForegroundColor Yellow
$attempts = 0
$maxAttempts = 30

while ($attempts -lt $maxAttempts) {
    $health = docker inspect --format='{{.State.Health.Status}}' trading_engine_db 2>&1
    if ($health -eq "healthy") {
        Write-Host "Database is ready!" -ForegroundColor Green
        break
    }
    $attempts++
    Write-Host "Waiting for database to be ready... (Attempt: $attempts/$maxAttempts)" -ForegroundColor Yellow
    Start-Sleep -Seconds 2
}

if ($attempts -eq $maxAttempts) {
    Write-Host "Error: Database failed to become ready in time" -ForegroundColor Red
    exit 1
}

# Run the application
Write-Host "Starting Rust application..." -ForegroundColor Green
cargo run