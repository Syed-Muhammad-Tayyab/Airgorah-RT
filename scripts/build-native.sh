#!/usr/bin/env bash
#
# build-native.sh — build Airgorah-RT and package it for the local
# distribution without going through the Docker-based `fpm` workflow.
#
# Added in the Airgorah-RT fork. The original project's Docker/fpm
# pipeline (see ../Dockerfile and ../.fpm) is still the recommended way to
# produce the .deb/.rpm/.pkg.tar.zst artifacts for CI and releases; this
# script is a lighter-weight option for building and testing locally on a
# machine that already has the dependencies installed.
#
# Usage:
#   ./scripts/build-native.sh [deb|rpm|pacman]
#
# With no argument, it only builds the binaries (no package).

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

TARGET="${1:-}"

echo "==> Building Airgorah-RT (release profile)"
cargo build --release

if [ -z "$TARGET" ]; then
    echo "==> Binaries built at target/release/airgorah(-agent)."
    echo "    Pass 'deb', 'rpm' or 'pacman' to also build a package with fpm."
    exit 0
fi

if ! command -v fpm >/dev/null 2>&1; then
    echo "error: fpm is not installed (gem install fpm), or run via Docker instead" >&2
    exit 1
fi

# fpm automatically reads default options (name, version, sources, etc.)
# from the ./.fpm file in the current directory — same as the Dockerfile's
# build step, so we don't need to repeat them here.
case "$TARGET" in
    deb)
        fpm -f -t deb -p "airgorah-rt_$(uname -m).deb" -a native \
            --depends libgtk-4-1 --depends dbus-x11 --depends iproute2 \
            --depends crunch --deb-recommends aircrack-ng
        ;;
    rpm)
        fpm -f -t rpm -p "airgorah-rt_$(uname -m).rpm" -a native \
            --depends gtk4-devel --depends dbus-x11 --depends iproute \
            --rpm-tag "Recommends:aircrack-ng"
        ;;
    pacman)
        fpm -f -t pacman -p "airgorah-rt_$(uname -m).pkg.tar.zst" -a native \
            --depends gtk4 --depends dbus --depends iproute2 \
            --pacman-optional-depends aircrack-ng
        ;;
    *)
        echo "error: unknown target '$TARGET' (expected deb, rpm or pacman)" >&2
        exit 1
        ;;
esac

echo "==> Done."
