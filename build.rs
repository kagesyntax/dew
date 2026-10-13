//! Stamps the build with its commit and date, shown in Settings → Software Update.
use std::process::Command;

fn main() {
    let sha = std::env::var("GITHUB_SHA").ok().map(|s| s[..7.min(s.len())].to_string()).or_else(|| {
        Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    });
    println!("cargo:rustc-env=BUILD_SHA={}", sha.unwrap_or_else(|| "dev".into()));
    let date = Command::new("date").args(["-u", "+%Y-%m-%d"]).output().ok().filter(|o| o.status.success());
    let date = date.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    println!("cargo:rustc-env=BUILD_DATE={date}");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
}
