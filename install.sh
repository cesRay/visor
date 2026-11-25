#!/bin/sh
# Installs the latest visor release on Raspberry Pi OS (64-bit or 32-bit),
# or on any other Debian-based arm64 / armhf system.
#
#   curl -fsSL https://raw.githubusercontent.com/cesRay/visor/main/install.sh | sh
#
# It downloads the .deb that matches this system and installs it with apt,
# which also installs vim.
set -eu

# Everything runs inside main, called on the last line, so a partly
# downloaded script does nothing
main() {
  repo="cesRay/visor"

  fail() {
    echo "visor install: $*" >&2
    exit 1
  }

  command -v dpkg >/dev/null 2>&1 ||
    fail "this script needs Raspberry Pi OS or another Debian-based system. Elsewhere, use Homebrew: brew install cesRay/tap/visor"

  arch="$(dpkg --print-architecture)"
  case "$arch" in
    arm64 | armhf) ;;
    *) fail "there is no visor package for '$arch'. On this system, use Homebrew: brew install cesRay/tap/visor" ;;
  esac

  sudo=""
  if [ "$(id -u)" -ne 0 ]; then
    command -v sudo >/dev/null 2>&1 || fail "run this script as root, or install sudo"
    sudo="sudo"
  fi

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT
  # apt reads the package as the unprivileged _apt user
  chmod 755 "$tmp"

  url="https://github.com/$repo/releases/latest/download/visor_$arch.deb"
  echo "Downloading $url"
  curl -fsSL -o "$tmp/visor_$arch.deb" "$url" ||
    fail "download failed: $url (the latest release may not include a package for $arch yet)"
  chmod 644 "$tmp/visor_$arch.deb"

  # Refresh the package lists so apt can find vim. Under `curl | sh`, stdin
  # is this script, so apt gets </dev/null instead
  $sudo apt-get update -qq </dev/null
  $sudo apt-get install -y "$tmp/visor_$arch.deb" </dev/null

  cat <<'MSG'

visor is installed. So that visor can change your directory when it exits,
add this line to ~/.bashrc and open a new terminal:

  eval "$(visor init bash)"

Then run: visor
MSG
}

main "$@"
