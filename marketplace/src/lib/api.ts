// ========== FRAMESHIFT API CLIENT ==========

import type { PackRecord, AuthorRecord, ListPacksResponse, SearchParams } from './types';
import { MOCK_PACKS, MOCK_AUTHORS } from './mock-data';

const API_BASE = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:8080/v1';

async function apiFetch<T>(path: string): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Accept': 'application/json' },
  });
  if (!res.ok) {
    throw new Error(`API error ${res.status} for ${path}`);
  }
  return res.json() as Promise<T>;
}

// ---- List all packs (with optional tag query) ----
export async function listPacks(query?: { tag?: string; page?: number }): Promise<ListPacksResponse> {
  try {
    const params = new URLSearchParams();
    if (query?.tag) params.set('tag', query.tag);
    if (query?.page) params.set('page', String(query.page));
    const qs = params.toString();
    return await apiFetch<ListPacksResponse>(`/packs${qs ? `?${qs}` : ''}`);
  } catch {
    // Graceful fallback to mock data
    const packs = query?.tag && query.tag !== 'all'
      ? MOCK_PACKS.filter(p => p.tags.includes(query.tag!))
      : MOCK_PACKS;
    return { packs, total: packs.length, page: 1, per_page: 50 };
  }
}

// ---- Get single pack by name ----
export async function getPack(name: string): Promise<PackRecord | null> {
  try {
    return await apiFetch<PackRecord>(`/packs/${name}`);
  } catch {
    return MOCK_PACKS.find(p => p.name === name) ?? null;
  }
}

// ---- Get author profile by handle ----
export async function getAuthor(handle: string): Promise<AuthorRecord | null> {
  try {
    return await apiFetch<AuthorRecord>(`/authors/${handle}`);
  } catch {
    return MOCK_AUTHORS.find(a => a.handle === handle) ?? null;
  }
}

// ---- Search packs ----
export async function searchPacks(params: SearchParams): Promise<ListPacksResponse> {
  try {
    const qs = new URLSearchParams();
    if (params.q) qs.set('q', params.q);
    if (params.author) qs.set('author', params.author);
    if (params.extends) qs.set('extends', params.extends);
    if (params.tag) qs.set('tag', params.tag);
    if (params.page) qs.set('page', String(params.page));
    return await apiFetch<ListPacksResponse>(`/packs/search?${qs.toString()}`);
  } catch {
    // Fallback: filter mock data
    let packs = MOCK_PACKS;
    if (params.q) {
      const q = params.q.toLowerCase();
      packs = packs.filter(p =>
        p.name.toLowerCase().includes(q) ||
        p.display_name.toLowerCase().includes(q) ||
        p.description.toLowerCase().includes(q) ||
        p.tags.some(t => t.toLowerCase().includes(q))
      );
    }
    if (params.author) {
      packs = packs.filter(p => p.author_handle === params.author);
    }
    if (params.extends) {
      packs = packs.filter(p => p.extends === params.extends);
    }
    if (params.tag && params.tag !== 'all') {
      packs = packs.filter(p => p.tags.includes(params.tag!));
    }
    return { packs, total: packs.length, page: 1, per_page: 50 };
  }
}
