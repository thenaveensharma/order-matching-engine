#!/bin/bash
set -e

# Start the database
docker-compose -f docker/docker-compose.yml up -d

# Wait for database to be ready
echo "Waiting for database to be ready..."
sleep 5

# Run the application
cargo run