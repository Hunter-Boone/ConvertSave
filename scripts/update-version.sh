#!/usr/bin/env bash
# Script to update version across all project files

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version argument required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.5"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (x.y.z)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Version must be in format x.y.z (e.g., 0.1.5)${NC}"
    exit 1
fi

# Get script directory and root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

PACKAGE_JSON="$ROOT_DIR/package.json"
CARGO_TOML="$ROOT_DIR/src-tauri/Cargo.toml"
TAURI_CONF="$ROOT_DIR/src-tauri/tauri.conf.json"

echo -e "${CYAN}Updating version to $NEW_VERSION...${NC}"
echo ""

# Function to update version in a file
update_version() {
    local file=$1
    local pattern=$2
    local replacement=$3
    local filename=$(basename "$file")
    
    if [ -f "$file" ]; then
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS
            sed -i '' -E "$pattern" "$file"
        else
            # Linux
            sed -i -E "$pattern" "$file"
        fi
        echo -e "${GREEN}✓ Updated: $filename${NC}"
        return 0
    else
        echo -e "${RED}✗ File not found: $file${NC}"
        return 1
    fi
}

# Update package.json
update_version "$PACKAGE_JSON" \
    's/("version"[[:space:]]*:[[:space:]]*)"[0-9]+\.[0-9]+\.[0-9]+"/\1"'"$NEW_VERSION"'"/'

# Update Cargo.toml
update_version "$CARGO_TOML" \
    's/(version[[:space:]]*=[[:space:]]*)"[0-9]+\.[0-9]+\.[0-9]+"/\1"'"$NEW_VERSION"'"/'

# Update tauri.conf.json
update_version "$TAURI_CONF" \
    's/("version"[[:space:]]*:[[:space:]]*)"[0-9]+\.[0-9]+\.[0-9]+"/\1"'"$NEW_VERSION"'"/'

echo ""

# Update Cargo.lock
echo -e "${CYAN}Updating Cargo.lock...${NC}"
cd "$ROOT_DIR/src-tauri"
cargo update -p convertsave --quiet
echo -e "${GREEN}✓ Updated: Cargo.lock${NC}"

echo ""
echo -e "${GREEN}Version update complete! 🎉${NC}"
echo ""
echo -e "${CYAN}Next steps:${NC}"
echo "  1. Review changes: git diff"
echo "  2. Commit changes: git add . && git commit -m 'chore: bump version to v$NEW_VERSION'"
echo "  3. Create tag: git tag v$NEW_VERSION"
echo "  4. Push: git push && git push --tags"

