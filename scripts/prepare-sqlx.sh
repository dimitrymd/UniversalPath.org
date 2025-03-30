#!/bin/bash

# Script to prepare SQLx for offline mode

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting SQLx preparation...${NC}"

# Check if DATABASE_URL is already set
if [ -z "$DATABASE_URL" ]; then
    # Try to load from .env file
    if [ -f .env ]; then
        echo -e "Loading DATABASE_URL from .env file..."
        export $(grep -v '^#' .env | grep DATABASE_URL | xargs)
    fi

    # If still not set, use default
    if [ -z "$DATABASE_URL" ]; then
        export DATABASE_URL="mysql://root@localhost:3306/catman"
        echo -e "${YELLOW}DATABASE_URL not found in environment or .env file.${NC}"
        echo -e "${YELLOW}Using default: ${DATABASE_URL}${NC}"
    fi
fi

echo -e "Using DATABASE_URL: ${DATABASE_URL}"

# Install sqlx-cli if not already installed
if ! command -v sqlx &> /dev/null; then
    echo -e "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features mysql
fi

# Run sqlx prepare
echo -e "Running sqlx prepare..."
cargo sqlx prepare --database-url "${DATABASE_URL}"

echo -e "${GREEN}SQLx preparation completed successfully!${NC}"
echo -e "You can now build the project without a direct database connection."