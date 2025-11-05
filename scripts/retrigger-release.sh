#!/usr/bin/env bash
# Script to delete and recreate a release tag to trigger GitHub Actions

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

VERSION=$1

# Clean version (remove 'v' prefix if present)
CLEAN_VERSION="${VERSION#v}"

# Validate version format (x.y.z)
if ! [[ $CLEAN_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Version must be in format x.y.z (e.g., 0.1.5)${NC}"
    exit 1
fi

TAG_NAME="v$CLEAN_VERSION"

echo -e "${CYAN}Re-triggering release for $TAG_NAME...${NC}"
echo ""

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Check if there are uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${YELLOW}Warning: You have uncommitted changes:${NC}"
    git status --porcelain
    echo ""
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Aborted.${NC}"
        exit 0
    fi
fi

echo -e "${CYAN}Step 1: Deleting local tag...${NC}"
if git tag -d "$TAG_NAME" 2>/dev/null; then
    echo -e "${GREEN}✓ Deleted local tag${NC}"
else
    echo -e "${YELLOW}Note: Local tag didn't exist (this is okay)${NC}"
fi

echo ""
echo -e "${CYAN}Step 2: Deleting remote tag...${NC}"
if git push origin --delete "$TAG_NAME" 2>/dev/null; then
    echo -e "${GREEN}✓ Deleted remote tag${NC}"
else
    echo -e "${YELLOW}Note: Remote tag may not exist (this is okay)${NC}"
fi

echo ""
echo -e "${CYAN}Step 3: Creating new tag...${NC}"
if git tag "$TAG_NAME"; then
    echo -e "${GREEN}✓ Created local tag${NC}"
else
    echo -e "${RED}✗ Failed to create tag${NC}"
    exit 1
fi

echo ""
echo -e "${CYAN}Step 4: Pushing tag to trigger release...${NC}"
if git push origin "$TAG_NAME"; then
    echo -e "${GREEN}✓ Pushed tag to origin${NC}"
else
    echo -e "${RED}✗ Failed to push tag${NC}"
    exit 1
fi

# Get repository URL for links
REPO_URL=$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')

echo ""
echo -e "${GREEN}Release re-triggered successfully! 🎉${NC}"
echo ""
echo -e "${CYAN}Next steps:${NC}"
echo "  1. Check GitHub Actions: https://github.com/$REPO_URL/actions"
echo "  2. Wait for builds to complete (~10-15 minutes)"
echo "  3. Check release page: https://github.com/$REPO_URL/releases/tag/$TAG_NAME"

