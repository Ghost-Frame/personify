<p align="center">
  <img src="assets/banner.png" alt="Frameshift" width="100%" />
</p>

# Frameshift

<!-- TODO(frameshift): This file is the persona-library overview, NOT the "deep product
     writeup" the main READMEs used to link to. That deep-dive doesn't exist yet. When it's
     written (here or as its own doc), re-add the "deep product writeup" / "Further reading"
     links in both README.md and server/README.md -- they were removed 2026-05-21 because
     they pointed at a writeup that wasn't real. -->

**Same model. Different frame.**

Activate the cryptographer and you get a spec-anchored operator who refuses to invent primitives. Switch to systems and you get a paranoid engineer who state-checks before touching anything and has a rollback ready. Switch to writer and you get a technical editor who deletes a sentence before adding one. Same model. Different frame.

Each frame is a complete behavioral identity. Not a list of instructions. A coherent stance that survives long sessions, surprising inputs, and the slow drift that turns careful operators into sloppy ones around turn 200.

## What Frameshift is

A marketplace and runtime for versioned, composable behavioral personas for AI coding agents.

- **Typed source format.** Personas are structured TOML -- not freeform markdown. Markdown is a render target, generated per-agent (Claude, Codex, Gemini).
- **CLI.** `frameshift install`, `activate`, `sync`, `gc`. Manages a central store outside your project tree. Your repo never gets persona files.
- **Signed packs.** Content-addressed, Ed25519-signed tarballs. Deterministic canonicalization for reproducible hashes.
- **Composition.** Extend a base persona, mix in overlays. Conflict detection at install time.
- **Marketplace server.** Catalog, version resolution, distribution.

## Persona source format

Personas are three TOML files, not a single markdown document:

```toml
# persona.toml
schema_version = 1
name = "cryptographic"
voice = "citation-driven, careful, willing to say I don't know"

[anchor.l2]
text = "You are working on cryptographic primitives, verifying not inventing"

[[default_questions]]
question = "Which specification or RFC governs this code?"
```

```toml
# rules.toml
[[rule]]
id = "no-rolling-crypto"
layer = "L1"
text = "Never roll a new cryptographic primitive when an audited implementation exists."

[[rule]]
id = "prefer-rfc-citations"
layer = "L3"
text = "Cite the governing RFC when discussing protocol behavior."
```

```toml
# skills.toml
[[skill]]
id = "test-driven-development"
invoke_when = "All cryptographic implementations -- tests BEFORE code"
```

The renderer projects this typed source into per-target markdown (Claude, Codex, Gemini, generic). Patch operations (`frameshift rule add`, `frameshift skill remove`) replace hand-editing. Semantic diffs show typed changes between versions -- not text diffs.

## Installation

```bash
frameshift install cryptographic@0.3.1
frameshift activate cryptographic
```

All state lives in `$XDG_DATA_HOME/frameshift/`:

```
cache/<sha256>/                   # Content-addressed pack cache, shared across projects
projects/<project-id>/
  lock.toml                       # Installed personas, versions, hashes
  active                          # Currently active persona
  personas/<name>/
    source/                       # Pack contents
    rendered/{claude,codex,gemini,generic}/
    growth.md                     # Local-only, append-only
```

Project ID is `sha256(realpath(project_root))`. Your project tree is never written to.

## Pack format

Each persona distributes as a signed pack:

```toml
# pack.toml
schema_version = 1
name = "cryptographic"
version = "0.3.1"
author_handle = "ghost-frame"
author_pubkey = "ed25519:<hex>"
license = "AGPL-3.0"

[capability_manifest]
required_tools = ["Read", "Edit", "Bash"]
filesystem_scope = "project-only"
network_egress = false

[conformance_baseline]
score = 0.92
bundle_hash = "sha256:..."
```

Packs are tarballs, canonicalized via recursive dir walk with unicode normalization, SHA256-hashed, Ed25519-signed. The capability manifest declares what tools and access the persona needs. The conformance baseline gates upgrades -- a newer version must meet the score floor.

## Composition

Personas can extend a base and mix in overlays:

```toml
extends = "base-persona@^1"
mixin = ["company-style@2.x", "safety-overlay@1.x"]
```

Resolution order: base -> mixins (in order) -> root persona. Conflicting rule IDs surface at install time and require explicit overrides.

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

## Why frames beat instruction lists

Most agent prompts read like ranked priority lists. These drift fast. Under pressure -- long sessions, surprising inputs, multi-step debugging -- the model treats lower-ranked items as optional, then irrelevant.

Schubert's behavioral architecture work (SFP-2, the L1/L2/L3 distinction) found that identity held as a coherent stance survives that pressure where ranked lists collapse. **"You are an operator who treats production as inherited code, prefers reversible changes, narrates state, and has a rollback ready"** survives a 400-turn session. *"1. Check state. 2. Be careful. 3. Have a rollback"* does not.

Every frame is built on that L2 anchor:

- **L2 semantic framing.** Identity as coherent stance. The first sentence names who the operator is, not what they do.
- **Cascade anchors.** Re-anchor at top, middle, and end. Drift propagates upward through context; redundancy at multiple positions beats thoroughness at one.
- **L1 hard constraints.** Never-do rules with reasoning attached. Scar tissue from real incidents.
- **Forced classification.** Each frame declares a judgment axis. The agent classifies before acting. The classification is the design pressure.
- **Self-evaluation hooks.** Checklist before non-trivial actions.
- **Growth.** Frame is read-only. Growth log is append-only. Next session reads both.

## Growth

Growth is local. A single append-only file per installed persona, stored in the central store. Sessions deposit findings. Future sessions read them back. Growth never flows upstream -- it stays on your machine, in your project context.

## References

- Schubert, J. (2026). *AIReason LLM Behavioral Architecture.* https://doi.org/10.5281/zenodo.19157027
- Schubert, J. (2026). *System Frame Persistency (SFP-2).* https://doi.org/10.5281/zenodo.19154800
- Schubert, J. (2026). *Structural Transformations in Multi-Stage Dialogues.* https://doi.org/10.5281/zenodo.18843970
- Schubert, J. (2026). *SL-20: Safety-Layer Frequency Analysis.* https://doi.org/10.5281/zenodo.18143850
