#!/usr/bin/env sh

set -eu

usage() {
  cat <<'EOF'
Usage: ./run.sh [--dev|--release] [-- <rustnake args>]

Options:
  --release   Run with release profile (default)
  --dev       Run with dev profile
  -h, --help  Show this help message
EOF
}

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is not installed or not in PATH." >&2
  exit 1
fi

profile="--release"

case "${1:-}" in
  --dev)
    profile=""
    shift
    ;;
  --release)
    profile="--release"
    shift
    ;;
  -h|--help)
    usage
    exit 0
    ;;
esac

echo "Starting Rustnake..."
echo "Controls: WASD/Arrows move | P pause | M mute | SPACE menu | Q quit"

if [ -n "$profile" ]; then
  cargo run "$profile" -- "$@"
else
  cargo run -- "$@"
fi
