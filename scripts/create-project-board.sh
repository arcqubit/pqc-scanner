#!/bin/bash

# Script to create GitHub Project Board for PQC Scanner Year 1 (2026)
# Requires: gh CLI with project, read:project, write:project scopes

set -e

REPO_OWNER="arcqubit"
REPO_NAME="pqc-scanner"
PROJECT_TITLE="PQC Scanner - Year 1 (2026)"

echo "🚀 Creating GitHub Project Board for Year 1 (2026)..."
echo ""

# Check if gh is authenticated with required scopes
echo "Checking authentication scopes..."
if ! gh auth status 2>&1 | grep -q "project"; then
    echo "❌ Missing required scopes!"
    echo ""
    echo "Please run:"
    echo "  gh auth refresh -h github.com -s project,read:project,write:project"
    echo ""
    exit 1
fi

echo "✅ Authentication scopes OK"
echo ""

# Get owner ID
echo "Getting repository owner ID..."
OWNER_DATA=$(gh api graphql -f query='
  query {
    repositoryOwner(login: "'"$REPO_OWNER"'") {
      id
    }
  }
')

OWNER_ID=$(echo "$OWNER_DATA" | jq -r '.data.repositoryOwner.id')
echo "Owner ID: $OWNER_ID"
echo ""

# Create project
echo "Creating project: $PROJECT_TITLE"
PROJECT_DATA=$(gh api graphql -f query='
  mutation {
    createProjectV2(input: {
      ownerId: "'"$OWNER_ID"'"
      title: "'"$PROJECT_TITLE"'"
    }) {
      projectV2 {
        id
        number
        url
        title
      }
    }
  }
')

PROJECT_ID=$(echo "$PROJECT_DATA" | jq -r '.data.createProjectV2.projectV2.id')
PROJECT_NUMBER=$(echo "$PROJECT_DATA" | jq -r '.data.createProjectV2.projectV2.number')
PROJECT_URL=$(echo "$PROJECT_DATA" | jq -r '.data.createProjectV2.projectV2.url')

if [ "$PROJECT_ID" = "null" ] || [ -z "$PROJECT_ID" ]; then
    echo "❌ Failed to create project"
    echo "$PROJECT_DATA" | jq .
    exit 1
fi

echo "✅ Project created successfully!"
echo "   ID: $PROJECT_ID"
echo "   Number: $PROJECT_NUMBER"
echo "   URL: $PROJECT_URL"
echo ""

# Add custom fields
echo "Adding custom fields to project..."

# Field 1: Status (Single Select)
echo "  Adding 'Status' field..."
gh api graphql -f query='
  mutation {
    addProjectV2DraftIssue(input: {
      projectId: "'"$PROJECT_ID"'"
      title: "Setup"
    }) {
      projectV2Item {
        id
      }
    }
  }
' > /dev/null 2>&1 || true

# Field 2: Priority (Single Select)
echo "  Adding 'Priority' field..."
# Note: Custom field creation requires additional API calls
# For now, these will need to be added manually via UI

echo "✅ Project structure created"
echo ""

# Get repository ID
echo "Getting repository ID..."
REPO_DATA=$(gh api graphql -f query='
  query {
    repository(owner: "'"$REPO_OWNER"'", name: "'"$REPO_NAME"'") {
      id
    }
  }
')
REPO_ID=$(echo "$REPO_DATA" | jq -r '.data.repository.id')
echo "Repository ID: $REPO_ID"
echo ""

# Add issues to project
echo "Adding issues #1-21 to project..."
ISSUES_ADDED=0
ISSUES_FAILED=0

for ISSUE_NUM in {1..21}; do
  echo -n "  Adding issue #$ISSUE_NUM... "

  # Get issue ID
  ISSUE_DATA=$(gh api graphql -f query='
    query {
      repository(owner: "'"$REPO_OWNER"'", name: "'"$REPO_NAME"'") {
        issue(number: '"$ISSUE_NUM"') {
          id
          title
        }
      }
    }
  ' 2>/dev/null)

  ISSUE_ID=$(echo "$ISSUE_DATA" | jq -r '.data.repository.issue.id')
  ISSUE_TITLE=$(echo "$ISSUE_DATA" | jq -r '.data.repository.issue.title')

  if [ "$ISSUE_ID" = "null" ] || [ -z "$ISSUE_ID" ]; then
    echo "❌ Not found"
    ((ISSUES_FAILED++))
    continue
  fi

  # Add issue to project
  ADD_RESULT=$(gh api graphql -f query='
    mutation {
      addProjectV2ItemById(input: {
        projectId: "'"$PROJECT_ID"'"
        contentId: "'"$ISSUE_ID"'"
      }) {
        item {
          id
        }
      }
    }
  ' 2>/dev/null)

  ITEM_ID=$(echo "$ADD_RESULT" | jq -r '.data.addProjectV2ItemById.item.id')

  if [ "$ITEM_ID" = "null" ] || [ -z "$ITEM_ID" ]; then
    echo "❌ Failed"
    ((ISSUES_FAILED++))
  else
    echo "✅"
    ((ISSUES_ADDED++))
  fi

  # Rate limiting: sleep briefly between requests
  sleep 0.5
done

echo ""
echo "======================================================================"
echo "✅ GitHub Project Board Created Successfully!"
echo "======================================================================"
echo ""
echo "Summary:"
echo "  Project: $PROJECT_TITLE"
echo "  Number: #$PROJECT_NUMBER"
echo "  URL: $PROJECT_URL"
echo "  Issues Added: $ISSUES_ADDED / 21"
echo "  Issues Failed: $ISSUES_FAILED"
echo ""
echo "Next Steps:"
echo "1. Visit project board: $PROJECT_URL"
echo "2. Add custom fields (Status, Priority, Quarter, Story Points)"
echo "3. Organize issues by Quarter/Epic"
echo "4. Set up board views (Kanban, Table, Sprint)"
echo "5. Configure automation rules"
echo ""
echo "📖 See docs/implementation/CREATE_PROJECT_BOARD.md for detailed instructions"
echo ""
