# GitHub App Setup for Release Please

Since repository-level permissions are disabled (Option A grayed out), we'll use a GitHub App instead. This is actually **more secure** and gives you fine-grained control.

## Quick Setup (5 minutes)

### Option 1: Automated Script

Run the setup script:

```bash
cd /mnt/c/Users/bowma/Projects/_aq_/pqc-scanner
.github/scripts/setup-release-please.sh
```

The script will guide you through each step interactively.

---

### Option 2: Manual Setup

Follow these steps manually:

## Step 1: Create GitHub App

1. **Open GitHub App Creation Page:**

   Click this link: [Create New GitHub App](https://github.com/settings/apps/new)

2. **Fill in Basic Information:**

   | Field | Value |
   |-------|-------|
   | **GitHub App name** | `pqc-scanner-release-please` |
   | **Homepage URL** | `https://github.com/arcqubit/pqc-scanner` |
   | **Callback URL** | Leave empty |
   | **Setup URL** | Leave empty |
   | **Webhook** | ❌ Uncheck "Active" |
   | **Description** | `Automated release management for PQC Scanner` (optional) |

3. **Set Permissions:**

   Scroll down to **Repository permissions** and set:

   | Permission | Access |
   |------------|--------|
   | **Contents** | Read and write ✅ |
   | **Pull requests** | Read and write ✅ |
   | **Metadata** | Read-only (automatic) |

   Leave all other permissions as "No access"

4. **Where can this GitHub App be installed?**

   Select: ✅ **Only on this account**

5. **Click "Create GitHub App"**

---

## Step 2: Note Your App ID

After creation, you'll be on the app settings page.

1. Look at the top of the page
2. You'll see: **App ID: 123456** (your actual ID will be different)
3. **Write this down** - you'll need it in Step 5

---

## Step 3: Generate Private Key

Still on the app settings page:

1. Scroll down to **Private keys** section
2. Click **"Generate a private key"**
3. A `.pem` file will download automatically
4. **Save this file securely** - you'll need it in Step 5
5. **Do NOT commit this file to git!**

---

## Step 4: Install App on Repository

1. In the left sidebar, click **"Install App"**
2. Click **"Install"** next to your account (arcqubit)
3. On the installation page:
   - Select: ✅ **Only select repositories**
   - Choose: ✅ **pqc-scanner**
4. Click **"Install"**

---

## Step 5: Add Secrets to Repository

Now we'll add the App ID and Private Key as repository secrets.

### Using GitHub Web Interface:

1. Go to: [Repository Secrets Settings](https://github.com/arcqubit/pqc-scanner/settings/secrets/actions)

2. Click **"New repository secret"**

3. **First Secret:**
   - Name: `RELEASE_PLEASE_APP_ID`
   - Value: Your App ID from Step 2 (e.g., `123456`)
   - Click **"Add secret"**

4. Click **"New repository secret"** again

5. **Second Secret:**
   - Name: `RELEASE_PLEASE_APP_PRIVATE_KEY`
   - Value: Open the `.pem` file in a text editor, copy **ALL** content including the BEGIN and END lines:
     ```
     -----BEGIN RSA PRIVATE KEY-----
     MIIEpAIBAAKCAQEA...
     ... (many lines) ...
     -----END RSA PRIVATE KEY-----
     ```
   - Paste the entire content
   - Click **"Add secret"**

### Using GitHub CLI:

If you prefer command line:

```bash
# Set App ID
gh secret set RELEASE_PLEASE_APP_ID --body "123456"

# Set Private Key (replace path with your actual .pem file location)
gh secret set RELEASE_PLEASE_APP_PRIVATE_KEY < ~/Downloads/pqc-scanner-release-please.2025-11-18.private-key.pem
```

---

## Step 6: Verify Setup

Check that secrets were added:

```bash
gh secret list
```

You should see:
```
RELEASE_PLEASE_APP_ID           Updated 2025-11-18
RELEASE_PLEASE_APP_PRIVATE_KEY  Updated 2025-11-18
```

---

## Step 7: Test Release Please

Make a test commit to trigger Release Please:

```bash
# Create empty commit with conventional format
git commit -m "feat: test release please automation" --allow-empty
git push origin main
```

Watch the workflow:

```bash
gh run watch
```

Expected result:
- ✅ Workflow completes successfully
- ✅ New PR created with changelog
- ✅ No permission errors!

Check for the PR:

```bash
gh pr list
```

---

## Cleanup

After successful setup:

1. **Delete the .pem file** (it's securely stored in GitHub Secrets):
   ```bash
   rm ~/Downloads/pqc-scanner-release-please.*.private-key.pem
   ```

2. **Test again** to ensure it works with the secrets

---

## Troubleshooting

### "Resource not accessible by integration"

**Cause:** App not installed or missing permissions

**Solution:**
1. Go to: https://github.com/settings/installations
2. Click on your app
3. Verify **pqc-scanner** is in the repository list
4. Check permissions: Contents (write), Pull requests (write)

### "Bad credentials"

**Cause:** Private key not set correctly

**Solution:**
1. Verify the entire `.pem` file content was copied (including BEGIN/END lines)
2. No extra spaces or line breaks added
3. Re-add the secret if needed

### App ID vs Installation ID

**Important:** Use the **App ID** (found on app settings page), NOT the installation ID.

### Secret Name Typos

Ensure exact names:
- `RELEASE_PLEASE_APP_ID` (not APP-ID, app_id, etc.)
- `RELEASE_PLEASE_APP_PRIVATE_KEY` (not PRIVATE-KEY, private_key, etc.)

### Still Getting Permission Errors?

Try regenerating the private key:
1. Go to app settings
2. Delete old private key
3. Generate new one
4. Update the secret

---

## Security Best Practices

1. ✅ Never commit `.pem` files to git
2. ✅ Never share private keys
3. ✅ Rotate keys every 90 days
4. ✅ Use minimum required permissions
5. ✅ Monitor app installations: https://github.com/settings/installations
6. ✅ Review secret access logs periodically

---

## What Happens Next?

Once configured, Release Please will:

1. **On every push to `main`:**
   - Analyze commits since last release
   - Determine next version number
   - Generate/update CHANGELOG.md
   - Create PR with version bumps

2. **When you merge the release PR:**
   - Git tag created automatically
   - Release workflow triggered
   - Artifacts built and signed
   - GitHub release published

---

## Need Help?

- 📖 [Release Please Docs](https://github.com/googleapis/release-please)
- 📖 [GitHub Apps Docs](https://docs.github.com/en/apps)
- 📖 Full guide: `.github/RELEASE_PLEASE_SETUP.md`

---

## Quick Reference

**App Settings:** https://github.com/settings/apps/pqc-scanner-release-please

**Repository Secrets:** https://github.com/arcqubit/pqc-scanner/settings/secrets/actions

**Workflow Runs:** https://github.com/arcqubit/pqc-scanner/actions/workflows/release-please.yml
