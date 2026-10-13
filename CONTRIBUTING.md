# Contributing

## Branches
| Branch | Purpose |
| --- | --- |
| `main` | Production. Protected: PR only, CI must pass. Tags `vX.Y.Z` trigger a release. |
| `develop` | Integration. Feature branches merge here first. |
| `feature/<name>` | New work, e.g. `feature/reading-plans` |
| `fix/<name>` | Bug fixes |
| `release/<version>` | Optional stabilisation before tagging |

Never commit directly to `main`.

## Workflow
1. `git checkout develop && git pull && git checkout -b feature/my-change`
2. Commit with Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `ci:`)
3. Push and open a PR into `develop`. CI runs fmt, clippy, tests, cargo-deny, gitleaks and a release build
4. To release: PR `develop` → `main`, bump `version` in `Cargo.toml`, then
   `git tag -a v0.2.0 -m "Reading plans and bookmarks screen" && git push --tags`.
   The tag message becomes the "What's new" text in Settings → Check for Updates.

## Repository settings to enable on GitHub
- Branch protection on `main` and `develop`: require the **CI** checks, require PR review
- Pages: Source = GitHub Actions
- Secrets: `TURSO_DATABASE_URL`, `TURSO_READONLY_TOKEN`. Variables: `CHURCH_NAME`
