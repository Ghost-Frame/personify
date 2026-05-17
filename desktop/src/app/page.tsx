"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { listPersonas, activePersona, getGrowth } from "@/lib/tauri";
import type { PersonaSummary, GrowthReport } from "@/lib/mock-data";

export default function DashboardPage() {
  const [personas, setPersonas] = useState<PersonaSummary[]>([]);
  const [active, setActive] = useState<string | null>(null);
  const [growth, setGrowth] = useState<GrowthReport | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const [p, a] = await Promise.all([listPersonas(), activePersona()]);
        setPersonas(p);
        setActive(a);
        if (a) {
          const g = await getGrowth(a);
          setGrowth(g);
        }
      } catch (err) {
        console.error("dashboard load error:", err);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const activePersonaData = personas.find((p) => p.name === active);

  if (loading) {
    return (
      <div>
        <div className="page-header">
          <div className="page-title">Dashboard</div>
        </div>
        <div className="card-meta">Loading...</div>
      </div>
    );
  }

  return (
    <div>
      <div className="page-header">
        <div className="page-title">Dashboard</div>
        <div className="page-subtitle">Overview of your active persona and recent activity</div>
      </div>

      {/* Stat row */}
      <div className="stat-grid">
        <div className="stat-card">
          <div className="stat-label">Installed Personas</div>
          <div className="stat-value">{personas.length}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Active Persona</div>
          <div className="stat-value" style={{ fontSize: "1.1rem" }}>
            {active ?? "none"}
          </div>
        </div>
        {growth && (
          <>
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
          </>
        )}
      </div>

      {/* Active persona detail */}
      {activePersonaData && (
        <div style={{ marginBottom: "2rem" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "1rem" }}>
            <div className="page-subtitle" style={{ fontSize: "0.72rem", letterSpacing: "0.12em", textTransform: "uppercase" }}>
              Active Persona
            </div>
            <Link href={`/personas/${activePersonaData.name}`} className="btn btn-sm">
              View Detail
            </Link>
          </div>
          <div className={`persona-card is-active`}>
            <div className="persona-card-header">
              <div>
                <div className="persona-name">{activePersonaData.name}</div>
                <div className="persona-version">v{activePersonaData.version}</div>
              </div>
              <span className="badge badge-active">active</span>
            </div>
            <div className="persona-description">{activePersonaData.description}</div>
            <div className="persona-caps">
              {activePersonaData.capabilities.map((cap) => (
                <span key={cap} className="persona-cap-tag">{cap}</span>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Capability scores */}
      {growth && growth.capability_scores.length > 0 && (
        <div style={{ marginBottom: "2rem" }}>
          <div className="page-subtitle" style={{ fontSize: "0.72rem", letterSpacing: "0.12em", textTransform: "uppercase", marginBottom: "1rem" }}>
            Capability Scores
          </div>
          <div className="card">
            <div className="cap-score-list">
              {growth.capability_scores.map((cs) => (
                <div key={cs.capability} className="cap-score-row">
                  <div className="cap-score-header">
                    <span className="cap-score-name">{cs.capability}</span>
                    <span className="cap-score-value">{(cs.score * 100).toFixed(0)}%</span>
                  </div>
                  <div className="progress-bar-track">
                    <div
                      className="progress-bar-fill"
                      style={{ width: `${cs.score * 100}%` }}
                    />
                  </div>
                  <div className="stat-delta">
                    <span className={cs.delta_7d >= 0 ? "stat-delta-pos" : "stat-delta-neg"}>
                      {cs.delta_7d >= 0 ? "+" : ""}{(cs.delta_7d * 100).toFixed(1)}% 7d
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Recent growth log */}
      {growth && growth.log.length > 0 && (
        <div>
          <div className="page-subtitle" style={{ fontSize: "0.72rem", letterSpacing: "0.12em", textTransform: "uppercase", marginBottom: "1rem" }}>
            Recent Activity
          </div>
          <div className="growth-log">
            {growth.log.slice(0, 5).map((entry, i) => (
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
    </div>
  );
}
