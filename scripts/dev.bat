@echo off
REM Start the database
docker-compose -f docker/docker-compose.yml up -d

REM Wait for database to be ready
echo Waiting for database to be ready...
timeout /t 5 /nobreak

REM Run the application
cargo run