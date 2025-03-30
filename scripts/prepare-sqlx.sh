#!/bin/bash

# Script to prepare SQLx for offline mode

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
export DATABASE_URL=${DATABASE_URL:-"mysql://root@localhost:3306/catman"}

echo -e "${YELLOW}Preparing SQLx for offline mode with database: ${DATABASE_URL}${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Cargo is not installed. Please install Rust and Cargo first.${NC}"
    exit 1
fi

# Install sqlx-cli if not already installed
if ! command -v sqlx &> /dev/null; then
    echo -e "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features mysql
fi

# Run sqlx prepare
echo -e "Running sqlx prepare..."
cargo sqlx prepare --database-url "${DATABASE_URL}"

echo -e "${GREEN}SQLx prepare completed successfully!${NC}"
echo -e "You can now build the project without a direct database connection."