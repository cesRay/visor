#!/bin/sh
# Points the formula at a release tag and fills in its SHA-256.
# Usage: packaging/homebrew/bump.sh 0.2.0   (the tag v0.2.0 must be pushed)
set -eu
version="$1"
formula="$(dirname "$0")/visor.rb"
url="https://github.com/cesRay/visor/archive/refs/tags/v${version}.tar.gz"
sha="$(curl -fsSL "$url" | shasum -a 256 | cut -d' ' -f1)"
sed -i.bak -e "s|^  url \".*\"|  url \"${url}\"|" -e "s|^  sha256 \".*\"|  sha256 \"${sha}\"|" "$formula"
rm -f "$formula.bak"
echo "visor.rb -> v${version} (${sha})"
