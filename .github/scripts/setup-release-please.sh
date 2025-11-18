#!/bin/bash
set -e

# Release Please GitHub App Setup Script
# This script helps you create and configure a GitHub App for Release Please automation

echo "========================================="
echo "Release Please GitHub App Setup"
echo "========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: GitHub CLI (gh) is not installed${NC}"
    echo "Install from: https://cli.github.com/"
    exit 1
fi

# Check if logged in
if ! gh auth status &> /dev/null; then
    echo -e "${RED}Error: Not logged into GitHub CLI${NC}"
    echo "Run: gh auth login"
    exit 1
fi

echo -e "${BLUE}Step 1: Creating GitHub App${NC}"
echo "This will open your browser to create a new GitHub App."
echo "Press Enter to continue..."
read

# Get repository info
REPO_OWNER=$(gh repo view --json owner -q .owner.login)
REPO_NAME=$(gh repo view --json name -q .name)
REPO_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}"

echo ""
echo -e "${YELLOW}Repository:${NC} ${REPO_OWNER}/${REPO_NAME}"
echo ""

# Open GitHub App creation page with pre-filled values
echo -e "${BLUE}Opening GitHub App creation page...${NC}"
echo ""
echo "Please configure the app with these settings:"
echo ""
echo -e "${GREEN}GitHub App name:${NC} ${REPO_NAME}-release-please"
echo -e "${GREEN}Homepage URL:${NC} ${REPO_URL}"
echo -e "${GREEN}Webhook:${NC} Uncheck 'Active'"
echo ""
echo -e "${GREEN}Permissions (Repository permissions):${NC}"
echo "  - Contents: Read and write"
echo "  - Pull requests: Read and write"
echo "  - Metadata: Read-only (automatic)"
echo ""
echo -e "${GREEN}Where can this GitHub App be installed?${NC}"
echo "  - Select: Only on this account"
echo ""

# Construct the GitHub App creation URL with query parameters
APP_NAME="${REPO_NAME}-release-please"
HOMEPAGE_URL="${REPO_URL}"
CALLBACK_URL=""
SETUP_URL=""
WEBHOOK_ACTIVE="false"

# URL encode the homepage
ENCODED_HOMEPAGE=$(printf '%s' "$HOMEPAGE_URL" | jq -sRr @uri)
ENCODED_APP_NAME=$(printf '%s' "$APP_NAME" | jq -sRr @uri)

# Open browser
if command -v xdg-open &> /dev/null; then
    xdg-open "https://github.com/settings/apps/new?name=${ENCODED_APP_NAME}&url=${ENCODED_HOMEPAGE}&webhook_active=false" &
elif command -v open &> /dev/null; then
    open "https://github.com/settings/apps/new?name=${ENCODED_APP_NAME}&url=${ENCODED_HOMEPAGE}&webhook_active=false" &
else
    echo "Please manually open: https://github.com/settings/apps/new"
fi

echo ""
echo "After creating the app, press Enter to continue..."
read

echo ""
echo -e "${BLUE}Step 2: Get App ID${NC}"
echo "What is your App ID? (Found at the top of the app settings page)"
read -p "App ID: " APP_ID

if [ -z "$APP_ID" ]; then
    echo -e "${RED}Error: App ID is required${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Step 3: Generate Private Key${NC}"
echo "1. Scroll down to 'Private keys' section"
echo "2. Click 'Generate a private key'"
echo "3. Save the downloaded .pem file"
echo ""
echo "Enter the path to the downloaded .pem file:"
read -p "Path: " PEM_PATH

# Expand ~ to home directory
PEM_PATH="${PEM_PATH/#\~/$HOME}"

if [ ! -f "$PEM_PATH" ]; then
    echo -e "${RED}Error: File not found: $PEM_PATH${NC}"
    exit 1
fi

# Read the private key
PRIVATE_KEY=$(cat "$PEM_PATH")

echo ""
echo -e "${BLUE}Step 4: Install App on Repository${NC}"
echo "1. Click 'Install App' in the left sidebar"
echo "2. Select your account: ${REPO_OWNER}"
echo "3. Choose: 'Only select repositories'"
echo "4. Select: ${REPO_NAME}"
echo "5. Click 'Install'"
echo ""
echo "Press Enter after installation..."
read

echo ""
echo -e "${BLUE}Step 5: Adding Secrets to Repository${NC}"

# Add secrets using gh CLI
echo "Adding RELEASE_PLEASE_APP_ID..."
echo "$APP_ID" | gh secret set RELEASE_PLEASE_APP_ID

echo "Adding RELEASE_PLEASE_APP_PRIVATE_KEY..."
echo "$PRIVATE_KEY" | gh secret set RELEASE_PLEASE_APP_PRIVATE_KEY

echo ""
echo -e "${GREEN}✅ Secrets added successfully!${NC}"

echo ""
echo -e "${BLUE}Step 6: Verify Setup${NC}"
echo "Checking secrets..."
gh secret list | grep RELEASE_PLEASE

echo ""
echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}Setup Complete!${NC}"
echo -e "${GREEN}=========================================${NC}"
echo ""
echo "Release Please is now configured to use GitHub App authentication."
echo ""
echo "Next steps:"
echo "1. Make a commit with conventional format:"
echo "   git commit -m 'feat: test release please automation' --allow-empty"
echo "   git push origin main"
echo ""
echo "2. Watch the workflow run:"
echo "   gh run watch"
echo ""
echo "3. Check for the release PR:"
echo "   gh pr list"
echo ""
echo -e "${YELLOW}Note:${NC} You can safely delete the .pem file now:"
echo "   rm \"$PEM_PATH\""
echo ""
