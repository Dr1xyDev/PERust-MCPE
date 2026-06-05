#!/bin/bash
#
# PeRust Server Startup Script (Linux/Mac)
#
# This script checks for the PeRust server binary and runs it
# with the proper settings. It uses exec to replace the shell
# process so signals are forwarded correctly.
#

set -e

# --- Configuration ---
SERVER_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARY_NAME="perust"
# ---------------------

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
RESET='\033[0m'

echo -e "${CYAN}PeRust Server Launcher${RESET}"

# Check if the binary exists
if [ -f "${SERVER_DIR}/${BINARY_NAME}" ]; then
    BINARY="${SERVER_DIR}/${BINARY_NAME}"
elif [ -f "${SERVER_DIR}/target/debug/${BINARY_NAME}" ]; then
    BINARY="${SERVER_DIR}/target/debug/${BINARY_NAME}"
elif [ -f "${SERVER_DIR}/target/release/${BINARY_NAME}" ]; then
    BINARY="${SERVER_DIR}/target/release/${BINARY_NAME}"
else
    echo -e "${YELLOW}Binary not found. Building PeRust server...${RESET}"
    cd "${SERVER_DIR}/.."

    # Build the server
    if command -v cargo &> /dev/null; then
        cargo build --bin perust 2>&1 || {
            echo -e "${RED}Failed to build PeRust server!${RESET}"
            exit 1
        }
        BINARY="${SERVER_DIR}/../target/debug/${BINARY_NAME}"
    else
        echo -e "${RED}Cargo not found! Please install Rust: https://rustup.rs/${RESET}"
        exit 1
    fi
fi

echo -e "${GREEN}Using binary: ${BINARY}${RESET}"

# Create data directory if it doesn't exist
mkdir -p "${SERVER_DIR}/data/worlds"
mkdir -p "${SERVER_DIR}/data/plugins"

# Run the server with exec to replace the shell process
# This ensures signals (SIGINT, SIGTERM) are forwarded to the server
echo -e "${CYAN}Starting PeRust server...${RESET}"
echo ""

exec "${BINARY}" "$@"
