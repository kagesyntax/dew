# Dew DevOps: the 7 C's

| C | What it means here | Where |
| --- | --- | --- |
| **Continuous Development** | `feature/*` → `develop` → release PR → `main`, conventional commits, CHANGELOG `[Unreleased]` written as you go | CONTRIBUTING.md, `prepare-release.yml` |
| **Continuous Integration** | Every push and PR: rustfmt, clippy (`-D warnings`), cargo-deny, gitleaks secret scan, web build | `ci.yml` |
| **Continuous Testing** | Core unit tests (scripture parser, manual model, versions, changelog), bundled-data validation; release refuses to ship if tests fail | `ci.yml`, `release.yml` → prepare |
| **Continuous Delivery** | Installable builds on every push to `develop`: web bundle + **Preview Android APK** artifacts for testers | `ci.yml` → `android.yml` |
| **Continuous Deployment** | Merging the release PR ships: GitHub Pages web app, signed **Android APK**, macOS/Windows/Linux installers, `version.json`, GitHub Release | `release.yml` |
| **Continuous Monitoring** | Every 6 hours: site, `version.json` and every installer link are checked; failures open a `monitoring` issue | `monitor.yml` |
| **Continuous Feedback** | In-app Settings → Send Feedback opens a GitHub issue form (feedback / bug, with version); Software Update + What's New close the loop back to users | `.github/ISSUE_TEMPLATE/`, Settings |

## Android
- Built by `android.yml` (reusable): Rust → `dx bundle --platform android` → zipalign → apksigner.
- One-time: run `tools/android-keystore.sh`, add the four `ANDROID_*` secrets. Keep the `.jks` safe;
  every update must be signed with the same key.
- Users install the APK from the GitHub Release page; Settings → Software Update links the new APK.

## Shipping a version
1. Actions → **Prepare Release** → patch/minor/major. 2. Merge the `release/vX.Y.Z` PR into `main`.
3. Merge `main` back into `develop`. Every installed Dew sees the update.
