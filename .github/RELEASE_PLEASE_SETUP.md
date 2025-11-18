# Release Please Setup Guide

Release Please requires special permissions to create pull requests. This guide covers three setup options.

## Current Status

The workflow is configured to support multiple authentication methods with automatic fallback.

## Setup Options

### **Option 1: Enable Repository Permissions (Simplest)**

Enable GitHub Actions to create PRs at the repository level:

1. Go to: https://github.com/arcqubit/pqc-scanner/settings/actions
2. Scroll to **Workflow permissions**
3. Select: ✅ **Read and write permissions**
4. Check: ✅ **Allow GitHub Actions to create and approve pull requests**
5. Click **Save**

**Pros:**
- ✅ Zero configuration required
- ✅ Works immediately
- ✅ No secrets to manage

**Cons:**
- ⚠️ All workflows in the repository get elevated permissions
- ⚠️ Less fine-grained control

---

### **Option 2: GitHub App (Recommended for Production)**

Create a GitHub App with scoped permissions:

#### Step 1: Create GitHub App

1. Go to: https://github.com/settings/apps/new
2. Configure:
   - **GitHub App name**: `pqc-scanner-release-please`
   - **Homepage URL**: `https://github.com/arcqubit/pqc-scanner`
   - **Webhook**: Uncheck "Active"

3. **Permissions** (Repository permissions):
   - Contents: **Read and write**
   - Pull requests: **Read and write**
   - Metadata: **Read-only** (automatic)

4. **Where can this GitHub App be installed?**
   - Select: **Only on this account**

5. Click **Create GitHub App**

#### Step 2: Generate Private Key

1. On the app settings page, scroll to **Private keys**
2. Click **Generate a private key**
3. Save the downloaded `.pem` file securely

#### Step 3: Install App on Repository

1. On the app settings page, click **Install App**
2. Select your account: **arcqubit**
3. Choose: **Only select repositories**
4. Select: **pqc-scanner**
5. Click **Install**

#### Step 4: Configure Repository Secrets

1. Go to: https://github.com/arcqubit/pqc-scanner/settings/secrets/actions
2. Click **New repository secret**

**Secret 1: RELEASE_PLEASE_APP_ID**
- Name: `RELEASE_PLEASE_APP_ID`
- Value: Your App ID (found on app settings page)

**Secret 2: RELEASE_PLEASE_APP_PRIVATE_KEY**
- Name: `RELEASE_PLEASE_APP_PRIVATE_KEY`
- Value: Full contents of the `.pem` file (including BEGIN/END lines)

#### Step 5: Test

Make a commit with conventional format and push to main:
```bash
git commit -m "feat: test release please automation"
git push origin main
```

**Pros:**
- ✅ Scoped permissions (most secure)
- ✅ Works across forks
- ✅ Can be revoked independently
- ✅ Audit trail via GitHub App

**Cons:**
- ⚠️ Requires initial setup
- ⚠️ Need to manage private key rotation

---

### **Option 3: Personal Access Token (Fallback)**

Create a PAT with limited scopes:

#### Step 1: Create Fine-Grained PAT

1. Go to: https://github.com/settings/personal-access-tokens/new
2. Configure:
   - **Token name**: `pqc-scanner-release-please`
   - **Expiration**: 90 days (or custom)
   - **Repository access**: Only select repositories → pqc-scanner

3. **Permissions**:
   - Contents: **Read and write**
   - Pull requests: **Read and write**

4. Click **Generate token**
5. Copy the token immediately (shown only once)

#### Step 2: Add Repository Secret

1. Go to: https://github.com/arcqubit/pqc-scanner/settings/secrets/actions
2. Click **New repository secret**
3. Name: `RELEASE_PLEASE_TOKEN`
4. Value: Your PAT
5. Click **Add secret**

#### Step 3: Update Workflow

Edit `.github/workflows/release-please.yml`:

```yaml
- name: Run Release Please
  id: release
  uses: googleapis/release-please-action@v4
  with:
    token: ${{ secrets.RELEASE_PLEASE_TOKEN }}
```

**Pros:**
- ✅ Simple to set up
- ✅ Works immediately

**Cons:**
- ⚠️ Token expires (need to renew)
- ⚠️ Tied to personal account
- ⚠️ Broad permissions if classic PAT

---

## Current Workflow Configuration

The workflow is configured with **smart fallback**:

```yaml
- name: Generate GitHub App Token
  id: app-token
  uses: actions/create-github-app-token@v1
  with:
    app-id: ${{ secrets.RELEASE_PLEASE_APP_ID }}
    private-key: ${{ secrets.RELEASE_PLEASE_APP_PRIVATE_KEY }}
  continue-on-error: true

- name: Run Release Please
  uses: googleapis/release-please-action@v4
  with:
    # Try GitHub App first, fall back to GITHUB_TOKEN
    token: ${{ steps.app-token.outputs.token || secrets.GITHUB_TOKEN }}
```

**Fallback Order:**
1. GitHub App token (if configured)
2. Default GITHUB_TOKEN (if repository permissions enabled)

---

## Testing the Setup

### Test Release Please

After configuring any option above:

```bash
# Make a test commit with conventional format
git commit -m "feat: test automated release" --allow-empty
git push origin main
```

Watch the workflow run:
```bash
gh run watch
```

### Expected Behavior

1. ✅ Workflow runs successfully
2. ✅ New branch created: `release-please--branches--main--components--pqc-scanner`
3. ✅ Pull request created: `chore(main): release X.Y.Z`
4. ✅ PR contains:
   - Updated CHANGELOG.md
   - Version bumps in Cargo.toml and package.json
   - Updated .release-please-manifest.json

---

## Troubleshooting

### Error: "GitHub Actions is not permitted to create or approve pull requests"

**Solution**: Choose one of the setup options above.

### Error: "Resource not accessible by integration"

**Cause**: GitHub App not installed or missing permissions.

**Solution**:
1. Verify app is installed: https://github.com/settings/installations
2. Check app has Contents and Pull Requests write permissions

### Error: "Bad credentials"

**Cause**: Invalid or expired PAT.

**Solution**:
1. Regenerate PAT
2. Update `RELEASE_PLEASE_TOKEN` secret

### PR Not Created But Branch Exists

**Cause**: Permission error occurred after branch creation.

**Solution**:
```bash
# Manually create PR from existing branch
gh pr create \
  --base main \
  --head release-please--branches--main--components--pqc-scanner \
  --title "chore(main): release X.Y.Z" \
  --body "Automated release PR"
```

### App Token Not Generated

**Check secret names match exactly:**
- `RELEASE_PLEASE_APP_ID` (not APP-ID or app_id)
- `RELEASE_PLEASE_APP_PRIVATE_KEY` (not PRIVATE-KEY)

**Verify private key format:**
```
-----BEGIN RSA PRIVATE KEY-----
...content...
-----END RSA PRIVATE KEY-----
```

---

## Recommended Setup

**For this project**: Use **Option 2 (GitHub App)** because:

- ✅ Security-conscious project (PQC scanner)
- ✅ Public repository
- ✅ Long-term maintenance expected
- ✅ Fine-grained permissions align with security goals

**Setup time**: ~5 minutes

---

## Security Notes

1. **Never commit private keys** to the repository
2. **Rotate keys regularly** (every 90 days recommended)
3. **Use least-privilege permissions** (only what's needed)
4. **Monitor app installations**: https://github.com/settings/installations
5. **Review secret access logs**: Repository → Settings → Secrets → Actions

---

## Additional Resources

- [Release Please Documentation](https://github.com/googleapis/release-please)
- [GitHub Apps Documentation](https://docs.github.com/en/apps)
- [Fine-grained PAT Documentation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
- [Conventional Commits](https://www.conventionalcommits.org/)
