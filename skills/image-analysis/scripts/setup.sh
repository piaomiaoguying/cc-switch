#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONFIG_FILE="$SCRIPT_DIR/.skill_config"
DEFAULT_VENV="$HOME/.venvs/image-analysis"
REQUIREMENTS="$SCRIPT_DIR/requirements.txt"

# Parse args
VENV_PATH=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --venv) VENV_PATH="$2"; shift 2 ;;
    *) echo "Unknown arg: $1" >&2; exit 1 ;;
  esac
done

# Interactive prompt if not provided via --venv
if [[ -z "$VENV_PATH" ]]; then
  echo "=== Image Analysis Skill Setup ==="
  echo ""
  echo "Where should the Python virtual environment be created?"
  echo "  [Enter] for default: $DEFAULT_VENV"
  echo "  Or type a custom path:"
  read -r -p "> " input
  VENV_PATH="${input:-$DEFAULT_VENV}"
fi

# Expand ~ if present
VENV_PATH="${VENV_PATH/#\~/$HOME}"

echo ""
echo "Creating virtual environment at: $VENV_PATH"
python3 -m venv "$VENV_PATH"

echo "Installing dependencies..."
"$VENV_PATH/bin/pip" install --upgrade pip -q
"$VENV_PATH/bin/pip" install -r "$REQUIREMENTS" -q

echo "Saving config..."
echo "VENV_PATH=$VENV_PATH" > "$CONFIG_FILE"

echo ""
echo "Setup complete. Config saved to: $CONFIG_FILE"
echo "Virtual environment: $VENV_PATH"
