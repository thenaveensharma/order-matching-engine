@echo off
REM Stop containers
docker-compose -f docker/docker-compose.yml down

REM Remove volume
docker volume rm docker_trading_engine_data

REM Start fresh
docker-compose -f docker/docker-compose.yml up -d