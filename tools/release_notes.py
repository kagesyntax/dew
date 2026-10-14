#!/usr/bin/env python3
"""Release helpers used by GitHub Actions.

  release_notes.py notes 0.2.0           -> markdown body of that CHANGELOG section
  release_notes.py manifest 0.2.0 ...    -> version.json (see church_core::version::UpdateManifest)
  release_notes.py bump patch|minor|major -> bumps Cargo.toml + moves [Unreleased] into the new version
"""
import datetime, json, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CHANGELOG = os.path.join(ROOT, "CHANGELOG.md")
CARGO = os.path.join(ROOT, "Cargo.toml")


def section(version):
    lines, out, inside = open(CHANGELOG, encoding="utf-8").read().splitlines(), [], False
    for line in lines:
        if line.startswith("## "):
            if inside:
                break
            inside = re.match(r"## \[?" + re.escape(version) + r"\]?(\s|$)", line) is not None
            continue
        if inside:
            out.append(line)
    return "\n".join(out).strip()


def bullets(md):
    return [re.sub(r"\*\*", "", l.strip()[2:]) for l in md.splitlines() if l.strip()[:2] in ("- ", "* ")]


def manifest(version, repo, minimum=None):
    tag = f"v{version}"
    base = f"https://github.com/{repo}/releases/download/{tag}"
    notes = bullets(section(version))
    data = {
        "version": version,
        "notes": notes[0] if notes else "Bug fixes and improvements.",
        "highlights": notes,
        "url": f"https://github.com/{repo}/releases/tag/{tag}",
        "published_at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "downloads": {
            "macos": f"{base}/Dew-macos.dmg",
            "windows": f"{base}/Dew-windows.exe",
            "linux": f"{base}/Dew-linux.AppImage",
            "android": f"{base}/Dew-android.apk",
        },
    }
    if minimum:
        data["minimum_version"] = minimum
    return json.dumps(data, indent=2)


def bump(level):
    cargo = open(CARGO, encoding="utf-8").read()
    cur = re.search(r'^version = "(\d+)\.(\d+)\.(\d+)"', cargo, re.M)
    major, minor, patch = map(int, cur.groups())
    major, minor, patch = {"major": (major + 1, 0, 0), "minor": (major, minor + 1, 0), "patch": (major, minor, patch + 1)}[level]
    new = f"{major}.{minor}.{patch}"
    open(CARGO, "w", encoding="utf-8").write(cargo.replace(cur.group(0), f'version = "{new}"', 1))
    today = datetime.date.today().isoformat()
    log = open(CHANGELOG, encoding="utf-8").read()
    log = log.replace("## [Unreleased]", f"## [Unreleased]\n\n## [{new}] - {today}", 1)
    open(CHANGELOG, "w", encoding="utf-8").write(log)
    return new


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "notes":
        print(section(sys.argv[2]) or "Bug fixes and improvements.")
    elif cmd == "manifest":
        print(manifest(sys.argv[2], sys.argv[3], sys.argv[4] if len(sys.argv) > 4 else None))
    elif cmd == "bump":
        print(bump(sys.argv[2]))
