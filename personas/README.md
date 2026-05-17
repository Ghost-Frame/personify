<p align="center">
  <img src="assets/banner.png" alt="Frameshift" width="100%" />
</p>

# Frameshift

**Same model. Different frame.**

Activate the cryptographer and you get a spec-anchored operator who refuses to invent primitives. Switch to systems and you get a paranoid engineer who state-checks before touching anything and has a rollback ready. Switch to writer and you get a technical editor who deletes a sentence before adding one. Same model. Different frame.

Each frame is a complete behavioral identity. Not a list of instructions. A coherent stance that survives long sessions, surprising inputs, and the slow drift that turns careful operators into sloppy ones around turn 200.

## How it works

A frame is an `AGENTS.md` file. Coding agents that honor this convention (Claude Code, Cursor, Windsurf, Codex, and others) auto-load it on session start. Each frame follows the same structure:

```
<frame>/
  AGENTS.md   # The frame. Loaded on session start.
  GROWTH.md   # Running observations log. Appended during sessions.
  sync.sh     # Validation wrapper. Delegates to bin/sync-context.sh.
```

**Two ways to use a frame:**

1. **Drop it in your project.** Copy a frame's `AGENTS.md` into your project root. Your agent loads it on the next session. One project, one frame.

2. **Install as a switchable agent.** Use `bin/sync-agents.sh` to install frames as Claude Code custom agents (under `~/.claude/agents/`). Switch frames on demand without changing your project files.

`GROWTH.md` is where sessions deposit findings ("don't trust this library's docs, read the tests instead") and where future sessions read them back. The frame is the identity. The growth log is the memory.

## Why frames beat instruction lists

Most agent prompts read like ordered priority lists: "First do X. Then Y. Most importantly Z." These drift fast. Under pressure (long sessions, surprising inputs, multi-step debugging) the model starts treating the lower-ranked items as optional, then irrelevant.

Schubert's behavioral architecture work (SFP-2, the L1/L2/L3 distinction) found that identity held as a coherent stance survives that pressure where ranked lists collapse. **"You are an operator who treats production as inherited code, prefers reversible changes, narrates state, and has a rollback ready"** survives a 400-turn session. *"1. Check state. 2. Be careful. 3. Have a rollback"* does not.

Every frame here is built on that L2 anchor. Cascade anchors at top, middle, and end of each file fight upward-propagating drift. L1 hard constraints encode scar tissue from real incidents. Forced classification axes make the agent declare its judgment before acting. The act of classifying is the design pressure.

## Frames

| Frame | What wakes up |
|---|---|
| `agents/` | Agent designer. Personas, growth, supervision, multi-agent loops |
| `architecture/` | Skeptical architect. Stress-tests proposals before they cost anything to fix |
| `bots/` | Discord bot personality engineer. Character fidelity across thousands of turns |
| `creative/` | Creative coder. Aesthetic judgment over convention |
| `cryptographic/` | Cryptographer. Spec-anchored, constant-time aware, never invents primitives |
| `data/` | Data engineer. Idempotent, observable, recoverable pipelines |
| `desktop/` | Desktop and TUI engineer. Tauri, ratatui, wgpu, native feel over web-wrapper convenience |
| `devops/` | Deployment engineer. Staged rollouts, named rollback paths, fleet-wide awareness |
| `devtools/` | Tooling builder. Developer experience as the product |
| `frontend/` | Frontend engineer. SvelteKit, Astro, Tailwind, no component library sludge |
| `gatekeeper/` | Paranoid gatekeeper. Classifies before it lets anything cross the public boundary |
| `lab/` | Experimenter. Speed over polish, findings over artifacts |
| `memory/` | Memory architect. Vector search, embedding pipelines, recall fidelity over latency |
| `performance/` | Performance analyst. Profiles before optimizing, benchmarks before claiming |
| `research/` | Source-grounded researcher. Refuses to paraphrase from training-data memory |
| `reviewer/` | Code reviewer. Five lenses: correctness, security, performance, style, documentation |
| `rust/` | Rust engineer. Idiomatic, clippy-strict, no unwraps in library code |
| `security/` | Security analyst. Opsec-first, classifies by noise level |
| `systems/` | Operator with steady hands. State-check first, change second, verify third |
| `testing/` | QA engineer. Finds the test that matters |
| `unreal/` | Unreal developer. Blueprint plus C++ hybrid. Verifies API names before using them |
| `writer/` | Technical editor. Every sentence earns its place |

## The toolkit

Six patterns repeat across every frame:

- **L2 semantic framing.** Identity as coherent stance, not ranked priorities. The first sentence of every frame names who the operator is, not what they do.
- **Cascade anchors.** Re-anchor at top, middle, and end. Drift propagates upward through context, so redundancy at multiple positions matters more than thoroughness at one.
- **L1 hard constraints.** Never-do rules with the reasoning attached. Scar tissue from real incidents. These survive context erosion better than soft guidance.
- **Forced classification.** Each frame declares an axis (LOCAL/SERVICE/HOST/FLEET/GLOBAL for systems, AUTONOMOUS/ASSISTED/CONSTRAINED/TOY for agents, ON-CHARACTER/DRIFTING/OFF-CHARACTER/BROKEN for bots). The agent must classify before acting. The classification is the design pressure.
- **Self-evaluation hooks.** Each frame ends with a short checklist run before non-trivial actions.
- **Growth integration.** The frame is read-only at session start. The growth log is append-only during the session. Next session reads both.

## Getting started

Pick the frames that match your work. Two paths:

**Path 1: Drop into your project.**
Copy a frame's `AGENTS.md` into your project root. Any AGENTS.md-aware agent (Claude Code, Cursor, Windsurf, Codex) reads it on session start. One project, one frame.

**Path 2: Install as switchable agents.**
Run `bin/sync-agents.sh` to install all frames as Claude Code custom agents under `~/.claude/agents/`. Switch between them on demand during a session.

Either way, tune the L1 rules to your own scar tissue. Every team has a "we tried this once and it took three days to recover" list. The framework is the structure. The content is yours.

Each frame's `sync.sh` runs the bundled validator at `bin/sync-context.sh`. Edit the frame, run `./sync.sh` from inside the frame directory, and it checks the cascade anchors, the L1 rule block, the required vocabulary, the minimum line count, and broken relative links. No setup, no env vars, no external dependencies beyond `bash`.

## Customizing for your environment

The frames reference a few tools by name. These are the ones we use; replace them with your equivalents or remove the references entirely.

| Reference in frames | What it does | Replace with |
|---|---|---|
| `$MEMORY_CLI store` | Persists findings to a searchable memory store | Your note system, a SQLite DB, a markdown file, or remove |
| `the structured dev workflow` workflow (`spec_task`, `challenge_code`, etc.) | Structured development workflow with pre/post hooks | Your team's PR checklist, or remove and rely on L1 rules alone |
| `cred get` / `cred exec` | Credential retrieval without hardcoding secrets | `pass`, `1password-cli`, `vault`, environment variables, or remove |
| `~/specs/` | Directory where design docs live outside project repos | Wherever your specs live |

The **Growth Integration** section in each frame is optional. If you do not have a persistent memory store, you can still use `GROWTH.md` as a plain file. The frame reads it on session start and appends during the session. No external tooling required.

The **Required Skills** tables reference skills that may not exist in your environment. The validator warns but does not fail on missing skills. Replace skill names with your own or remove the table. The frame works without it.

## References

The L1/L2/L3 framing, cascade anchors, and Safety-Layer Frequency work draw from Schubert:

- *AIReason LLM Behavioral Architecture.* https://doi.org/10.5281/zenodo.19157027
- *System Frame Persistency (SFP-2).* https://doi.org/10.5281/zenodo.19154800
- *Structural Transformations in Multi-Stage Dialogues: The Runport Study.* https://doi.org/10.5281/zenodo.18843970
- *SL-20: Safety-Layer Frequency Analysis.* https://doi.org/10.5281/zenodo.18143850
