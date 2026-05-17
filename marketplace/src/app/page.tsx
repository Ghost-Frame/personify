import Link from 'next/link';
import { listPacks } from '@/lib/api';
import { SITE_STATS } from '@/lib/mock-data';
import type { PackRecord } from '@/lib/types';

function ConformanceBar({ score }: { score: number }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
      <div className="score-bar" style={{ flex: 1 }}>
        <div className="score-fill" style={{ width: `${score}%` }} />
      </div>
      <span style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.65rem', color: 'var(--sa-accent)', minWidth: '2.5rem', textAlign: 'right' }}>
        {score}%
      </span>
    </div>
  );
}

function FeaturedCard({ pack }: { pack: PackRecord }) {
  return (
    <Link
      href={`/packs/${pack.name}`}
      style={{ textDecoration: 'none', display: 'block' }}
    >
      <div
        className="hover-card"
        style={{ padding: '1.25rem' }}
      >
        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', marginBottom: '0.75rem' }}>
          <span
            style={{
              fontFamily: 'Orbitron, monospace',
              fontWeight: 700,
              fontSize: '0.85rem',
              letterSpacing: '0.06em',
              color: 'var(--sa-text-bright)',
            }}
          >
            {pack.display_name}
          </span>
          {pack.is_new && (
            <span className="new-badge" style={{ position: 'static', display: 'inline-block' }}>NEW</span>
          )}
        </div>
        <p style={{ fontSize: '0.78rem', color: 'var(--sa-text-dim)', lineHeight: 1.5, marginBottom: '0.9rem', margin: '0 0 0.9rem' }}>
          {pack.description.slice(0, 100)}{pack.description.length > 100 ? '...' : ''}
        </p>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.3rem', marginBottom: '0.75rem' }}>
          {pack.tags.slice(0, 3).map(tag => (
            <span key={tag} className="grid-tag">{tag}</span>
          ))}
        </div>
        <ConformanceBar score={pack.conformance_score} />
        <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '0.75rem', fontSize: '0.7rem', color: 'var(--sa-text-dim)' }}>
          <span>v{pack.version}</span>
          <span>{pack.downloads.toLocaleString()} installs</span>
        </div>
      </div>
    </Link>
  );
}

export default async function HomePage() {
  const { packs } = await listPacks();
  const featured = [...packs]
    .sort((a, b) => b.downloads - a.downloads)
    .slice(0, 6);

  return (
    <div>
      {/* Hero */}
      <section
        style={{
          textAlign: 'center',
          padding: '5rem 1.5rem 4rem',
          maxWidth: '760px',
          margin: '0 auto',
          position: 'relative',
        }}
      >
        {/* Glow behind hero */}
        <div
          aria-hidden="true"
          style={{
            position: 'absolute',
            top: '30%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            width: '600px',
            height: '300px',
            background: 'radial-gradient(ellipse, rgba(94,186,239,0.06) 0%, transparent 70%)',
            pointerEvents: 'none',
          }}
        />
        <div
          style={{
            display: 'inline-block',
            fontFamily: 'Orbitron, monospace',
            fontSize: '0.65rem',
            fontWeight: 500,
            letterSpacing: '0.18em',
            textTransform: 'uppercase',
            color: 'var(--sa-accent)',
            border: '1px solid rgba(94,186,239,0.25)',
            borderRadius: '4px',
            padding: '0.3rem 0.8rem',
            marginBottom: '1.5rem',
          }}
        >
          Persona Registry
        </div>
        <h1
          style={{
            fontFamily: 'Orbitron, monospace',
            fontSize: 'clamp(1.8rem, 5vw, 3rem)',
            fontWeight: 900,
            lineHeight: 1.15,
            color: 'var(--sa-text-bright)',
            marginBottom: '1rem',
            letterSpacing: '0.04em',
          }}
        >
          The FrameShift<br />
          <span style={{ color: 'var(--sa-accent)' }}>Marketplace</span>
        </h1>
        <p
          style={{
            fontSize: '1rem',
            color: 'var(--sa-text-dim)',
            lineHeight: 1.65,
            maxWidth: '520px',
            margin: '0 auto 2rem',
          }}
        >
          Install structured AI personas into your FrameShift workspace.
          Each pack defines behavior, capabilities, and conformance guarantees.
        </p>
        <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center', flexWrap: 'wrap' }}>
          <Link href="/packs" className="install-btn">
            Browse Packs
          </Link>
          <Link
            href="/packs"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '0.6rem',
              padding: '0.8rem 1.8rem',
              background: 'transparent',
              border: '1px solid var(--sa-border)',
              borderRadius: '6px',
              color: 'var(--sa-text)',
              fontFamily: 'Orbitron, monospace',
              fontSize: '0.75rem',
              fontWeight: 600,
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
              textDecoration: 'none',
              transition: 'border-color 0.2s, color 0.2s',
            }}
          >
            View All {packs.length} Personas
          </Link>
        </div>
      </section>

      {/* Stats row */}
      <section style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem 3rem' }}>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '1rem' }}>
          {[
            { value: SITE_STATS.total_packs, label: 'Personas' },
            { value: SITE_STATS.total_downloads.toLocaleString(), label: 'Total Installs' },
            { value: SITE_STATS.total_authors, label: 'Authors' },
            { value: `${SITE_STATS.avg_conformance}%`, label: 'Avg Conformance' },
          ].map(s => (
            <div key={s.label} className="stat-card">
              <div className="stat-value">{s.value}</div>
              <div className="stat-label">{s.label}</div>
            </div>
          ))}
        </div>
      </section>

      {/* Featured packs */}
      <section style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem 4rem' }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '1.5rem' }}>
          <h2
            style={{
              fontFamily: 'Orbitron, monospace',
              fontSize: '0.9rem',
              fontWeight: 700,
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
              color: 'var(--sa-text-bright)',
              margin: 0,
            }}
          >
            Featured Packs
          </h2>
          <Link
            href="/packs"
            style={{ fontSize: '0.75rem', color: 'var(--sa-accent)', fontFamily: 'Orbitron, monospace', letterSpacing: '0.08em' }}
          >
            View all
          </Link>
        </div>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '1rem' }}>
          {featured.map(pack => (
            <FeaturedCard key={pack.name} pack={pack} />
          ))}
        </div>
      </section>
    </div>
  );
}
