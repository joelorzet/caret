#!/usr/bin/env bash
# Builds caret.app, and a .dmg to install it from.
#
# Unsigned: macOS will ask once on first launch unless you sign it yourself.
# `--install` copies it to ~/Applications, which is where Spotlight looks.
set -euo pipefail

cd "$(dirname "$0")/.."
root=$PWD
out=$root/target/macos
app=$out/caret.app
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)

install=false
dmg=false
universal=false
for arg in "$@"; do
  case $arg in
    --install) install=true ;;
    --dmg) dmg=true ;;
    --universal) universal=true ;;
    *) echo "usage: $0 [--install] [--dmg] [--universal]" >&2; exit 2 ;;
  esac
done

echo "building caret $version"
if $universal; then
  # Both architectures from one machine, which is one macOS runner instead
  # of two. They are scarce enough that asking for a second is what keeps
  # a release waiting.
  for target in aarch64-apple-darwin x86_64-apple-darwin; do
    rustup target add "$target" >/dev/null 2>&1 || true
    cargo build --release --quiet --target "$target"
  done
  mkdir -p "$out"
  lipo -create -output "$out/caret" \
    target/aarch64-apple-darwin/release/caret \
    target/x86_64-apple-darwin/release/caret
  binary=$out/caret
else
  cargo build --release --quiet
  binary=target/release/caret
fi

# The icon is drawn rather than stored, so it is rebuilt with the bundle.
python3 -I scripts/icon.py macos/caret.icns >/dev/null

rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

sed "s/__VERSION__/$version/g" macos/Info.plist > "$app/Contents/Info.plist"
cp "$binary" "$app/Contents/MacOS/caret"
cp macos/caret.icns "$app/Contents/Resources/caret.icns"
printf 'APPL????' > "$app/Contents/PkgInfo"

# An ad-hoc signature keeps Gatekeeper from refusing it outright and stops
# the "damaged" dialog that an unsigned arm64 binary gets.
codesign --force --deep --sign - "$app" 2>/dev/null || \
  echo "note: could not sign; the first launch will need right-click then Open"

echo "built $app"

if $install; then
  mkdir -p "$HOME/Applications"
  # Replacing the bundle under a running copy is allowed, and the running
  # copy carries on from the old one until it is quit. Say so rather than
  # leaving someone wondering why nothing changed.
  running=$(pgrep -f "Applications/caret.app" || true)
  rm -rf "$HOME/Applications/caret.app"
  cp -R "$app" "$HOME/Applications/"
  # Tell Spotlight about it now rather than whenever it next looks.
  mdimport "$HOME/Applications/caret.app" 2>/dev/null || true
  echo "installed to ~/Applications, findable in Spotlight as caret"
  if [ -n "$running" ]; then
    echo "note: a copy is still running and is still the old one; quit it to pick this up"
  fi
fi

if $dmg; then
  staging=$out/dmg
  rm -rf "$staging" "$out/caret-$version.dmg"
  mkdir -p "$staging"
  cp -R "$app" "$staging/"
  ln -s /Applications "$staging/Applications"
  hdiutil create -quiet -volname "caret" -srcfolder "$staging" \
    -ov -format ULFO "$out/caret-$version.dmg"
  rm -rf "$staging"
  echo "built $out/caret-$version.dmg"
  lipo -archs "$app/Contents/MacOS/caret"
fi
