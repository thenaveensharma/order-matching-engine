Write-Host "Resetting database environment..." -ForegroundColor Yellow

# Stop containers
Write-Host "Stopping containers..." -ForegroundColor Yellow
docker-compose -f docker/docker-compose.yml down

# Remove volume
Write-Host "Removing old volume..." -ForegroundColor Yellow
docker volume rm docker_trading_engine_data -f

# Start fresh
Write-Host "Starting fresh containers..." -ForegroundColor Yellow
docker-compose -f docker/docker-compose.yml up -d

Write-Host "Waiting for database to be ready..." -ForegroundColor Yellow
$attempts = 0
$maxAttempts = 30

while ($attempts -lt $maxAttempts) {
    $health = docker inspect --format='{{.State.Health.Status}}' trading_engine_db 2>&1
    if ($health -eq "healthy") {
        Write-Host "Database reset complete!" -ForegroundColor Green
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