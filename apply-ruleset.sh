# Set the repo once
REPO=arcqubit/pqc-scanner

# ── A) Strict protection for main ──────────────────────────────────────────────
gh api -X POST repos/$REPO/rulesets \
  -f name='Main – Strict' \
  -f target='main' \
  -f enforcement='active' \
  -F conditions='{"ref_name":{"include":["main"],"exclude":[]}}' \
  -F rules='[
    {"type":"pull_request","parameters":{
      "dismiss_stale_reviews_on_push":true,
      "require_code_owner_review":true,
      "required_approvals":2,
      "require_last_push_approval":true,
      "require_conversation_resolution":true}},
    {"type":"required_status_checks","parameters":{
      "strict_required_status_checks_policy":true,
      "required_status_checks":[
        "ci/build",
        "ci/test",
        "fmt/check",
        "lint/clippy",
        "cargo-audit",
        "cargo-deny",
        "supplychain/sbom-syft",
        "supplychain/trivy-scan",
        "supplychain/cosign-verify",
        "codeql"
      ]}},
    {"type":"signature","parameters":{"commits":"required","tags":"required"}},
    {"type":"linear_history"},
    {"type":"non_fast_forward"},
    {"type":"restrict_creation","parameters":{"block_creations":false,"block_deletions":true,"block_force_pushes":true}}
  ]'

# ── B) Very-strict protection for release/* and hotfix/* ───────────────────────
gh api -X POST repos/$REPO/rulesets \
  -f name='Release/Hotfix – Very Strict' \
  -f target='main' \
  -f enforcement='active' \
  -F conditions='{"ref_name":{"include":["release/*","hotfix/*"],"exclude":[]}}' \
  -F rules='[
    {"type":"pull_request","parameters":{
      "dismiss_stale_reviews_on_push":true,
      "require_code_owner_review":true,
      "required_approvals":3,
      "require_last_push_approval":true,
      "require_conversation_resolution":true}},
    {"type":"required_status_checks","parameters":{
      "strict_required_status_checks_policy":true,
      "required_status_checks":[
        "ci/build",
        "ci/test",
        "fmt/check",
        "lint/clippy",
        "cargo-audit",
        "cargo-deny",
        "supplychain/sbom-syft",
        "supplychain/trivy-scan",
        "supplychain/cosign-verify",
        "release/sign-and-publish",
        "codeql"
      ]}},
    {"type":"signature","parameters":{"commits":"required","tags":"required"}},
    {"type":"linear_history"},
    {"type":"non_fast_forward"},
    {"type":"restrict_creation","parameters":{
      "block_creations":false,"block_deletions":true,"block_force_pushes":true}},
    {"type":"required_deployments","parameters":{
      "environments":["staging"],"type":"updated_ref"}}
  ]'

# ── C) Immutable, signed tags v*.*.* ───────────────────────────────────────────
gh api -X POST repos/$REPO/rulesets \
  -f name='Release Tags – Immutable & Signed' \
  -f target='tag' \
  -f enforcement='active' \
  -F conditions='{"ref_name":{"include":["v*.*.*"],"exclude":[]}}' \
  -F rules='[
    {"type":"restrict_creation","parameters":{"block_creations":false,"block_deletions":true,"block_force_pushes":true}},
    {"type":"signature","parameters":{"commits":"off","tags":"required"}},
    {"type":"required_status_checks","parameters":{
      "strict_required_status_checks_policy":false,
      "required_status_checks":["release/tag-attest"]}}
  ]'

# (Optional) List rulesets to confirm and note their IDs
gh api repos/$REPO/rulesets --jq '.[] | {id, name, target, conditions}'

