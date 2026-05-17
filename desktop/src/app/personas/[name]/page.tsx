"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import Link from "next/link";
import { listPersonas, activatePersona, getGrowth } from "@/lib/tauri";
import type { PersonaSummary, GrowthReport } from "@/lib/mock-data";

export default function PersonaDetailPage() {
  const params = useParams();
  const name = typeof params.name === "string" ? params.name : "";

  const [persona, setPersona] = useState<PersonaSummary | null>(null);
  const [growth, setGrowth] = useState<GrowthReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [activating, setActivating] = useState(false);

  useEffect(() => {
    if (!name) return;
    async function load() {
      try {
        const [personas, g] = await Promise.all([
          listPersonas(),
          getGrowth(name),
        ]);
        const found = personas.find((p) => p.name === name) ?? null;
        setPersona(found);
        setGrowth(g);
      } catch (err) {
        console.error("persona detail load error:", err);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, [name]);

  async function handleActivate() {
    if (!persona) return;
    setActivating(true);
    try {
      await activatePersona(persona.name);
      setPersona((prev) => prev ? { ...prev, active: true } : prev);
    } catch (err) {
      console.error("activate error:", err);
    } finally {
      setActivating(false);
    }
  }

  if (loading) {
    return (
      <div>
        <div className="page-header">
          <div className="page-title">{name}</div>
        </div>
        <div className="card-meta">Loading...</div>
      </div>
    );
  }

  if (!persona) {
    return (
      <div>
        <div className="page-header">
          <div className="page-title">Not Found</div>
        </div>
        <div className="card-meta">Persona &quot;{name}&quot; is not installed.</div>
        <Link href="/personas" className="btn" style={{ marginTop: "1rem" }}>
          Back to Personas
        </Link>
      </div>
    );
  }

  return (
    <div>
      <div className="page-header">
        <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", gap: "1rem" }}>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "0.75rem", marginBottom: "0.25rem" }}>
              <Link href="/personas" className="card-meta" style={{ textDecoration: "none" }}>
                Personas
              </Link>
              <span className="card-meta">/</span>
              <div className="page-title">{persona.name}</div>
              {persona.active && <span className="badge badge-active">active</span>}
            </div>
            <div className="page-subtitle">
              v{persona.version} -- installed {new Date(persona.installed_at).toLocaleDateString()}
            </div>
          </div>
          <div style={{ display: "flex", gap: "0.5rem" }}>
            {!persona.active && (
              <button
                className="btn btn-primary"
                onClick={handleActivate}
                disabled={activating}
              >
                {activating ? "Activating..." : "Activate"}
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Description */}
      <div className="card" style={{ marginBottom: "1.5rem" }}>
        <div className="card-title">Description</div>
        <div style={{ fontSize: "0.82rem", color: "var(--sa-text)", lineHeight: 1.6 }}>
          {persona.description}
        </div>
      </div>

      {/* Capabilities */}
      <div className="card" style={{ marginBottom: "1.5rem" }}>
        <div className="card-title">Capabilities</div>
        <div className="persona-caps" style={{ marginTop: "0.5rem" }}>
          {persona.capabilities.map((cap) => (
            <span key={cap} className="badge badge-accent">{cap}</span>
          ))}
        </div>

        {growth && growth.capability_scores.length > 0 && (
          <>
            <hr className="section-divider" />
            <div className="cap-score-list">
              {growth.capability_scores.map((cs) => (
                <div key={cs.capability} className="cap-score-row">
                  <div className="cap-score-header">
                    <span className="cap-score-name">{cs.capability}</span>
                    <div style={{ display: "flex", gap: "0.75rem", alignItems: "center" }}>
                      <span
                        className={`stat-delta ${cs.delta_7d >= 0 ? "stat-delta-pos" : "stat-delta-neg"}`}
                        style={{ fontSize: "0.68rem" }}
                      >
                        {cs.delta_7d >= 0 ? "+" : ""}{(cs.delta_7d * 100).toFixed(1)}% 7d
                      </span>
                      <span className="cap-score-value">{(cs.score * 100).toFixed(0)}%</span>
                    </div>
                  </div>
                  <div className="progress-bar-track">
                    <div
                      className="progress-bar-fill"
                      style={{ width: `${cs.score * 100}%` }}
                    />
                  </div>
                </div>
              ))}
            </div>
          </>
        )}
      </div>

      {/* Growth stats */}
      {growth && (
        <>
          <div className="stat-grid" style={{ marginBottom: "1.5rem" }}>
            <div className="stat-card">
              <div className="stat-label">Total Sessions</div>
              <div className="stat-value">{growth.total_sessions}</div>
            </div>
            <div className="stat-card">
              <div className="stat-label">Tokens Processed</div>
              <div className="stat-value" style={{ fontSize: "1.1rem" }}>
                {(growth.total_tokens_processed / 1_000_000).toFixed(2)}M
              </div>
            </div>
          </div>

          {/* Growth log */}
          {growth.log.length > 0 && (
            <div>
              <div
                className="page-subtitle"
                style={{ fontSize: "0.72rem", letterSpacing: "0.12em", textTransform: "uppercase", marginBottom: "0.75rem" }}
              >
                Growth Log
              </div>
              <div className="growth-log">
                {growth.log.map((entry, i) => (
                  <div key={i} className="growth-log-entry">
                    <span className="growth-log-time">
                      {new Date(entry.timestamp).toLocaleDateString()}
                    </span>
                    <span className="growth-log-event">{entry.event}</span>
                    <span
                      className={`growth-log-delta ${entry.delta >= 0 ? "stat-delta-pos" : "stat-delta-neg"}`}
                    >
                      {entry.delta >= 0 ? "+" : ""}{(entry.delta * 100).toFixed(1)}%
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}
