# Order Matching Engine

A high-performance order matching engine built with Rust.

## Prerequisites

### All Operating Systems

- [Docker](https://www.docker.com/get-started)
- [Rust](https://www.rust-lang.org/tools/install)

### Windows Specific

- [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop)
- Enable WSL 2 in Docker Desktop settings
- Git Bash (recommended) or PowerShell

## Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/thenaveensharma/order-matching-engine.git
   cd order-matching-engine
   ```

2. **Copy the environment file:**

   ```bash
   cp .env.example .env
   ```

3. **Update `.env` with your settings:**

   Default values should work for local development.

## Development Commands

### Windows (PowerShell)

- **Start development environment:**

  ```powershell
  .\scripts\dev.ps1
  ```

- **Reset database:**

  ```powershell
  .\scripts\reset-db.ps1
  ```

# Run as administrator

Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

- **View logs:**

  ```powershell
  docker-compose -f docker/docker-compose.yml logs -f
  ```

- **Connect to database:**

  ```powershell
  docker exec -it trading_engine_db psql -U trading_user -d trading_engine
  ```

### Windows (Command Prompt)

- **Start development environment:**

  ```cmd
  scripts\dev.bat
  ```

- **Reset database:**

  ```cmd
  scripts\reset-db.bat
  ```

- **View logs:**

  ```cmd
  docker-compose -f docker/docker-compose.yml logs -f
  ```

- **Connect to database:**

  ```cmd
  docker exec -it trading_engine_db psql -U trading_user -d trading_engine
  ```

### macOS/Linux

1. **Make scripts executable (first-time setup):**

   ```bash
   chmod +x scripts/dev.sh scripts/reset-db.sh
   ```

2. **Start development environment:**

   ```bash
   ./scripts/dev.sh
   ```

3. **Reset database:**

   ```bash
   ./scripts/reset-db.sh
   ```

4. **View logs:**

   ```bash
   docker-compose -f docker/docker-compose.yml logs -f
   ```

5. **Connect to database:**

   ```bash
   docker exec -it trading_engine_db psql -U trading_user -d trading_engine
   ```

## Database Schema

The trading engine uses TimescaleDB (a PostgreSQL extension) with the following schema:

<!-- Include schema details or a link to schema documentation -->

## Troubleshooting

### All Operating Systems

1. **If database connection fails:**

   - **Check if containers are running:**

     ```bash
     docker-compose -f docker/docker-compose.yml ps
     ```

   - **Check container logs:**

     ```bash
     docker-compose -f docker/docker-compose.yml logs timescaledb
     ```

2. **To completely reset the environment:**

   - **Stop all containers:**

     ```bash
     docker-compose -f docker/docker-compose.yml down
     ```

   - **Remove volume:**

     ```bash
     docker volume rm docker_trading_engine_data
     ```

   - **Start fresh:**

     ```bash
     docker-compose -f docker/docker-compose.yml up -d
     ```

### Windows Specific

- **Run the troubleshooting script:**

  ```powershell
  .\scripts\troubleshoot.ps1
  ```

- **Common issues:**
  - Ensure Docker Desktop is running
  - Ensure WSL 2 is enabled
  - Check Windows Defender firewall settings

### macOS Specific

- **If you get permission denied:**

  ```bash
  chmod +x ./scripts/*
  ```

### Linux Specific

- **If Docker commands require `sudo`:**

  ```bash
  sudo usermod -aG docker $USER
  ```

  Log out and log back in for changes to take effect.

## Development Guidelines

1. **Always run tests before committing:**

   ```bash
   cargo test
   ```

2. **Format code:**

   ```bash
   cargo fmt
   ```

3. **Check lints:**

   ```bash
   cargo clippy
   ```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License

Copyright (c) 2025 Naveen Sharma

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
