// Tauri invoke wrappers with graceful browser fallback.
// When window.__TAURI__ is undefined (plain browser dev), mock data is returned.

import type { PersonaSummary, GrowthReport } from "./mock-data";
import {
  MOCK_PERSONAS,
  MOCK_ACTIVE_PERSONA,
  mockGrowthReport,
} from "./mock-data";

function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window;
}

async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return tauriInvoke<T>(command, args);
  }
  throw new Error(`Not in Tauri context, command: ${command}`);
}

// -- Personas --

export async function listPersonas(): Promise<PersonaSummary[]> {
  if (!isTauri()) {
    return MOCK_PERSONAS;
  }
  return invoke<PersonaSummary[]>("list_personas");
}

export async function activePersona(): Promise<string | null> {
  if (!isTauri()) {
    return MOCK_ACTIVE_PERSONA;
  }
  return invoke<string | null>("active_persona");
}

export async function activatePersona(name: string): Promise<void> {
  if (!isTauri()) {
    // In browser dev mode, just log -- state is not persisted
    console.info(`[mock] activate_persona: ${name}`);
    return;
  }
  return invoke<void>("activate_persona", { name });
}

export async function installPersona(name: string, source: string): Promise<void> {
  if (!isTauri()) {
    console.info(`[mock] install_persona: ${name} from ${source}`);
    return;
  }
  return invoke<void>("install_persona", { name, source });
}

// -- Growth --

export async function getGrowth(name: string): Promise<GrowthReport> {
  if (!isTauri()) {
    return mockGrowthReport(name);
  }
  const raw = await invoke<string>("get_growth", { name });
  return JSON.parse(raw) as GrowthReport;
}
