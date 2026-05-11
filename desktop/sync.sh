#!/bin/bash
set -euo pipefail
CTX_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export CTX_DIR
CONTEXT_PHRASES=("native" "Tauri" "TUI" "rendering")
export CONTEXT_PHRASES
source "${AGENT_CONFIG_DIR:-$HOME/.agent-config}/claude/bin/sync-context.sh"
