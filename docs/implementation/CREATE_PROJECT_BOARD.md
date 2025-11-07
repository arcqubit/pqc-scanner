# Creating GitHub Project Board for Year 1 (2026)

**Issue**: The GitHub CLI requires additional scopes (`project`, `read:project`, `write:project`) to create and manage project boards programmatically.

---

## Option 1: Create Project Board via GitHub Web UI (Recommended)

### Step 1: Navigate to Projects
1. Visit https://github.com/orgs/arcqubit/projects (if organization)
   - OR https://github.com/users/arcqubit/projects (if personal account)
2. Click **"New project"** button

### Step 2: Create the Project
1. **Title**: `PQC Scanner - Year 1 (2026)`
2. **Template**: Choose **"Table"** or **"Board"** view
3. Click **"Create project"**

### Step 3: Add Custom Fields (Recommended)
Add these custom fields to track progress:

| Field Name | Type | Options |
|------------|------|---------|
| **Status** | Single select | `Backlog`, `Todo`, `In Progress`, `In Review`, `Done` |
| **Priority** | Single select | `Critical`, `High`, `Medium`, `Low` |
| **Quarter** | Single select | `Q1 2026`, `Q2 2026`, `Q3 2026`, `Q4 2026` |
| **Story Points** | Number | 1-21 |
| **Sprint** | Text | Sprint 1, Sprint 2, etc. |
| **Epic** | Single select | `CLI Tool`, `IDE Extensions`, `Detection`, `Compliance` |

### Step 4: Add Issues to Project
1. Click **"Add items"** or press `#`
2. Search for issues by number or title
3. Add all 21 issues (#1-#21)
4. Or use bulk add: Type `arcqubit/pqc-scanner#1-21`

### Step 5: Organize Issues
Group issues by:
- **Quarter**: Q1, Q2, Q3, Q4
- **Epic**: CLI Tool, IDE Extensions, Detection, Compliance
- **Status**: Backlog → Todo → In Progress → Done

---

## Option 2: Create via GitHub CLI (After Granting Scopes)

### Step 1: Grant Required Scopes
```bash
# Login with additional scopes
gh auth login --scopes project,read:project,write:project

# Or refresh existing auth
gh auth refresh -h github.com -s project,read:project,write:project
```

### Step 2: Run the Automated Script
```bash
# Make script executable
chmod +x scripts/create-project-board.sh

# Run the script
./scripts/create-project-board.sh
```

---

## Option 3: Manual Script Execution (After Granting Scopes)

### Create Project Board

```bash
cd /mnt/c/Users/bowma/Projects/_aq_/pqc-scanner

# Get owner ID
OWNER_DATA=$(gh api graphql -f query='
  query {
    repositoryOwner(login: "arcqubit") {
      id
    }
  }
')
OWNER_ID=$(echo "$OWNER_DATA" | jq -r '.data.repositoryOwner.id')

# Create project
PROJECT_DATA=$(gh api graphql -f query='
  mutation {
    createProjectV2(input: {
      ownerId: "'"$OWNER_ID"'"
      title: "PQC Scanner - Year 1 (2026)"
    }) {
      projectV2 {
        id
        number
        url
      }
    }
  }
')

PROJECT_ID=$(echo "$PROJECT_DATA" | jq -r '.data.createProjectV2.projectV2.id')
PROJECT_URL=$(echo "$PROJECT_DATA" | jq -r '.data.createProjectV2.projectV2.url')

echo "✅ Project created: $PROJECT_URL"
```

### Add Issues to Project

```bash
# Get repository ID
REPO_DATA=$(gh api graphql -f query='
  query {
    repository(owner: "arcqubit", name: "pqc-scanner") {
      id
    }
  }
')
REPO_ID=$(echo "$REPO_DATA" | jq -r '.data.repository.id')

# Add each issue to the project (issues #1-21)
for ISSUE_NUM in {1..21}; do
  echo "Adding issue #$ISSUE_NUM..."

  # Get issue ID
  ISSUE_DATA=$(gh api graphql -f query='
    query {
      repository(owner: "arcqubit", name: "pqc-scanner") {
        issue(number: '"$ISSUE_NUM"') {
          id
        }
      }
    }
  ')
  ISSUE_ID=$(echo "$ISSUE_DATA" | jq -r '.data.repository.issue.id')

  # Add to project
  gh api graphql -f query='
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
  '

  echo "✅ Added issue #$ISSUE_NUM to project"
done

echo ""
echo "🎉 All 21 issues added to project!"
echo "View project at: $PROJECT_URL"
```

---

## Recommended Project Board Structure

### Board View (Kanban)

```
┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
│   Backlog   │    Todo     │ In Progress │  In Review  │    Done     │
├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤
│ Epic #1     │             │             │             │             │
│ Epic #6     │             │             │             │             │
│ Epic #12    │             │             │             │             │
│ Epic #17    │             │             │             │             │
│             │             │             │             │             │
│ Feature #2  │             │             │             │             │
│ Feature #3  │             │             │             │             │
│ Feature #4  │             │             │             │             │
│ (18 more)   │             │             │             │             │
└─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
```

### Table View

| Issue | Title | Priority | Quarter | Story Points | Sprint | Status |
|-------|-------|----------|---------|--------------|--------|--------|
| #1 | Epic: CLI Tool Development | Critical | Q1 2026 | 21 | - | Backlog |
| #2 | Feature: Global CLI Installation | Critical | Q1 2026 | 5 | Sprint 1 | Backlog |
| #3 | Feature: Interactive Scanning | Critical | Q1 2026 | 8 | Sprint 2 | Backlog |
| ... | ... | ... | ... | ... | ... | ... |

---

## Grouping and Filtering

### Group By Quarter
- **Q1 2026**: Issues #1-5 (CLI Tool)
- **Q2 2026**: Issues #6-11 (IDE Extensions)
- **Q3 2026**: Issues #12-16 (Enhanced Detection)
- **Q4 2026**: Issues #17-21 (Compliance)

### Group By Epic
- **CLI Tool**: #1, #2, #3, #4, #5
- **IDE Extensions**: #6, #7, #8, #9, #10, #11
- **Detection**: #12, #13, #14, #15, #16
- **Compliance**: #17, #18, #19, #20, #21

### Filter by Priority
- **Critical**: #1, #2, #3, #6, #7, #8, #9, #15, #17, #18, #20
- **High**: #4, #10, #11, #12, #13, #14, #19, #21
- **Medium**: #5, #16

### Filter by Story Points
- **21 points (Epic)**: #1, #6, #12, #17
- **13 points**: #8, #11, #15, #16, #18, #19, #20, #21
- **8 points**: #3, #4, #9, #10, #13, #14
- **5 points**: #2, #5, #7

---

## Automation Rules (Optional)

Set up GitHub Actions workflows to automate project board updates:

### Auto-move to "In Progress"
```yaml
# When issue is assigned → Move to "In Progress"
```

### Auto-move to "In Review"
```yaml
# When PR is opened → Move to "In Review"
```

### Auto-move to "Done"
```yaml
# When issue is closed → Move to "Done"
```

---

## Sprint Planning Views

Create custom views for each sprint:

### Sprint 1 View (Q1 2026)
- Filter: `Quarter = "Q1 2026"` AND `Sprint = "Sprint 1"`
- Issues: #2, #3
- Story Points: 13

### Sprint 2 View (Q1 2026)
- Filter: `Quarter = "Q1 2026"` AND `Sprint = "Sprint 2"`
- Issues: #4, #5
- Story Points: 13

### Current Sprint View
- Filter: `Status = "In Progress"` OR `Status = "In Review"`
- Sorted by: Priority (Critical → High → Medium → Low)

---

## Team Views

### Developer 1 View
- Filter: `Assignee = "developer1"`
- Group by: Status

### QA Engineer View
- Filter: `Status = "In Review"`
- Group by: Priority

### Team Overview
- Show all issues
- Group by: Status
- Sort by: Priority, Story Points

---

## Integration with CI/CD

Link project board to GitHub Actions:

```yaml
# .github/workflows/update-project.yml
name: Update Project Board

on:
  issues:
    types: [opened, closed, assigned]
  pull_request:
    types: [opened, closed, merged]

jobs:
  update-board:
    runs-on: ubuntu-latest
    steps:
      - name: Update project status
        uses: actions/github-script@v7
        with:
          script: |
            // Auto-update project board based on issue/PR state
```

---

## Metrics and Reporting

### Velocity Tracking
- Calculate completed story points per sprint
- Track team velocity over time
- Adjust sprint planning based on actual velocity

### Burndown Chart
- Track remaining story points
- Monitor progress toward quarterly goals
- Identify blockers early

### Cycle Time
- Measure time from "Todo" → "Done"
- Identify bottlenecks in workflow
- Optimize team processes

---

## Quick Links

- **GitHub Issues**: https://github.com/arcqubit/pqc-scanner/issues
- **GitHub Milestones**: https://github.com/arcqubit/pqc-scanner/milestones
- **GitHub Projects**: https://github.com/orgs/arcqubit/projects (or /users/arcqubit/projects)
- **Implementation Plan**: [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)
- **Issues Summary**: [GITHUB_ISSUES_SUMMARY.md](./GITHUB_ISSUES_SUMMARY.md)

---

## Troubleshooting

### "INSUFFICIENT_SCOPES" Error
**Problem**: GitHub CLI doesn't have `project` scopes

**Solution**:
```bash
gh auth refresh -h github.com -s project,read:project,write:project
```

### Can't Add Issues to Project
**Problem**: Issues not appearing in project board

**Solution**:
1. Ensure issues are created (check https://github.com/arcqubit/pqc-scanner/issues)
2. Manually add via project board UI: Click "Add items" → Search by issue number
3. Use bulk add: `arcqubit/pqc-scanner#1-21`

### Project Board Not Visible
**Problem**: Can't find the project board

**Solution**:
1. Check organization projects: https://github.com/orgs/arcqubit/projects
2. Check user projects: https://github.com/users/arcqubit/projects
3. Verify you have proper permissions (Admin/Write access to repo)

---

## Manual Setup Checklist

- [ ] Create GitHub Project board via web UI
- [ ] Set project title: "PQC Scanner - Year 1 (2026)"
- [ ] Add custom fields (Status, Priority, Quarter, Story Points, Sprint, Epic)
- [ ] Add all 21 issues to the project (#1-#21)
- [ ] Organize issues by Quarter (Q1, Q2, Q3, Q4)
- [ ] Set up board views (Kanban, Table, Sprint views)
- [ ] Configure automation rules (optional)
- [ ] Share project link with team
- [ ] Begin sprint planning for Q1 2026

---

**Document Version**: 1.0
**Created**: 2025-11-07
**Author**: ArcQubit Team
**Last Updated**: 2025-11-07
