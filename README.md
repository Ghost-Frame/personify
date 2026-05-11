# custom-agents

Per-context persona files for Claude Code. Each subdirectory is a self-contained "context" with its own behavioral identity, hard constraints, and growth log. When Claude Code runs in a given subdirectory, it loads that directory's `AGENTS.md` as the active persona.

The design is based on Schubert's L2 semantic-framing research -- personas held as a coherent identity rather than a ranked list of priorities. Each `AGENTS.md` follows the same structural template: anchor, operating frame, hard constraints (L1), concrete patterns, mid-document re-anchor, conflict resolution as a semantic frame, self-evaluation hooks, recency anchor.

## Layout

```
<context>/
  AGENTS.md   # The persona. Read on session start.
  GROWTH.md   # Running observations log. Appended during sessions.
  sync.sh     # Validation/sync wrapper (expects an external helper).
```

## Contexts

| Directory | What lives here |
|---|---|
| `agents/` | Agent design -- personas, growth mechanics, supervision, multi-agent dynamics |
| `architecture/` | Architecture review and design -- skeptical posture, stress-testing proposals |
| `bots/` | Discord bot development -- botcore, Bun, discord.js, character fidelity |
| `creative/` | Creative coding and generative art -- aesthetic judgment, surprise |
| `cryptographic/` | Cryptographic implementation -- RustCrypto, spec-anchored, test vectors |
| `data/` | Data pipelines and vector indexing -- idempotent, observable, recoverable |
| `desktop/` | Desktop and TUI applications -- Tauri, ratatui, wgpu |
| `devops/` | Deployment pipelines -- CI/CD, staged rollouts, rollback paths |
| `devtools/` | Developer tooling -- CLIs, analyzers, build systems |
| `frontend/` | Frontend implementation -- SvelteKit, Astro, Tailwind, TypeScript |
| `gatekeeper/` | Pre-publication scrubbing -- secrets, identity, infra, methodology classification |
| `lab/` | Proof-of-concept experiments -- speed over polish, findings over artifacts |
| `memory/` | Knowledge systems -- vector search, embedding pipelines, knowledge graphs |
| `performance/` | Profiling and optimization -- benchmark before claiming, data over intuition |
| `research/` | Codebase archaeology and synthesis -- source-cited claims only |
| `reviewer/` | Multi-concern code review -- correctness, security, performance, style, docs |
| `rust/` | Rust implementation -- Axum, Tokio, thiserror, tracing, clippy-strict |
| `security/` | Security analysis -- opsec-first, noise-level classification |
| `systems/` | Infrastructure operations -- blast-radius classification, reversible changes |
| `testing/` | QA and test engineering -- strategy, coverage, flaky-test debugging |
| `unreal/` | Unreal Engine development -- Blueprint + C++ hybrid, engine-native patterns |
| `writer/` | Technical writing -- anti-slop, every sentence earns its place |

## Patterns shared across contexts

- **L2 semantic framing.** Personas are held as a coherent identity ("You are an operator who treats production as inherited code, prefers reversible changes, narrates state, has a rollback ready"). Not as a ranked priority list -- those drift under conversational pressure.
- **Cascade anchors.** Each persona has anchor passages at the top, middle, and bottom. Drift cascades upward, so redundancy reduces propagation.
- **Hard constraints (L1).** Each context has explicit never-do rules with reasoning. These survive context erosion better than soft guidance.
- **Blast-radius / capability-tier classification.** Each context forces a declared classification on actions (LOCAL/SERVICE/HOST/FLEET/GLOBAL for systems, AUTONOMOUS/ASSISTED/CONSTRAINED/TOY for agents, etc.). Forcing the declaration is the design pressure.
- **Self-evaluation hooks.** Each persona ends with a short checklist the agent runs before non-trivial actions.
- **Growth integration.** Each context has a `GROWTH.md` that the agent reads on session start and appends to during the session. The persona file is the soul; the growth log is the substrate.

## Note on `sync.sh`

The `sync.sh` in each context wraps a shared validation helper not included in this repo. The persona files themselves are the substantive content; the sync wrapper is plumbing.

## References

The L1/L2/L3 framing, cascade anchors, and Safety-Layer Frequency work draw from:

- Schubert, J. (2026). *AIReason LLM Behavioral Architecture.* https://doi.org/10.5281/zenodo.19157027
- Schubert, J. (2026). *System Frame Persistency (SFP-2).* https://doi.org/10.5281/zenodo.19154800
- Schubert, J. (2026). *Structural Transformations in Multi-Stage Dialogues -- Runport.* https://doi.org/10.5281/zenodo.18843970
- Schubert, J. (2026). *SL-20 -- Safety-Layer Frequency Analysis.* https://doi.org/10.5281/zenodo.18143850
