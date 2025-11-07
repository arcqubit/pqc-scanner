#!/bin/bash

# Script to populate an existing GitHub Project Board with issues
# Usage: ./populate-project-board.sh <PROJECT_NUMBER>

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <PROJECT_NUMBER>"
    echo ""
    echo "Example: $0 1"
    echo ""
    echo "To find your project number:"
    echo "  1. Create the project board at: https://github.com/orgs/arcqubit/projects"
    echo "  2. The URL will be: https://github.com/orgs/arcqubit/projects/NUMBER"
    echo "  3. Use that NUMBER as the argument"
    echo ""
    exit 1
fi

PROJECT_NUMBER=$1
REPO_OWNER="arcqubit"
REPO_NAME="pqc-scanner"

echo "🚀 Populating GitHub Project Board #$PROJECT_NUMBER..."
echo ""

# Check authentication
echo "Checking authentication..."
if ! gh auth status > /dev/null 2>&1; then
    echo "❌ Not authenticated with GitHub CLI"
    echo "Run: gh auth login"
    exit 1
fi

echo "✅ Authenticated"
echo ""

# Get project ID from project number
echo "Getting project ID for project #$PROJECT_NUMBER..."
PROJECT_DATA=$(gh api graphql -f query='
  query {
    organization(login: "'"$REPO_OWNER"'") {
      projectV2(number: '"$PROJECT_NUMBER"') {
        id
        title
        url
      }
    }
  }
' 2>/dev/null || gh api graphql -f query='
  query {
    user(login: "'"$REPO_OWNER"'") {
      projectV2(number: '"$PROJECT_NUMBER"') {
        id
        title
        url
      }
    }
  }
' 2>/dev/null)

PROJECT_ID=$(echo "$PROJECT_DATA" | jq -r '.data.organization.projectV2.id // .data.user.projectV2.id')
PROJECT_TITLE=$(echo "$PROJECT_DATA" | jq -r '.data.organization.projectV2.title // .data.user.projectV2.title')
PROJECT_URL=$(echo "$PROJECT_DATA" | jq -r '.data.organization.projectV2.url // .data.user.projectV2.url')

if [ "$PROJECT_ID" = "null" ] || [ -z "$PROJECT_ID" ]; then
    echo "❌ Project #$PROJECT_NUMBER not found"
    echo ""
    echo "Please ensure:"
    echo "  1. The project exists"
    echo "  2. You have access to it"
    echo "  3. You're using the correct project number"
    echo ""
    exit 1
fi

echo "✅ Found project: $PROJECT_TITLE"
echo "   ID: $PROJECT_ID"
echo "   URL: $PROJECT_URL"
echo ""

# Add issues to project
echo "Adding issues #1-21 to project..."
echo ""

ISSUES_ADDED=0
ISSUES_FAILED=0
ISSUES_ALREADY_ADDED=0

for ISSUE_NUM in {1..21}; do
  printf "  Issue #%-2d ... " "$ISSUE_NUM"

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

  # Check if already in project (to avoid duplicates)
  CHECK_RESULT=$(gh api graphql -f query='
    query {
      node(id: "'"$ISSUE_ID"'") {
        ... on Issue {
          projectItems(first: 10) {
            nodes {
              project {
                id
              }
            }
          }
        }
      }
    }
  ' 2>/dev/null)

  ALREADY_IN_PROJECT=$(echo "$CHECK_RESULT" | jq -r '.data.node.projectItems.nodes[] | select(.project.id == "'"$PROJECT_ID"'") | .project.id')

  if [ ! -z "$ALREADY_IN_PROJECT" ]; then
    echo "⚠️  Already in project"
    ((ISSUES_ALREADY_ADDED++))
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
    echo "❌ Failed to add"
    ((ISSUES_FAILED++))
  else
    echo "✅ Added"
    ((ISSUES_ADDED++))
  fi

  # Rate limiting: sleep briefly between requests
  sleep 0.5
done

echo ""
echo "======================================================================"
echo "✅ Project Board Population Complete!"
echo "======================================================================"
echo ""
echo "Summary:"
echo "  Project: $PROJECT_TITLE (#$PROJECT_NUMBER)"
echo "  URL: $PROJECT_URL"
echo ""
echo "Results:"
echo "  ✅ Issues Added: $ISSUES_ADDED"
echo "  ⚠️  Already in Project: $ISSUES_ALREADY_ADDED"
echo "  ❌ Failed: $ISSUES_FAILED"
echo "  📊 Total: $((ISSUES_ADDED + ISSUES_ALREADY_ADDED)) / 21"
echo ""

if [ $ISSUES_ADDED -gt 0 ]; then
    echo "Next Steps:"
    echo "1. Visit project board: $PROJECT_URL"
    echo "2. Add custom fields (Status, Priority, Quarter, Story Points)"
    echo "3. Set Status for all issues to 'Backlog'"
    echo "4. Group by Quarter to see roadmap"
    echo "5. Begin sprint planning for Q1 2026"
    echo ""
fi

if [ $ISSUES_FAILED -gt 0 ]; then
    echo "⚠️  Some issues failed to add. This might be due to:"
    echo "   - Missing 'project' scopes (run: gh auth refresh -h github.com -s project)"
    echo "   - Network issues (try again)"
    echo "   - Project permissions"
    echo ""
fi

echo "📖 See docs/implementation/CREATE_PROJECT_BOARD.md for more details"
echo ""
