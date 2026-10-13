#!/usr/bin/env bash
# One-time: create the Android signing key for Dew and print the GitHub secrets to add
# (Settings -> Secrets and variables -> Actions). Keep dew-release.jks and the password safe and
# OFF git: every future update must be signed with the same key or phones will refuse it.
set -euo pipefail
read -rsp "Choose a keystore password: " PASS; echo
keytool -genkeypair -keystore dew-release.jks -storepass "$PASS" -keypass "$PASS" -alias dew \
  -keyalg RSA -keysize 4096 -validity 10000 -dname "CN=Dew, O=AGC Oshodi, L=Lagos, C=NG"
echo
echo "ANDROID_KEYSTORE_BASE64 = $(base64 -w0 dew-release.jks)"
echo "ANDROID_KEYSTORE_PASSWORD = (the password you chose)"
echo "ANDROID_KEY_ALIAS = dew"
echo "ANDROID_KEY_PASSWORD = (the same password)"
