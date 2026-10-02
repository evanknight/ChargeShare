# Publishing the reviewed scaffold

The source archive includes only public-safe project files. It excludes Git history, dependencies, scanner binaries, local databases and credentials. Repository creation and the prepared archive are separate from a successful publication.

## Preferred route

Use a Codex task environment authorized for this repository only, or an already-authorized local Git checkout with write access. Copy the reviewed scaffold into a fresh clone of the repository so its existing history is preserved. A read-only GitHub connection can inspect a repository but cannot publish it. Do not paste a token into chat or commit one; new persistent credentials or app access require a separate secure setup.

## From an authorized local checkout

1. Clone the intended repository with your normal authenticated Git workflow. Verify the owner, name, public visibility and remote before editing. This scaffold targets the ChargeShare repository whose existing initial README contains only its project title.
2. Extract `chargeshare-initial-scaffold.zip` outside the clone. Review its `chargeshare/` directory, then copy its contents, including dotfiles, into the clone. Do not replace `.git/` or overwrite any newly added project files without reviewing them.
3. Use Node.js 24 or newer. Review your Git author metadata; use your verified GitHub noreply address if you do not want a personal email in public commits. Do not change account-wide settings as part of these commands.
4. Install tooling and repository-local safeguards:

```sh
npm ci --ignore-scripts
bash scripts/security/install-gitleaks.sh
bash scripts/security/install-hooks.sh
node scripts/security/test-guards.mjs
npm run spec:validate
```

5. Stage only the intended source files:

```sh
git add .agents .env.example .githooks .github .gitignore .gitleaks.toml .nvmrc \
  AGENTS.md README.md SECURITY.md docs openspec package.json package-lock.json scripts
git diff --cached --check
npm run security:staged
git diff --cached
```

Review the complete diff for secrets and personal data, including filenames, examples and metadata. Automated detection cannot replace this review. Stop on any finding or unexpected file; do not bypass a hook or force-add ignored data.

6. Commit and scan the full local history before pushing:

```sh
git commit -m "Initialize ChargeShare plans, OpenSpec and public-repository safeguards"
npm run security:scan
git log --format=fuller -1
git remote -v
git push origin HEAD:main
```

This is a normal fast-forward push; never add `--force`. If the remote changed, stop and integrate the new work deliberately before repeating the checks. Authentication failures require an authorized publishing environment, not a workaround around a denied connection.

7. Verify the exact resulting commit on GitHub, confirm the expected files and public visibility, and wait for both repository-check jobs to pass on that commit. The CI workflow is prepared in this archive but is not evidence of a passed hosted run until published and executed.

## Scope remains planning-only

Publishing the scaffold does not authorize Tesla registration, OAuth, credential creation, vehicle access, deployment, spending or changes to GitHub security settings. Review available repository secret protection and private-reporting settings separately. Real implementation starts only after the initial proposal is approved.
