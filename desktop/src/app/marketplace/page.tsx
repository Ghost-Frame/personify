"use client";

import { useState } from "react";
import { installPersona } from "@/lib/tauri";
import { MOCK_MARKETPLACE_PERSONAS } from "@/lib/mock-data";

export default function MarketplacePage() {
  const [installing, setInstalling] = useState<string | null>(null);
  const [installed, setInstalled] = useState<Set<string>>(new Set());
  const [search, setSearch] = useState("");

  const filtered = MOCK_MARKETPLACE_PERSONAS.filter(
    (p) =>
      p.name.includes(search.toLowerCase()) ||
      p.description.toLowerCase().includes(search.toLowerCase()) ||
      p.tags.some((t) => t.includes(search.toLowerCase()))
  );

  async function handleInstall(name: string) {
    setInstalling(name);
    try {
      await installPersona(name, `frameshift://marketplace/${name}`);
      setInstalled((prev) => new Set(prev).add(name));
    } catch (err) {
      console.error("install error:", err);
    } finally {
      setInstalling(null);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div className="page-title">Marketplace</div>
        <div className="page-subtitle">Browse and install community personas</div>
      </div>

      {/* Search */}
      <div style={{ marginBottom: "1.5rem" }}>
        <input
          type="text"
          placeholder="Search personas..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{
            width: "100%",
            maxWidth: "400px",
            padding: "0.5rem 0.75rem",
            background: "var(--sa-surface)",
            border: "1px solid var(--sa-border)",
            borderRadius: "6px",
            color: "var(--sa-text)",
            fontFamily: "inherit",
            fontSize: "0.82rem",
            outline: "none",
          }}
          onFocus={(e) => {
            e.currentTarget.style.borderColor = "var(--sa-accent)";
          }}
          onBlur={(e) => {
            e.currentTarget.style.borderColor = "var(--sa-border)";
          }}
        />
      </div>

      {filtered.length === 0 ? (
        <div className="card-meta">No personas match your search.</div>
      ) : (
        <div className="marketplace-grid">
          {filtered.map((persona) => {
            const isInstalled = installed.has(persona.name);
            const isInstalling = installing === persona.name;
            return (
              <div key={persona.name} className="marketplace-card">
                <div>
                  <div className="marketplace-card-name">{persona.name}</div>
                  <div className="marketplace-card-meta">
                    <span>by {persona.author}</span>
                    <span>v{persona.version}</span>
                    <span>{persona.downloads.toLocaleString()} installs</span>
                  </div>
                </div>

                <div style={{ fontSize: "0.78rem", color: "var(--sa-text)", lineHeight: 1.5 }}>
                  {persona.description}
                </div>

                <div className="marketplace-tags">
                  {persona.tags.map((tag) => (
                    <span key={tag} className="marketplace-tag">{tag}</span>
                  ))}
                </div>

                <div style={{ marginTop: "auto" }}>
                  {isInstalled ? (
                    <span className="badge badge-active">installed</span>
                  ) : (
                    <button
                      className="btn btn-sm btn-primary"
                      onClick={() => handleInstall(persona.name)}
                      disabled={isInstalling}
                    >
                      {isInstalling ? "Installing..." : "Install"}
                    </button>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
