"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { listPersonas, activatePersona } from "@/lib/tauri";
import type { PersonaSummary } from "@/lib/mock-data";

export default function PersonasPage() {
  const [personas, setPersonas] = useState<PersonaSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [activating, setActivating] = useState<string | null>(null);

  useEffect(() => {
    listPersonas()
      .then(setPersonas)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  async function handleActivate(name: string) {
    setActivating(name);
    try {
      await activatePersona(name);
      setPersonas((prev) =>
        prev.map((p) => ({ ...p, active: p.name === name }))
      );
    } catch (err) {
      console.error("activate error:", err);
    } finally {
      setActivating(null);
    }
  }

  return (
    <div>
      <div className="page-header">
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
          <div>
            <div className="page-title">Personas</div>
            <div className="page-subtitle">
              {personas.length} installed -- click a persona to view details
            </div>
          </div>
          <Link href="/marketplace" className="btn btn-primary btn-sm">
            Browse Marketplace
          </Link>
        </div>
      </div>

      {loading ? (
        <div className="card-meta">Loading personas...</div>
      ) : (
        <div className="persona-grid">
          {personas.map((persona) => (
            <div
              key={persona.name}
              className={`persona-card${persona.active ? " is-active" : ""}`}
            >
              <div className="persona-card-header">
                <div>
                  <div className="persona-name">{persona.name}</div>
                  <div className="persona-version">v{persona.version}</div>
                </div>
                {persona.active ? (
                  <span className="badge badge-active">active</span>
                ) : null}
              </div>

              <div className="persona-description">{persona.description}</div>

              <div className="persona-caps">
                {persona.capabilities.map((cap) => (
                  <span key={cap} className="persona-cap-tag">{cap}</span>
                ))}
              </div>

              <div className="persona-card-footer">
                <span className="card-meta">
                  Installed {new Date(persona.installed_at).toLocaleDateString()}
                </span>
                <div style={{ display: "flex", gap: "0.4rem" }}>
                  <Link href={`/personas/${persona.name}`} className="btn btn-sm">
                    Details
                  </Link>
                  {!persona.active && (
                    <button
                      className="btn btn-sm btn-primary"
                      onClick={() => handleActivate(persona.name)}
                      disabled={activating === persona.name}
                    >
                      {activating === persona.name ? "Activating..." : "Activate"}
                    </button>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
