#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SKILL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_FILE="$SCRIPT_DIR/.skill_config"

if [[ ! -f "$CONFIG_FILE" ]]; then
  echo "Error: Config not found. Run 'scripts/setup.sh' first." >&2
  exit 1
fi

source "$CONFIG_FILE"

if [[ -z "${VENV_PATH:-}" ]]; then
  echo "Error: VENV_PATH not set in $CONFIG_FILE" >&2
  exit 1
fi

PYTHON="$VENV_PATH/bin/python3"

if [[ ! -x "$PYTHON" ]]; then
  echo "Error: Python not found at $PYTHON. Run 'scripts/setup.sh' to recreate." >&2
  exit 1
fi

cd "$SKILL_DIR"

# Force arm64 on Apple Silicon to avoid arch mismatch with universal Python binaries
if [[ "$(uname -s)" == "Darwin" ]] && sysctl -n hw.optional.arm64 2>/dev/null | grep -q 1; then
  exec arch -arm64 "$PYTHON" "$@"
else
  exec "$PYTHON" "$@"
fi
