#!/bin/bash

# Set colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running troubleshooting checks...${NC}\n"

# Check Docker
echo -e "${GREEN}Checking Docker...${NC}"
if command -v docker &> /dev/null; then
    DOCKER_VERSION=$(docker version --format '{{.Server.Version}}' 2>/dev/null)
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Docker is installed and running (Version: $DOCKER_VERSION)${NC}"
    else
        echo -e "${RED}✗ Docker is installed but not running!${NC}"
        echo "Please start Docker and try again"
    fi
else
    echo -e "${RED}✗ Docker is not installed!${NC}"
fi

# Check Docker Compose
echo -e "\n${GREEN}Checking Docker Compose...${NC}"
if command -v docker-compose &> /dev/null; then
    COMPOSE_VERSION=$(docker-compose version --short)
    echo -e "${GREEN}✓ Docker Compose is installed (Version: $COMPOSE_VERSION)${NC}"
else
    echo -e "${RED}✗ Docker Compose is not installed!${NC}"
fi

# Check Rust
echo -e "\n${GREEN}Checking Rust...${NC}"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust is installed ($RUST_VERSION)${NC}"
else
    echo -e "${RED}✗ Rust is not installed!${NC}"
fi

# Check Database Connection
echo -e "\n${GREEN}Checking database connection...${NC}"
echo -e "${YELLOW}Container Status:${NC}"
docker-compose -f docker/docker-compose.yml ps

# Check Database Logs
echo -e "\n${GREEN}Recent database logs:${NC}"
docker-compose -f docker/docker-compose.yml logs --tail=10 timescaledb

echo -e "\n${GREEN}Troubleshooting complete!${NC}"
echo -e "${YELLOW}
If you're experiencing issues:
1. Ensure Docker is running
2. Try resetting the database: ./scripts/reset-db.sh
3. Check the logs: docker-compose -f docker/docker-compose.yml logs
4. Ensure all permissions are set: chmod +x scripts/*.sh${NC}"

# Check if running as root with Docker
if [ "$EUID" -ne 0 ] && ! groups | grep -q "docker"; then
    echo -e "\n${YELLOW}Note: If Docker commands require sudo, run:${NC}"
    echo "sudo usermod -aG docker $USER"
    echo "Then log out and log back in."
fi