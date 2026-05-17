import { notFound } from 'next/navigation';
import Link from 'next/link';
import { getPack } from '@/lib/api';
import { MOCK_PACKS } from '@/lib/mock-data';
import type { Metadata } from 'next';
import { InstallButton } from '@/components/InstallButton';

interface Props {
  params: Promise<{ name: string }>;
}

export async function generateStaticParams() {
  return MOCK_PACKS.map(p => ({ name: p.name }));
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { name } = await params;
  const pack = await getPack(name);
  if (!pack) return { title: 'Pack not found' };
  return {
    title: `Install ${pack.display_name} -- FrameShift`,
    description: `Install the ${pack.display_name} persona pack into FrameShift Desktop.`,
  };
}

export default async function InstallPage({ params }: Props) {
  const { name } = await params;
  const pack = await getPack(name);

  if (!pack) {
    notFound();
  }

  const deepLink = `frameshift://install?pack=${encodeURIComponent(pack.name)}&version=${encodeURIComponent(pack.version)}`;

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: 'calc(100vh - 56px)',
        padding: '3rem 1.5rem',
      }}
    >
      {/* Ambient glow */}
      <div
        aria-hidden="true"
        style={{
          position: 'fixed',
          top: '40%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          width: '700px',
          height: '400px',
          background: 'radial-gradient(ellipse, rgba(94,186,239,0.07) 0%, transparent 65%)',
          pointerEvents: 'none',
          zIndex: 0,
        }}
      />

      <div className="deep-link-card" style={{ position: 'relative', zIndex: 1 }}>
        {/* Pack identity */}
        <div
          style={{
            width: '56px',
            height: '56px',
            borderRadius: '12px',
            background: 'linear-gradient(135deg, var(--sa-accent), var(--sa-pink))',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            margin: '0 auto 1.25rem',
          }}
        >
          <span style={{ fontFamily: 'Orbitron, monospace', fontWeight: 900, fontSize: '1.4rem', color: '#fff' }}>
            {pack.display_name.slice(0, 1)}
          </span>
        </div>

        <div
          style={{
            fontFamily: 'Orbitron, monospace',
            fontSize: '0.6rem',
            letterSpacing: '0.15em',
            textTransform: 'uppercase',
            color: 'var(--sa-text-dim)',
            marginBottom: '0.4rem',
          }}
        >
          Install Persona Pack
        </div>

        <h1
          style={{
            fontFamily: 'Orbitron, monospace',
            fontWeight: 900,
            fontSize: '1.6rem',
            letterSpacing: '0.06em',
            color: 'var(--sa-text-bright)',
            marginBottom: '0.3rem',
          }}
        >
          {pack.display_name}
        </h1>

        <div
          style={{
            fontFamily: 'Orbitron, monospace',
            fontSize: '0.65rem',
            color: 'var(--sa-accent)',
            letterSpacing: '0.1em',
            marginBottom: '1rem',
          }}
        >
          v{pack.version} by {pack.author_handle}
        </div>

        <p style={{ fontSize: '0.85rem', color: 'var(--sa-text-dim)', lineHeight: 1.6, marginBottom: '1.5rem' }}>
          {pack.description.slice(0, 160)}{pack.description.length > 160 ? '...' : ''}
        </p>

        {/* Stats bar */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'center',
            gap: '2rem',
            marginBottom: '2rem',
            padding: '0.75rem',
            background: 'var(--sa-bg)',
            borderRadius: '8px',
            border: '1px solid var(--sa-border)',
          }}
        >
          {[
            { value: `${pack.conformance_score}%`, label: 'Conformance' },
            { value: pack.downloads.toLocaleString(), label: 'Installs' },
            { value: pack.versions.length.toString(), label: 'Versions' },
          ].map(stat => (
            <div key={stat.label} style={{ textAlign: 'center' }}>
              <div style={{ fontFamily: 'Orbitron, monospace', fontWeight: 700, fontSize: '0.9rem', color: 'var(--sa-accent)' }}>
                {stat.value}
              </div>
              <div style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.55rem', letterSpacing: '0.1em', color: 'var(--sa-text-dim)', textTransform: 'uppercase' }}>
                {stat.label}
              </div>
            </div>
          ))}
        </div>

        {/* Install button (client component for deep link) */}
        <InstallButton deepLink={deepLink} packName={pack.display_name} />

        {/* Tags */}
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem', justifyContent: 'center', marginTop: '1.5rem' }}>
          {pack.tags.map(tag => (
            <span key={tag} className="grid-tag">{tag}</span>
          ))}
        </div>

        {/* Back link */}
        <div style={{ marginTop: '1.5rem' }}>
          <Link
            href={`/packs/${pack.name}`}
            style={{
              fontFamily: 'Orbitron, monospace',
              fontSize: '0.6rem',
              letterSpacing: '0.1em',
              color: 'var(--sa-text-dim)',
              textTransform: 'uppercase',
            }}
          >
            View Pack Details
          </Link>
        </div>
      </div>
    </div>
  );
}
