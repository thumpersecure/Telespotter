#!/bin/bash
#
# TeleSpotter Rust Build Script
# Builds optimized binary and optionally installs to system
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
cat << "EOF"
████████╗███████╗██╗     ███████╗███████╗██████╗  ██████╗ ████████╗████████╗███████╗██████╗ 
╚══██╔══╝██╔════╝██║     ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚══██╔══╝╚══██╔══╝██╔════╝██╔══██╗
   ██║   █████╗  ██║     █████╗  ███████╗██████╔╝██║   ██║   ██║      ██║   █████╗  ██████╔╝
   ██║   ██╔══╝  ██║     ██╔══╝  ╚════██║██╔═══╝ ██║   ██║   ██║      ██║   ██╔══╝  ██╔══██╗
   ██║   ███████╗███████╗███████╗███████║██║     ╚██████╔╝   ██║      ██║   ███████╗██║  ██║
   ╚═╝   ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝      ╚═════╝    ╚═╝      ╚═╝   ╚══════╝╚═╝  ╚═╝
                                                                  Rust Build Script
EOF
echo -e "${NC}\n"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   TeleSpotter Rust Builder${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed.${NC}"
    echo -e "${YELLOW}Please install Rust from https://rustup.rs/${NC}"
    echo -e "${CYAN}Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo -e "${GREEN}✓ Found Rust ${RUST_VERSION}${NC}\n"

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed.${NC}"
    exit 1
fi

echo -e "${CYAN}Building TeleSpotter in release mode (optimized)...${NC}"
echo -e "${YELLOW}This may take a few minutes on first build...${NC}\n"

# Build release version
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}========================================${NC}"
    echo -e "${GREEN}   Build Successful! 🎉${NC}"
    echo -e "${GREEN}========================================${NC}\n"
    
    # Get binary size
    BINARY_PATH="target/release/telespotter"
    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        echo -e "${CYAN}Binary location:${NC} $BINARY_PATH"
        echo -e "${CYAN}Binary size:${NC} $BINARY_SIZE\n"
        
        # Test the binary
        echo -e "${CYAN}Testing binary...${NC}"
        ./"$BINARY_PATH" --version
        echo ""
        
        # Ask if user wants to install
        echo -e "${BLUE}Installation Options:${NC}"
        echo -e "  1) Copy to /usr/local/bin (system-wide, requires sudo)"
        echo -e "  2) Copy to ~/.local/bin (user-only)"
        echo -e "  3) Keep in target/release/ (no installation)"
        echo ""
        read -p "Choose option (1/2/3): " -n 1 -r
        echo ""
        
        if [[ $REPLY =~ ^[1]$ ]]; then
            echo -e "${CYAN}Installing to /usr/local/bin...${NC}"
            sudo cp "$BINARY_PATH" /usr/local/bin/telespotter
            sudo chmod +x /usr/local/bin/telespotter
            echo -e "${GREEN}✓ Installed to /usr/local/bin/telespotter${NC}"
            echo -e "${CYAN}You can now run: ${YELLOW}telespotter${NC}\n"
        elif [[ $REPLY =~ ^[2]$ ]]; then
            mkdir -p ~/.local/bin
            cp "$BINARY_PATH" ~/.local/bin/telespotter
            chmod +x ~/.local/bin/telespotter
            echo -e "${GREEN}✓ Installed to ~/.local/bin/telespotter${NC}"
            echo -e "${YELLOW}Note: Make sure ~/.local/bin is in your PATH${NC}"
            echo -e "${CYAN}Add to ~/.bashrc or ~/.zshrc:${NC}"
            echo -e "${YELLOW}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}\n"
        else
            echo -e "${CYAN}Binary kept at: ${YELLOW}$BINARY_PATH${NC}"
            echo -e "${CYAN}Run with: ${YELLOW}./$BINARY_PATH${NC}\n"
        fi
        
        # Usage examples
        echo -e "${BLUE}Quick Start:${NC}"
        echo -e "${YELLOW}  telespotter 5555551212${NC}"
        echo -e "${YELLOW}  telespotter --debug \"(555) 555-1212\"${NC}"
        echo -e "${YELLOW}  telespotter -s -n 10 5555551212${NC}\n"
        
    else
        echo -e "${RED}Error: Binary not found at $BINARY_PATH${NC}"
        exit 1
    fi
else
    echo -e "\n${RED}========================================${NC}"
    echo -e "${RED}   Build Failed ❌${NC}"
    echo -e "${RED}========================================${NC}\n"
    exit 1
fi

# Offer to run tests
echo ""
read -p "Would you like to run the test suite? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${CYAN}Running tests...${NC}\n"
    cargo test
fi

echo -e "\n${GREEN}Setup complete!${NC}\n"
