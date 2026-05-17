// ========== FRAMESHIFT MARKETPLACE TYPES ==========

export interface VersionEntry {
  version: string;
  published_at: string;
  changelog?: string;
  downloads: number;
}

export interface CapabilityManifest {
  tools?: string[];
  memory?: boolean;
  reasoning?: string;
  context_window?: number;
  languages?: string[];
  frameworks?: string[];
  specialties?: string[];
}

export interface PackRecord {
  name: string;
  display_name: string;
  description: string;
  author_handle: string;
  version: string;
  tags: string[];
  conformance_score: number;
  downloads: number;
  published_at: string;
  updated_at: string;
  versions: VersionEntry[];
  capabilities: CapabilityManifest;
  extends?: string;
  is_new?: boolean;
}

export interface AuthorRecord {
  handle: string;
  display_name: string;
  bio: string;
  avatar_url?: string;
  packs: PackRecord[];
  total_downloads: number;
  joined_at: string;
}

export interface ListPacksResponse {
  packs: PackRecord[];
  total: number;
  page: number;
  per_page: number;
}

export interface SearchParams {
  q?: string;
  author?: string;
  extends?: string;
  tag?: string;
  page?: number;
}
