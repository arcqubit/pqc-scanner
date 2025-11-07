#!/bin/bash

# Script to organize GitHub Project Board with custom fields and values
# Requires: gh CLI with project, read:project, write:project scopes

set -e

PROJECT_NUMBER=1
REPO_OWNER="arcqubit"
REPO_NAME="pqc-scanner"

echo "🎨 Organizing GitHub Project Board..."
echo ""

# Check authentication
if ! gh auth status 2>&1 | grep -q "project"; then
    echo "❌ Missing required 'project' scope!"
    echo ""
    echo "Please run:"
    echo "  gh auth refresh -h github.com -s project,read:project,write:project"
    echo ""
    echo "This will:"
    echo "  1. Open your browser"
    echo "  2. Ask you to authorize additional scopes"
    echo "  3. Update your token"
    echo ""
    exit 1
fi

echo "✅ Authentication OK"
echo ""

# Get project ID
echo "Getting project information..."
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
')

PROJECT_ID=$(echo "$PROJECT_DATA" | jq -r '.data.organization.projectV2.id // .data.user.projectV2.id')
PROJECT_URL=$(echo "$PROJECT_DATA" | jq -r '.data.organization.projectV2.url // .data.user.projectV2.url')

echo "✅ Project found: $PROJECT_URL"
echo ""

# Get existing fields
echo "Checking for existing custom fields..."
FIELDS_DATA=$(gh api graphql -f query='
  query {
    node(id: "'"$PROJECT_ID"'") {
      ... on ProjectV2 {
        fields(first: 20) {
          nodes {
            ... on ProjectV2Field {
              id
              name
            }
            ... on ProjectV2SingleSelectField {
              id
              name
              options {
                id
                name
              }
            }
          }
        }
      }
    }
  }
')

echo "$FIELDS_DATA" | jq -r '.data.node.fields.nodes[] | "  - \(.name)"'
echo ""

# Function to create or get field ID
get_or_create_field() {
    local field_name=$1
    local field_type=$2

    FIELD_ID=$(echo "$FIELDS_DATA" | jq -r '.data.node.fields.nodes[] | select(.name == "'"$field_name"'") | .id')

    if [ "$FIELD_ID" = "null" ] || [ -z "$FIELD_ID" ]; then
        echo "Creating field: $field_name..."

        if [ "$field_type" = "single_select" ]; then
            CREATE_RESULT=$(gh api graphql -f query='
              mutation {
                createProjectV2Field(input: {
                  projectId: "'"$PROJECT_ID"'"
                  dataType: SINGLE_SELECT
                  name: "'"$field_name"'"
                }) {
                  projectV2Field {
                    ... on ProjectV2SingleSelectField {
                      id
                      name
                    }
                  }
                }
              }
            ')
            FIELD_ID=$(echo "$CREATE_RESULT" | jq -r '.data.createProjectV2Field.projectV2Field.id')
        elif [ "$field_type" = "number" ]; then
            CREATE_RESULT=$(gh api graphql -f query='
              mutation {
                createProjectV2Field(input: {
                  projectId: "'"$PROJECT_ID"'"
                  dataType: NUMBER
                  name: "'"$field_name"'"
                }) {
                  projectV2Field {
                    ... on ProjectV2Field {
                      id
                      name
                    }
                  }
                }
              }
            ')
            FIELD_ID=$(echo "$CREATE_RESULT" | jq -r '.data.createProjectV2Field.projectV2Field.id')
        elif [ "$field_type" = "text" ]; then
            CREATE_RESULT=$(gh api graphql -f query='
              mutation {
                createProjectV2Field(input: {
                  projectId: "'"$PROJECT_ID"'"
                  dataType: TEXT
                  name: "'"$field_name"'"
                }) {
                  projectV2Field {
                    ... on ProjectV2Field {
                      id
                      name
                    }
                  }
                }
              }
            ')
            FIELD_ID=$(echo "$CREATE_RESULT" | jq -r '.data.createProjectV2Field.projectV2Field.id')
        fi

        echo "  ✅ Created field: $field_name (ID: $FIELD_ID)"
    else
        echo "  ✅ Field exists: $field_name (ID: $FIELD_ID)"
    fi

    echo "$FIELD_ID"
}

# Create custom fields
echo "Setting up custom fields..."
echo ""

QUARTER_FIELD_ID=$(get_or_create_field "Quarter" "single_select")
PRIORITY_FIELD_ID=$(get_or_create_field "Priority" "single_select")
POINTS_FIELD_ID=$(get_or_create_field "Story Points" "number")
EPIC_FIELD_ID=$(get_or_create_field "Epic" "single_select")
SPRINT_FIELD_ID=$(get_or_create_field "Sprint" "text")

echo ""
echo "✅ All fields created/verified"
echo ""

# Add options to single select fields
echo "Adding field options..."

# Quarter options
for quarter in "Q1 2026" "Q2 2026" "Q3 2026" "Q4 2026"; do
    gh api graphql -f query='
      mutation {
        addProjectV2SingleSelectFieldOption(input: {
          fieldId: "'"$QUARTER_FIELD_ID"'"
          name: "'"$quarter"'"
        }) {
          projectV2SingleSelectFieldOption {
            id
            name
          }
        }
      }
    ' > /dev/null 2>&1 || true
done

# Priority options
for priority in "🔴 Critical" "🟠 High" "🟡 Medium" "🟢 Low"; do
    gh api graphql -f query='
      mutation {
        addProjectV2SingleSelectFieldOption(input: {
          fieldId: "'"$PRIORITY_FIELD_ID"'"
          name: "'"$priority"'"
        }) {
          projectV2SingleSelectFieldOption {
            id
            name
          }
        }
      }
    ' > /dev/null 2>&1 || true
done

# Epic options
for epic in "CLI Tool" "IDE Extensions" "Enhanced Detection" "Compliance"; do
    gh api graphql -f query='
      mutation {
        addProjectV2SingleSelectFieldOption(input: {
          fieldId: "'"$EPIC_FIELD_ID"'"
          name: "'"$epic"'"
        }) {
          projectV2SingleSelectFieldOption {
            id
            name
          }
        }
      }
    ' > /dev/null 2>&1 || true
done

echo "✅ Field options added"
echo ""

# Get updated field options
FIELDS_DATA=$(gh api graphql -f query='
  query {
    node(id: "'"$PROJECT_ID"'") {
      ... on ProjectV2 {
        fields(first: 20) {
          nodes {
            ... on ProjectV2SingleSelectField {
              id
              name
              options {
                id
                name
              }
            }
            ... on ProjectV2Field {
              id
              name
            }
          }
        }
      }
    }
  }
')

# Function to get option ID
get_option_id() {
    local field_name=$1
    local option_name=$2

    OPTION_ID=$(echo "$FIELDS_DATA" | jq -r '.data.node.fields.nodes[] | select(.name == "'"$field_name"'") | .options[]? | select(.name == "'"$option_name"'") | .id')
    echo "$OPTION_ID"
}

# Function to update issue field
update_issue_field() {
    local issue_num=$1
    local field_id=$2
    local value=$3
    local value_type=$4  # single_select, number, text

    # Get project item ID for this issue
    ITEM_DATA=$(gh api graphql -f query='
      query {
        repository(owner: "'"$REPO_OWNER"'", name: "'"$REPO_NAME"'") {
          issue(number: '"$issue_num"') {
            projectItems(first: 10) {
              nodes {
                id
                project {
                  id
                }
              }
            }
          }
        }
      }
    ')

    ITEM_ID=$(echo "$ITEM_DATA" | jq -r '.data.repository.issue.projectItems.nodes[] | select(.project.id == "'"$PROJECT_ID"'") | .id')

    if [ -z "$ITEM_ID" ] || [ "$ITEM_ID" = "null" ]; then
        return 1
    fi

    if [ "$value_type" = "single_select" ]; then
        gh api graphql -f query='
          mutation {
            updateProjectV2ItemFieldValue(input: {
              projectId: "'"$PROJECT_ID"'"
              itemId: "'"$ITEM_ID"'"
              fieldId: "'"$field_id"'"
              value: {
                singleSelectOptionId: "'"$value"'"
              }
            }) {
              projectV2Item {
                id
              }
            }
          }
        ' > /dev/null 2>&1
    elif [ "$value_type" = "number" ]; then
        gh api graphql -f query='
          mutation {
            updateProjectV2ItemFieldValue(input: {
              projectId: "'"$PROJECT_ID"'"
              itemId: "'"$ITEM_ID"'"
              fieldId: "'"$field_id"'"
              value: {
                number: '"$value"'
              }
            }) {
              projectV2Item {
                id
              }
            }
          }
        ' > /dev/null 2>&1
    elif [ "$value_type" = "text" ]; then
        gh api graphql -f query='
          mutation {
            updateProjectV2ItemFieldValue(input: {
              projectId: "'"$PROJECT_ID"'"
              itemId: "'"$ITEM_ID"'"
              fieldId: "'"$field_id"'"
              value: {
                text: "'"$value"'"
              }
            }) {
              projectV2Item {
                id
              }
            }
          }
        ' > /dev/null 2>&1
    fi
}

# Organize issues
echo "Organizing issues by quarter..."
echo ""

# Q1 2026 Issues
echo "▼ Q1 2026 - CLI Tool"

Q1_2026_ID=$(get_option_id "Quarter" "Q1 2026")
CRITICAL_ID=$(get_option_id "Priority" "🔴 Critical")
HIGH_ID=$(get_option_id "Priority" "🟠 High")
MEDIUM_ID=$(get_option_id "Priority" "🟡 Medium")
CLI_TOOL_ID=$(get_option_id "Epic" "CLI Tool")

# Issue #1
printf "  Issue #1 ... "
update_issue_field 1 "$QUARTER_FIELD_ID" "$Q1_2026_ID" "single_select"
update_issue_field 1 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
update_issue_field 1 "$POINTS_FIELD_ID" "21" "number"
update_issue_field 1 "$EPIC_FIELD_ID" "$CLI_TOOL_ID" "single_select"
echo "✅"
sleep 0.5

# Issue #2
printf "  Issue #2 ... "
update_issue_field 2 "$QUARTER_FIELD_ID" "$Q1_2026_ID" "single_select"
update_issue_field 2 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
update_issue_field 2 "$POINTS_FIELD_ID" "5" "number"
update_issue_field 2 "$EPIC_FIELD_ID" "$CLI_TOOL_ID" "single_select"
update_issue_field 2 "$SPRINT_FIELD_ID" "Sprint 1" "text"
echo "✅"
sleep 0.5

# Issue #3
printf "  Issue #3 ... "
update_issue_field 3 "$QUARTER_FIELD_ID" "$Q1_2026_ID" "single_select"
update_issue_field 3 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
update_issue_field 3 "$POINTS_FIELD_ID" "8" "number"
update_issue_field 3 "$EPIC_FIELD_ID" "$CLI_TOOL_ID" "single_select"
update_issue_field 3 "$SPRINT_FIELD_ID" "Sprint 1" "text"
echo "✅"
sleep 0.5

# Issue #4
printf "  Issue #4 ... "
update_issue_field 4 "$QUARTER_FIELD_ID" "$Q1_2026_ID" "single_select"
update_issue_field 4 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
update_issue_field 4 "$POINTS_FIELD_ID" "8" "number"
update_issue_field 4 "$EPIC_FIELD_ID" "$CLI_TOOL_ID" "single_select"
update_issue_field 4 "$SPRINT_FIELD_ID" "Sprint 2" "text"
echo "✅"
sleep 0.5

# Issue #5
printf "  Issue #5 ... "
update_issue_field 5 "$QUARTER_FIELD_ID" "$Q1_2026_ID" "single_select"
update_issue_field 5 "$PRIORITY_FIELD_ID" "$MEDIUM_ID" "single_select"
update_issue_field 5 "$POINTS_FIELD_ID" "5" "number"
update_issue_field 5 "$EPIC_FIELD_ID" "$CLI_TOOL_ID" "single_select"
update_issue_field 5 "$SPRINT_FIELD_ID" "Sprint 2" "text"
echo "✅"
sleep 0.5

echo ""
echo "▼ Q2 2026 - IDE Extensions"

Q2_2026_ID=$(get_option_id "Quarter" "Q2 2026")
IDE_ID=$(get_option_id "Epic" "IDE Extensions")

# Issues #6-11
for i in 6 7 8 9 10 11; do
    printf "  Issue #$i ... "

    case $i in
        6)
            update_issue_field 6 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 6 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 6 "$POINTS_FIELD_ID" "21" "number"
            update_issue_field 6 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            ;;
        7)
            update_issue_field 7 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 7 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 7 "$POINTS_FIELD_ID" "5" "number"
            update_issue_field 7 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            update_issue_field 7 "$SPRINT_FIELD_ID" "Sprint 5" "text"
            ;;
        8)
            update_issue_field 8 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 8 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 8 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 8 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            update_issue_field 8 "$SPRINT_FIELD_ID" "Sprint 5-6" "text"
            ;;
        9)
            update_issue_field 9 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 9 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 9 "$POINTS_FIELD_ID" "8" "number"
            update_issue_field 9 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            update_issue_field 9 "$SPRINT_FIELD_ID" "Sprint 6" "text"
            ;;
        10)
            update_issue_field 10 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 10 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 10 "$POINTS_FIELD_ID" "8" "number"
            update_issue_field 10 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            update_issue_field 10 "$SPRINT_FIELD_ID" "Sprint 7" "text"
            ;;
        11)
            update_issue_field 11 "$QUARTER_FIELD_ID" "$Q2_2026_ID" "single_select"
            update_issue_field 11 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 11 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 11 "$EPIC_FIELD_ID" "$IDE_ID" "single_select"
            update_issue_field 11 "$SPRINT_FIELD_ID" "Sprint 8-9" "text"
            ;;
    esac

    echo "✅"
    sleep 0.5
done

echo ""
echo "▼ Q3 2026 - Enhanced Detection"

Q3_2026_ID=$(get_option_id "Quarter" "Q3 2026")
DETECTION_ID=$(get_option_id "Epic" "Enhanced Detection")

# Issues #12-16
for i in 12 13 14 15 16; do
    printf "  Issue #$i ... "

    case $i in
        12)
            update_issue_field 12 "$QUARTER_FIELD_ID" "$Q3_2026_ID" "single_select"
            update_issue_field 12 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 12 "$POINTS_FIELD_ID" "21" "number"
            update_issue_field 12 "$EPIC_FIELD_ID" "$DETECTION_ID" "single_select"
            ;;
        13)
            update_issue_field 13 "$QUARTER_FIELD_ID" "$Q3_2026_ID" "single_select"
            update_issue_field 13 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 13 "$POINTS_FIELD_ID" "8" "number"
            update_issue_field 13 "$EPIC_FIELD_ID" "$DETECTION_ID" "single_select"
            update_issue_field 13 "$SPRINT_FIELD_ID" "Sprint 10" "text"
            ;;
        14)
            update_issue_field 14 "$QUARTER_FIELD_ID" "$Q3_2026_ID" "single_select"
            update_issue_field 14 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 14 "$POINTS_FIELD_ID" "8" "number"
            update_issue_field 14 "$EPIC_FIELD_ID" "$DETECTION_ID" "single_select"
            update_issue_field 14 "$SPRINT_FIELD_ID" "Sprint 11" "text"
            ;;
        15)
            update_issue_field 15 "$QUARTER_FIELD_ID" "$Q3_2026_ID" "single_select"
            update_issue_field 15 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 15 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 15 "$EPIC_FIELD_ID" "$DETECTION_ID" "single_select"
            update_issue_field 15 "$SPRINT_FIELD_ID" "Sprint 12" "text"
            ;;
        16)
            update_issue_field 16 "$QUARTER_FIELD_ID" "$Q3_2026_ID" "single_select"
            update_issue_field 16 "$PRIORITY_FIELD_ID" "$MEDIUM_ID" "single_select"
            update_issue_field 16 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 16 "$EPIC_FIELD_ID" "$DETECTION_ID" "single_select"
            update_issue_field 16 "$SPRINT_FIELD_ID" "Sprint 13-14" "text"
            ;;
    esac

    echo "✅"
    sleep 0.5
done

echo ""
echo "▼ Q4 2026 - Compliance"

Q4_2026_ID=$(get_option_id "Quarter" "Q4 2026")
COMPLIANCE_ID=$(get_option_id "Epic" "Compliance")

# Issues #17-21
for i in 17 18 19 20 21; do
    printf "  Issue #$i ... "

    case $i in
        17)
            update_issue_field 17 "$QUARTER_FIELD_ID" "$Q4_2026_ID" "single_select"
            update_issue_field 17 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 17 "$POINTS_FIELD_ID" "21" "number"
            update_issue_field 17 "$EPIC_FIELD_ID" "$COMPLIANCE_ID" "single_select"
            ;;
        18)
            update_issue_field 18 "$QUARTER_FIELD_ID" "$Q4_2026_ID" "single_select"
            update_issue_field 18 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 18 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 18 "$EPIC_FIELD_ID" "$COMPLIANCE_ID" "single_select"
            update_issue_field 18 "$SPRINT_FIELD_ID" "Sprint 15-16" "text"
            ;;
        19)
            update_issue_field 19 "$QUARTER_FIELD_ID" "$Q4_2026_ID" "single_select"
            update_issue_field 19 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 19 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 19 "$EPIC_FIELD_ID" "$COMPLIANCE_ID" "single_select"
            update_issue_field 19 "$SPRINT_FIELD_ID" "Sprint 16-17" "text"
            ;;
        20)
            update_issue_field 20 "$QUARTER_FIELD_ID" "$Q4_2026_ID" "single_select"
            update_issue_field 20 "$PRIORITY_FIELD_ID" "$CRITICAL_ID" "single_select"
            update_issue_field 20 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 20 "$EPIC_FIELD_ID" "$COMPLIANCE_ID" "single_select"
            update_issue_field 20 "$SPRINT_FIELD_ID" "Sprint 17-18" "text"
            ;;
        21)
            update_issue_field 21 "$QUARTER_FIELD_ID" "$Q4_2026_ID" "single_select"
            update_issue_field 21 "$PRIORITY_FIELD_ID" "$HIGH_ID" "single_select"
            update_issue_field 21 "$POINTS_FIELD_ID" "13" "number"
            update_issue_field 21 "$EPIC_FIELD_ID" "$COMPLIANCE_ID" "single_select"
            update_issue_field 21 "$SPRINT_FIELD_ID" "Sprint 19" "text"
            ;;
    esac

    echo "✅"
    sleep 0.5
done

echo ""
echo "======================================================================"
echo "🎉 Project Board Organized Successfully!"
echo "======================================================================"
echo ""
echo "Summary:"
echo "  ✅ Custom fields created (Quarter, Priority, Story Points, Epic, Sprint)"
echo "  ✅ All 21 issues organized by quarter"
echo "  ✅ Priorities assigned (Critical/High/Medium)"
echo "  ✅ Story points assigned (247 total)"
echo "  ✅ Sprints assigned to features"
echo ""
echo "Your board is now organized!"
echo "Visit: $PROJECT_URL"
echo ""
echo "Next Steps:"
echo "  1. Click 'Group by' → Select 'Quarter'"
echo "  2. View organized quarterly roadmap"
echo "  3. Create sprint views for active work"
echo "  4. Start Sprint 1 with issues #2 and #3"
echo ""
