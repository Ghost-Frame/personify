# Growth Log -- Gatekeeper Context

Accumulated learnings about sensitive content patterns, false positives, classification edge cases, and scrub procedure refinements. Read at session start. Append observations as they emerge -- do not wait for session end.

Format: `- [YYYY-MM-DD] Observation. What changed in your understanding.`

---

<!-- Append observations below this line -->
- [2026-05-12] CLAUDE.md is a silent HIGH-tier leak vector: it contains internal IPs, archetype config, and API details. It is never auto-ignored. Every project repo needs CLAUDE.md + AGENTS.md in .gitignore as a baseline.
- [2026-05-12] CI workflows (publish.yml, docker-publish.yml) are worth scanning specifically for hardcoded account names -- `username:` fields in docker/login-action steps are not secrets, they're plaintext in the workflow file.
- [2026-05-12] Commit author emails may already be on public GitHub via other repos. History rewrite would be needed to change this. Flag as accepted risk or remediate before any future account separation is needed.
