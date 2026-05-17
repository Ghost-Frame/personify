'use client';

import { useState, useMemo } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import type { PackRecord } from '@/lib/types';
import { CARD_ASSETS } from '@/lib/card-assets';

interface PacksBrowserProps {
  initialPacks: PackRecord[];
  allTags: string[];
}

type ViewMode = 'carousel' | 'grid';

function PackCard({ pack }: { pack: PackRecord }) {
  const [flipped, setFlipped] = useState(false);
  const cardImage = CARD_ASSETS[pack.name] || '/cards/agents.svg';

  return (
    <div
      className={`pack-card ${flipped ? 'is-flipped' : ''}`}
      onClick={() => setFlipped(f => !f)}
      role="button"
      tabIndex={0}
      onKeyDown={e => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          setFlipped(f => !f);
        }
      }}
    >
      <div className="pack-card-inner">
        {/* Front */}
        <div className="pack-card-front">
          {pack.is_new && <div className="new-badge">NEW</div>}
          <Image
            src={cardImage}
            alt={pack.display_name}
            width={400}
            height={560}
            className="pack-card-image"
            unoptimized
          />
          <div className="pack-card-front-hint">click to flip</div>
        </div>

        {/* Back */}
        <div className="pack-card-back">
          <div className="pack-card-back-inner">
            <div className="pack-card-back-title">{pack.display_name}</div>
            <p className="pack-card-back-desc">{pack.description}</p>
            <div className="pack-card-back-meta">
              <div className="spec-row">
                <span className="spec-key">version</span>
                <span className="spec-val">{pack.version}</span>
              </div>
              <div className="spec-row">
                <span className="spec-key">conformance</span>
                <span className="spec-val">{pack.conformance_score}%</span>
              </div>
              <div className="spec-row">
                <span className="spec-key">installs</span>
                <span className="spec-val">{pack.downloads.toLocaleString()}</span>
              </div>
              <div className="spec-row">
                <span className="spec-key">author</span>
                <span className="spec-val">@{pack.author_handle}</span>
              </div>
            </div>
            <div className="pack-card-back-tags">
              {pack.tags.map(t => (
                <span key={t} className="grid-tag">{t}</span>
              ))}
            </div>
            <Link
              href={`/packs/${pack.name}`}
              className="pack-card-action"
              onClick={e => e.stopPropagation()}
            >
              View Details
            </Link>
            <div className="pack-card-back-hint">click to flip back</div>
          </div>
        </div>
      </div>
    </div>
  );
}

function CarouselView({ packs }: { packs: PackRecord[] }) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [flipped, setFlipped] = useState(false);

  const len = packs.length;
  if (len === 0) return null;

  const navigate = (offset: number) => {
    setFlipped(false);
    setCurrentIndex(prev => ((prev + offset) % len + len) % len);
  };

  const SLOTS = 7;
  const HALF = 3;

  const slots = [];
  for (let s = -HALF; s <= HALF; s++) {
    const idx = ((currentIndex + s) % len + len) % len;
    slots.push({ slot: s, pack: packs[idx] });
  }

  const current = packs[currentIndex];
  const cardImage = CARD_ASSETS[current.name] || '/cards/agents.svg';

  return (
    <div className="carousel-wrapper">
      <button
        className="carousel-nav carousel-prev"
        onClick={() => navigate(-1)}
        aria-label="Previous"
      >
        &#8249;
      </button>
      <button
        className="carousel-nav carousel-next"
        onClick={() => navigate(1)}
        aria-label="Next"
      >
        &#8250;
      </button>

      <div className="carousel-track">
        {slots.map(({ slot, pack }) => {
          const abs = Math.abs(slot);
          const tx = slot * 240;
          const tz = -abs * 180;
          const ry = -Math.sign(slot) * Math.min(abs * 40, 55);
          const scale = slot === 0 ? 1.05 : Math.max(1 - abs * 0.18, 0.5);
          const opacity = Math.max(1 - abs * 0.45, 0);
          const isFront = slot === 0;
          const img = CARD_ASSETS[pack.name] || '/cards/agents.svg';

          return (
            <div
              key={slot}
              className={`carousel-card ${isFront ? 'is-front' : ''} ${isFront && flipped ? 'is-flipped' : ''}`}
              style={{
                position: 'absolute',
                top: 0,
                left: '50%',
                marginLeft: '-140px',
                transform: `translateX(${tx}px) translateZ(${tz}px) rotateY(${ry}deg) scale(${scale})`,
                opacity,
                zIndex: 10 - abs,
                pointerEvents: abs <= 1 ? 'auto' : 'none',
                transition: 'transform 0.7s cubic-bezier(0.4,0,0.2,1), opacity 0.7s cubic-bezier(0.4,0,0.2,1)',
              }}
              onClick={() => {
                if (isFront) setFlipped(f => !f);
                else navigate(slot);
              }}
              role="button"
              tabIndex={isFront ? 0 : -1}
            >
              <div
                className="card-inner"
                style={{
                  transform: isFront && flipped ? 'rotateY(180deg)' : undefined,
                  transition: 'transform 1s cubic-bezier(0.4,0,0.2,1)',
                  transformStyle: 'preserve-3d',
                }}
              >
                <div className="card-front">
                  <Image
                    src={img}
                    alt={pack.display_name}
                    width={280}
                    height={392}
                    unoptimized
                    style={{ width: '100%', height: '100%', objectFit: 'cover', borderRadius: '8px' }}
                  />
                </div>
                <div className="card-back">
                  <div className="card-back-inner">
                    <div className="card-back-title">{pack.display_name}</div>
                    <p style={{ fontSize: '0.75rem', color: 'var(--sa-text-dim)', margin: '0.5rem 0' }}>
                      {pack.description.slice(0, 100)}...
                    </p>
                    <div className="spec-row">
                      <span className="spec-key">conformance</span>
                      <span className="spec-val">{pack.conformance_score}%</span>
                    </div>
                    <div className="spec-row">
                      <span className="spec-key">installs</span>
                      <span className="spec-val">{pack.downloads.toLocaleString()}</span>
                    </div>
                    <Link
                      href={`/packs/${pack.name}`}
                      className="pack-card-action"
                      onClick={e => e.stopPropagation()}
                      style={{ marginTop: '0.75rem' }}
                    >
                      View Details
                    </Link>
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <div className="carousel-counter">
        <span className="counter-current">{currentIndex + 1}</span> / {len}
      </div>

      <div className="carousel-reflection" />

      <div className="carousel-info">
        <div className="info-title">{current.display_name}</div>
        <div className="info-desc">{current.description.slice(0, 80)}...</div>
      </div>
    </div>
  );
}

export function PacksBrowser({ initialPacks, allTags }: PacksBrowserProps) {
  const [query, setQuery] = useState('');
  const [activeTag, setActiveTag] = useState('all');
  const [sort, setSort] = useState('downloads');
  const [view, setView] = useState<ViewMode>('carousel');

  const filtered = useMemo(() => {
    let result = initialPacks;

    if (query.trim()) {
      const q = query.toLowerCase();
      result = result.filter(p =>
        p.name.toLowerCase().includes(q) ||
        p.display_name.toLowerCase().includes(q) ||
        p.description.toLowerCase().includes(q) ||
        p.tags.some(t => t.toLowerCase().includes(q))
      );
    }

    if (activeTag !== 'all') {
      result = result.filter(p => p.tags.includes(activeTag));
    }

    switch (sort) {
      case 'downloads':
        result = [...result].sort((a, b) => b.downloads - a.downloads);
        break;
      case 'conformance':
        result = [...result].sort((a, b) => b.conformance_score - a.conformance_score);
        break;
      case 'name':
        result = [...result].sort((a, b) => a.display_name.localeCompare(b.display_name));
        break;
      case 'updated':
        result = [...result].sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime());
        break;
    }

    return result;
  }, [initialPacks, query, activeTag, sort]);

  return (
    <div>
      {/* Controls */}
      <div className="packs-controls">
        <div className="search-bar">
          <input
            type="search"
            value={query}
            onChange={e => setQuery(e.target.value)}
            placeholder="Search packs..."
            aria-label="Search packs"
          />
          <select
            value={sort}
            onChange={e => setSort(e.target.value)}
            aria-label="Sort order"
          >
            <option value="downloads">Most Installed</option>
            <option value="conformance">Conformance</option>
            <option value="name">Name</option>
            <option value="updated">Recently Updated</option>
          </select>
        </div>

        <div className="tag-filters">
          <select
            className="tag-select"
            value={activeTag}
            onChange={e => setActiveTag(e.target.value)}
            aria-label="Filter by tag"
          >
            <option value="all">All Categories</option>
            {allTags.map(tag => (
              <option key={tag} value={tag}>{tag}</option>
            ))}
          </select>
        </div>

        {/* View toggle */}
        <div className="view-toggle">
          <button
            className={`view-btn ${view === 'carousel' ? 'active' : ''}`}
            onClick={() => setView('carousel')}
            aria-label="Carousel view"
            title="Carousel view"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
              <rect x="1" y="4" width="6" height="10" rx="1" stroke="currentColor" strokeWidth="1.5" opacity="0.4" />
              <rect x="6" y="2" width="6" height="14" rx="1" stroke="currentColor" strokeWidth="1.5" />
              <rect x="11" y="4" width="6" height="10" rx="1" stroke="currentColor" strokeWidth="1.5" opacity="0.4" />
            </svg>
          </button>
          <button
            className={`view-btn ${view === 'grid' ? 'active' : ''}`}
            onClick={() => setView('grid')}
            aria-label="Grid view"
            title="Grid view"
          >
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
              <rect x="1" y="1" width="7" height="7" rx="1" stroke="currentColor" strokeWidth="1.5" />
              <rect x="10" y="1" width="7" height="7" rx="1" stroke="currentColor" strokeWidth="1.5" />
              <rect x="1" y="10" width="7" height="7" rx="1" stroke="currentColor" strokeWidth="1.5" />
              <rect x="10" y="10" width="7" height="7" rx="1" stroke="currentColor" strokeWidth="1.5" />
            </svg>
          </button>
        </div>
      </div>

      {/* Results count */}
      <div className="results-count">
        {filtered.length} result{filtered.length !== 1 ? 's' : ''}
        {query && ` for "${query}"`}
        {activeTag !== 'all' && ` in ${activeTag}`}
      </div>

      {/* Content */}
      {filtered.length === 0 ? (
        <div className="empty-state">
          No packs found. Try a different search or tag.
        </div>
      ) : view === 'carousel' ? (
        <CarouselView packs={filtered} />
      ) : (
        <div className="packs-grid">
          {filtered.map(pack => (
            <PackCard key={pack.name} pack={pack} />
          ))}
        </div>
      )}
    </div>
  );
}
