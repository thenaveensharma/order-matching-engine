Write-Host "Cleaning up development environment..." -ForegroundColor Yellow

# Stop all containers
Write-Host "Stopping containers..." -ForegroundColor Yellow
docker-compose -f docker/docker-compose.yml down

# Remove volumes
Write-Host "Removing volumes..." -ForegroundColor Yellow
docker volume rm docker_trading_engine_data -f

# Clean Rust build artifacts
Write-Host "Cleaning Rust artifacts..." -ForegroundColor Yellow
cargo clean

Write-Host "Cleanup complete!" -ForegroundColor Green