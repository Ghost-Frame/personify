#!/bin/bash
# Per-context sync wrapper. Delegates to the shared sync library.
# Edit AGENTS.md, then run this to validate.

set -euo pipefail

CTX_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export CTX_DIR
CONTEXT_PHRASES=("fidelity tier" "PRIMARY-SOURCE" "cite" "training-data memory")
export CONTEXT_PHRASES

source "${AGENT_CONFIG_DIR:-$HOME/.agent-config}/claude/bin/sync-context.sh"
