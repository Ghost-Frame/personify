#!/usr/bin/env bash
set -euo pipefail

AGENTS_DIR="${HOME}/.claude/agents"
ARCHETYPES_DIR="${HOME}/projects/archetypes"

# archetype -> agent-name mapping
declare -A MAPPING=(
    [agents]="agent-designer"
    [architecture]="architect"
    [bots]="bot-developer"
    [creative]="creative-coder"
    [cryptographic]="crypto-engineer"
    [data]="data-engineer"
    [desktop]="desktop-developer"
    [devops]="devops-engineer"
    [devtools]="devtools-builder"
    [frontend]="frontend-dev"
    [gatekeeper]="gatekeeper"
    [lab]="lab-experimenter"
    [memory]="memory-architect"
    [performance]="performance-analyst"
    [research]="researcher"
    [reviewer]="code-reviewer"
    [rust]="rust-engineer"
    [security]="security-analyst"
    [systems]="systems-ops"
    [testing]="test-engineer"
    [unreal]="unreal-developer"
    [writer]="technical-writer"
)

# Gatekeeper default frontmatter (no existing file)
GATEKEEPER_FRONTMATTER="---
name: gatekeeper
description: Gatekeeper archetype. Access control, permission enforcement, trust boundaries.
model: sonnet
effort: high
tools: Read, Edit, Write, Bash, Grep, Glob
---"

count=0

for context in "${!MAPPING[@]}"; do
    agent_name="${MAPPING[$context]}"
    source_agents_md="${ARCHETYPES_DIR}/${context}/AGENTS.md"
    target_file="${AGENTS_DIR}/${agent_name}.md"

    if [[ ! -f "$source_agents_md" ]]; then
        echo "[${context} -> ${agent_name}.md] SKIP (no AGENTS.md found)"
        continue
    fi

    body="$(cat "$source_agents_md")"

    if [[ "$context" == "gatekeeper" ]]; then
        # New file -- use default frontmatter
        printf '%s\n\n%s\n' "$GATEKEEPER_FRONTMATTER" "$body" > "$target_file"
        echo "[${context} -> ${agent_name}.md] CREATED"
    else
        if [[ ! -f "$target_file" ]]; then
            echo "[${context} -> ${agent_name}.md] SKIP (target agent file not found and not gatekeeper)"
            continue
        fi

        # Extract frontmatter: lines between first and second ---
        # awk: print from first --- through and including second ---
        frontmatter="$(awk '
            /^---$/ {
                count++
                print
                if (count == 2) exit
                next
            }
            count == 1 { print }
        ' "$target_file")"

        printf '%s\n\n%s\n' "$frontmatter" "$body" > "$target_file"
        echo "[${context} -> ${agent_name}.md] OK"
    fi

    (( count++ )) || true
done

echo ""
echo "Synced ${count} archetype files."
