// Mock data used as fallback when not running in Tauri context (browser dev mode)

export interface PersonaSummary {
  name: string;
  description: string;
  version: string;
  active: boolean;
  capabilities: string[];
  installed_at: string;
}

export interface CapabilityScore {
  capability: string;
  score: number;
  delta_7d: number;
}

export interface GrowthEntry {
  timestamp: string;
  event: string;
  delta: number;
}

export interface GrowthReport {
  persona: string;
  total_sessions: number;
  total_tokens_processed: number;
  capability_scores: CapabilityScore[];
  log: GrowthEntry[];
}

export const MOCK_PERSONAS: PersonaSummary[] = [
  {
    name: "security",
    description: "Security-focused persona with threat modeling and audit capabilities",
    version: "0.3.1",
    active: true,
    capabilities: ["threat-model", "audit", "vuln-scan"],
    installed_at: "2026-05-01T00:00:00Z",
  },
  {
    name: "cryptographic",
    description: "Cryptographic systems expert -- key management, protocol design",
    version: "0.2.0",
    active: false,
    capabilities: ["key-derivation", "protocol-review"],
    installed_at: "2026-05-05T00:00:00Z",
  },
  {
    name: "systems",
    description: "Low-level systems programming, kernel interfaces, memory safety",
    version: "0.4.2",
    active: false,
    capabilities: ["memory-analysis", "perf-profiling", "kernel-debug"],
    installed_at: "2026-04-20T00:00:00Z",
  },
  {
    name: "frontend",
    description: "Frontend engineering -- React, accessibility, performance",
    version: "0.1.8",
    active: false,
    capabilities: ["a11y-audit", "bundle-analysis"],
    installed_at: "2026-05-10T00:00:00Z",
  },
];

export const MOCK_ACTIVE_PERSONA = "security";

export function mockGrowthReport(name: string): GrowthReport {
  return {
    persona: name,
    total_sessions: 47,
    total_tokens_processed: 1_284_930,
    capability_scores: [
      { capability: "threat-model", score: 0.82, delta_7d: 0.04 },
      { capability: "audit", score: 0.74, delta_7d: 0.02 },
      { capability: "vuln-scan", score: 0.68, delta_7d: -0.01 },
    ],
    log: [
      {
        timestamp: "2026-05-17T10:00:00Z",
        event: "session completed -- threat model review",
        delta: 0.02,
      },
      {
        timestamp: "2026-05-16T14:30:00Z",
        event: "capability unlocked: advanced-audit",
        delta: 0.05,
      },
      {
        timestamp: "2026-05-15T09:15:00Z",
        event: "session completed -- CVE analysis",
        delta: 0.01,
      },
    ],
  };
}

export const MOCK_MARKETPLACE_PERSONAS = [
  {
    name: "research",
    description: "Academic research assistant -- literature review, citation, synthesis",
    version: "0.5.0",
    author: "ghostframe",
    downloads: 1204,
    tags: ["research", "writing", "analysis"],
  },
  {
    name: "agents",
    description: "Agentic systems design and orchestration patterns",
    version: "0.3.3",
    author: "ghostframe",
    downloads: 892,
    tags: ["agents", "orchestration", "ai"],
  },
  {
    name: "architecture",
    description: "Software architecture, system design, tradeoff analysis",
    version: "0.6.1",
    author: "ghostframe",
    downloads: 2341,
    tags: ["architecture", "design", "systems"],
  },
  {
    name: "rust",
    description: "Rust expert -- lifetimes, async, unsafe, performance",
    version: "0.4.0",
    author: "ghostframe",
    downloads: 3102,
    tags: ["rust", "systems", "performance"],
  },
  {
    name: "lab",
    description: "Experimental persona for testing new capabilities",
    version: "0.1.0",
    author: "ghostframe",
    downloads: 341,
    tags: ["experimental", "lab"],
  },
];
