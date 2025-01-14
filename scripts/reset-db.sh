#!/bin/bash
set -e

# Stop containers
docker-compose -f docker/docker-compose.yml down

# Remove volume
docker volume rm docker_trading_engine_data

# Start fresh
docker-compose -f docker/docker-compose.yml up -d