"use client";

import { useState } from "react";

export default function SettingsPage() {
  const [sandboxEnabled, setSandboxEnabled] = useState(true);
  const [autoActivate, setAutoActivate] = useState(false);
  const [growthTracking, setGrowthTracking] = useState(true);
  const [dataDir] = useState("~/.local/share/frameshift");
  const [apiUrl, setApiUrl] = useState("https://api.ghostframe.io");

  return (
    <div>
      <div className="page-header">
        <div className="page-title">Settings</div>
        <div className="page-subtitle">Configure FrameShift runtime preferences</div>
      </div>

      {/* Capability Sandbox */}
      <div className="settings-section">
        <div className="settings-section-title">Capability Sandbox</div>

        <div className="card">
          <div className="settings-row">
            <div>
              <div className="settings-label">Enable Sandbox</div>
              <div className="settings-description">
                Run capability checks in an isolated context before applying to active persona
              </div>
            </div>
            <button
              className={`toggle${sandboxEnabled ? " on" : ""}`}
              onClick={() => setSandboxEnabled((v) => !v)}
              aria-label="Toggle sandbox"
            />
          </div>

          <div className="settings-row">
            <div>
              <div className="settings-label">Auto-activate on Install</div>
              <div className="settings-description">
                Automatically activate a persona after installation
              </div>
            </div>
            <button
              className={`toggle${autoActivate ? " on" : ""}`}
              onClick={() => setAutoActivate((v) => !v)}
              aria-label="Toggle auto-activate"
            />
          </div>

          <div className="settings-row">
            <div>
              <div className="settings-label">Growth Tracking</div>
              <div className="settings-description">
                Track session data to improve capability scores over time
              </div>
            </div>
            <button
              className={`toggle${growthTracking ? " on" : ""}`}
              onClick={() => setGrowthTracking((v) => !v)}
              aria-label="Toggle growth tracking"
            />
          </div>
        </div>
      </div>

      {/* Data */}
      <div className="settings-section">
        <div className="settings-section-title">Data</div>

        <div className="card">
          <div className="settings-row">
            <div>
              <div className="settings-label">Data Directory</div>
              <div className="settings-description">
                Where personas, growth logs, and config are stored
              </div>
            </div>
            <span className="settings-value mono">{dataDir}</span>
          </div>
        </div>
      </div>

      {/* API */}
      <div className="settings-section">
        <div className="settings-section-title">API</div>

        <div className="card">
          <div className="settings-row">
            <div>
              <div className="settings-label">API Endpoint</div>
              <div className="settings-description">
                FrameShift server for marketplace and sync
              </div>
            </div>
            <input
              type="text"
              value={apiUrl}
              onChange={(e) => setApiUrl(e.target.value)}
              style={{
                padding: "0.3rem 0.6rem",
                background: "var(--sa-surface-hover)",
                border: "1px solid var(--sa-border)",
                borderRadius: "4px",
                color: "var(--sa-accent)",
                fontFamily: "inherit",
                fontSize: "0.78rem",
                width: "260px",
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
        </div>
      </div>

      {/* About */}
      <div className="settings-section">
        <div className="settings-section-title">About</div>

        <div className="card">
          <div className="settings-row">
            <div className="settings-label">Version</div>
            <span className="settings-value">0.1.0</span>
          </div>
          <div className="settings-row">
            <div className="settings-label">Identifier</div>
            <span className="settings-value mono">io.ghostframe.frameshift</span>
          </div>
          <div className="settings-row">
            <div className="settings-label">Runtime</div>
            <span className="settings-value">Tauri 2 / Next.js 15</span>
          </div>
        </div>
      </div>
    </div>
  );
}
