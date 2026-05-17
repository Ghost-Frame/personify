#!/usr/bin/env bash
# verify-no-leaks.sh -- Pre-push leak scan for the public personas repo (Ghost-Frame/personify).
# Scans all git-tracked files for known sensitive patterns.
# Exit 0 = clean. Exit 1 = findings detected.
#
# Usage: verify-no-leaks.sh [--quiet]
#   --quiet : suppress per-file output, only print summary and exit code

set -euo pipefail

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -P "$SCRIPT_DIR/.." && pwd)"
ALLOWED_FILE="${REPO_ROOT}/bin/allowed-patterns.txt"
PATTERNS_FILE="${REPO_ROOT}/bin/leak-patterns.txt"

# ---------------------------------------------------------------------------
# Load sensitive patterns from gitignored file (ERE, passed to grep -E)
# ---------------------------------------------------------------------------
if [[ ! -f "$PATTERNS_FILE" ]]; then
  echo "WARNING: No patterns file found at bin/leak-patterns.txt -- no patterns to scan for." >&2
  echo "Create bin/leak-patterns.txt with one ERE pattern per line to enable scanning." >&2
  exit 0
fi

declare -a PATTERNS=()
while IFS= read -r line; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  PATTERNS+=("$line")
done < "$PATTERNS_FILE"

if [[ ${#PATTERNS[@]} -eq 0 ]]; then
  echo "WARNING: bin/leak-patterns.txt exists but contains no patterns -- nothing to scan for." >&2
  exit 0
fi

# Build a single alternation pattern for grep
GREP_PATTERN="$(printf '%s|' "${PATTERNS[@]}")"
GREP_PATTERN="${GREP_PATTERN%|}"  # strip trailing |

QUIET=false
for arg in "$@"; do
  case "$arg" in
    --quiet) QUIET=true ;;
    *) echo "Unknown flag: $arg" >&2; exit 1 ;;
  esac
done

# ---------------------------------------------------------------------------
# Load allowed exceptions
# Lines in allowed-patterns.txt are fixed strings; any finding line that
# contains an allowed string is suppressed.
# ---------------------------------------------------------------------------
declare -a ALLOWED=()
if [[ -f "$ALLOWED_FILE" ]]; then
  while IFS= read -r line; do
    [[ -z "$line" || "$line" == \#* ]] && continue
    ALLOWED+=("$line")
  done < "$ALLOWED_FILE"
fi

is_allowed() {
  local finding="$1"
  local a
  for a in "${ALLOWED[@]}"; do
    if [[ "$finding" == *"$a"* ]]; then
      return 0
    fi
  done
  return 1
}

# ---------------------------------------------------------------------------
# Scan
# ---------------------------------------------------------------------------
cd "$REPO_ROOT"

# Collect tracked files, excluding the scanner, its allowlist, and the patterns file
# (all three contain pattern strings by definition)
mapfile -t TRACKED_FILES < <(git ls-files | grep -vE '^bin/(verify-no-leaks\.sh|allowed-patterns\.txt|leak-patterns\.txt)$')

if [[ ${#TRACKED_FILES[@]} -eq 0 ]]; then
  echo "No tracked files found." >&2
  exit 0
fi

findings=0
suppressed=0

while IFS= read -r raw_finding; do
  [[ -z "$raw_finding" ]] && continue
  if is_allowed "$raw_finding"; then
    (( suppressed++ )) || true
    continue
  fi
  (( findings++ )) || true
  if ! $QUIET; then
    printf 'LEAK: %s\n' "$raw_finding"
  fi
done < <(
  printf '%s\n' "${TRACKED_FILES[@]}" \
    | xargs grep -nE "$GREP_PATTERN" 2>/dev/null \
    || true
)

echo ""
echo "--- verify-no-leaks summary ---"
echo "Files scanned    : ${#TRACKED_FILES[@]}"
echo "Findings         : $findings"
echo "Suppressed       : $suppressed (via allowed-patterns.txt)"

if [[ $findings -gt 0 ]]; then
  echo "STATUS: FAIL -- sensitive content detected"
  exit 1
else
  echo "STATUS: CLEAN"
  exit 0
fi
