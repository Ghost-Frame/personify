import { notFound } from 'next/navigation';
import Link from 'next/link';
import { getPack } from '@/lib/api';
import { MOCK_PACKS } from '@/lib/mock-data';
import type { Metadata } from 'next';

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
    title: `${pack.display_name} -- FrameShift Marketplace`,
    description: pack.description,
  };
}

function ConformanceMeter({ score }: { score: number }) {
  const color = score >= 95 ? 'var(--sa-accent)' : score >= 85 ? '#f0b429' : 'var(--sa-pink)';
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
      <div
        style={{
          width: '80px',
          height: '80px',
          borderRadius: '50%',
          background: `conic-gradient(${color} ${score * 3.6}deg, var(--sa-border) 0deg)`,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          position: 'relative',
        }}
      >
        <div
          style={{
            width: '60px',
            height: '60px',
            borderRadius: '50%',
            background: 'var(--sa-surface)',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <span style={{ fontFamily: 'Orbitron, monospace', fontWeight: 700, fontSize: '1rem', color, lineHeight: 1 }}>
            {score}
          </span>
          <span style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.45rem', color: 'var(--sa-text-dim)', letterSpacing: '0.08em' }}>
            SCORE
          </span>
        </div>
      </div>
      <div>
        <div style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.7rem', color: 'var(--sa-text-bright)', marginBottom: '0.25rem' }}>
          Conformance Score
        </div>
        <div style={{ fontSize: '0.75rem', color: 'var(--sa-text-dim)' }}>
          {score >= 95 ? 'Excellent -- production-ready' : score >= 85 ? 'Good -- minor edge cases' : 'Fair -- review recommended'}
        </div>
      </div>
    </div>
  );
}

export default async function PackDetailPage({ params }: Props) {
  const { name } = await params;
  const pack = await getPack(name);

  if (!pack) {
    notFound();
  }

  const publishedDate = new Date(pack.published_at).toLocaleDateString('en-US', {
    year: 'numeric', month: 'long', day: 'numeric',
  });
  const updatedDate = new Date(pack.updated_at).toLocaleDateString('en-US', {
    year: 'numeric', month: 'long', day: 'numeric',
  });

  return (
    <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '2.5rem 1.5rem 4rem' }}>
      {/* Breadcrumb */}
      <div style={{ marginBottom: '1.5rem', fontSize: '0.75rem', color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', letterSpacing: '0.08em' }}>
        <Link href="/packs" style={{ color: 'var(--sa-text-dim)' }}>Packs</Link>
        <span style={{ margin: '0 0.5rem', opacity: 0.5 }}>/</span>
        <span style={{ color: 'var(--sa-text)' }}>{pack.display_name}</span>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 320px', gap: '2rem', alignItems: 'start' }}>
        {/* Main content */}
        <div>
          {/* Header */}
          <div style={{ marginBottom: '2rem' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '1rem', flexWrap: 'wrap', marginBottom: '0.6rem' }}>
              <h1
                style={{
                  fontFamily: 'Orbitron, monospace',
                  fontWeight: 900,
                  fontSize: '2rem',
                  letterSpacing: '0.04em',
                  color: 'var(--sa-text-bright)',
                  margin: 0,
                }}
              >
                {pack.display_name}
              </h1>
              {pack.is_new && <span className="new-badge" style={{ position: 'static', display: 'inline-block' }}>NEW</span>}
            </div>
            <p style={{ fontSize: '0.95rem', color: 'var(--sa-text-dim)', lineHeight: 1.65, marginBottom: '1rem' }}>
              {pack.description}
            </p>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem' }}>
              {pack.tags.map(tag => (
                <Link key={tag} href={`/packs?tag=${tag}`}>
                  <span className="grid-tag" style={{ cursor: 'pointer' }}>{tag}</span>
                </Link>
              ))}
            </div>
          </div>

          {/* Conformance */}
          <div className="detail-section">
            <div className="detail-label">Conformance</div>
            <ConformanceMeter score={pack.conformance_score} />
          </div>

          {/* Capabilities */}
          <div className="detail-section">
            <div className="detail-label">Capability Manifest</div>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(180px, 1fr))', gap: '1rem' }}>
              {pack.capabilities.tools && pack.capabilities.tools.length > 0 && (
                <div>
                  <div style={{ fontSize: '0.65rem', fontFamily: 'Orbitron, monospace', color: 'var(--sa-text-dim)', letterSpacing: '0.1em', marginBottom: '0.5rem', textTransform: 'uppercase' }}>
                    Tools
                  </div>
                  {pack.capabilities.tools.map(tool => (
                    <div key={tool} className="capability-tag">{tool}</div>
                  ))}
                </div>
              )}
              {pack.capabilities.specialties && pack.capabilities.specialties.length > 0 && (
                <div>
                  <div style={{ fontSize: '0.65rem', fontFamily: 'Orbitron, monospace', color: 'var(--sa-text-dim)', letterSpacing: '0.1em', marginBottom: '0.5rem', textTransform: 'uppercase' }}>
                    Specialties
                  </div>
                  {pack.capabilities.specialties.map(s => (
                    <div key={s} className="capability-tag">{s}</div>
                  ))}
                </div>
              )}
              {pack.capabilities.languages && pack.capabilities.languages.length > 0 && (
                <div>
                  <div style={{ fontSize: '0.65rem', fontFamily: 'Orbitron, monospace', color: 'var(--sa-text-dim)', letterSpacing: '0.1em', marginBottom: '0.5rem', textTransform: 'uppercase' }}>
                    Languages
                  </div>
                  {pack.capabilities.languages.map(l => (
                    <div key={l} className="capability-tag">{l}</div>
                  ))}
                </div>
              )}
              {pack.capabilities.frameworks && pack.capabilities.frameworks.length > 0 && (
                <div>
                  <div style={{ fontSize: '0.65rem', fontFamily: 'Orbitron, monospace', color: 'var(--sa-text-dim)', letterSpacing: '0.1em', marginBottom: '0.5rem', textTransform: 'uppercase' }}>
                    Frameworks
                  </div>
                  {pack.capabilities.frameworks.map(f => (
                    <div key={f} className="capability-tag">{f}</div>
                  ))}
                </div>
              )}
            </div>
            <div style={{ marginTop: '1rem', display: 'flex', gap: '1.5rem', flexWrap: 'wrap' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontSize: '0.8rem' }}>
                <span style={{ width: '8px', height: '8px', borderRadius: '50%', background: pack.capabilities.memory ? 'var(--sa-accent)' : 'var(--sa-border)', display: 'inline-block' }} />
                <span style={{ color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', fontSize: '0.65rem', letterSpacing: '0.08em' }}>
                  Memory: {pack.capabilities.memory ? 'enabled' : 'disabled'}
                </span>
              </div>
              {pack.capabilities.reasoning && (
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontSize: '0.8rem' }}>
                  <span style={{ width: '8px', height: '8px', borderRadius: '50%', background: 'var(--sa-pink)', display: 'inline-block' }} />
                  <span style={{ color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', fontSize: '0.65rem', letterSpacing: '0.08em' }}>
                    Reasoning: {pack.capabilities.reasoning}
                  </span>
                </div>
              )}
            </div>
          </div>

          {/* Version History */}
          <div className="detail-section">
            <div className="detail-label">Version History</div>
            {pack.versions.map(v => (
              <div key={v.version} className="version-row">
                <span className="version-tag">v{v.version}</span>
                <span style={{ flex: 1, fontSize: '0.8rem', color: 'var(--sa-text)' }}>
                  {v.changelog ?? 'No changelog'}
                </span>
                <span style={{ fontSize: '0.7rem', color: 'var(--sa-text-dim)', whiteSpace: 'nowrap' }}>
                  {new Date(v.published_at).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })}
                </span>
                <span style={{ fontSize: '0.7rem', color: 'var(--sa-text-dim)', whiteSpace: 'nowrap', minWidth: '6rem', textAlign: 'right' }}>
                  {v.downloads.toLocaleString()} installs
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Sidebar */}
        <div style={{ position: 'sticky', top: '72px' }}>
          {/* Install CTA */}
          <div className="detail-section" style={{ textAlign: 'center', marginBottom: '1rem' }}>
            <Link
              href={`/install/${pack.name}`}
              className="install-btn"
              style={{ width: '100%', justifyContent: 'center', marginBottom: '0.75rem', display: 'flex' }}
            >
              Install Pack
            </Link>
            <div style={{ fontSize: '0.65rem', color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', letterSpacing: '0.08em' }}>
              Opens FrameShift Desktop
            </div>
          </div>

          {/* Meta info */}
          <div className="detail-section">
            <div className="detail-label">Info</div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
              {[
                { label: 'Version', value: `v${pack.version}` },
                { label: 'Author', value: pack.author_handle, link: `/authors/${pack.author_handle}` },
                { label: 'Published', value: publishedDate },
                { label: 'Updated', value: updatedDate },
                { label: 'Downloads', value: pack.downloads.toLocaleString() },
                ...(pack.extends ? [{ label: 'Extends', value: pack.extends, link: `/packs/${pack.extends}` }] : []),
              ].map(item => (
                <div key={item.label} style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.8rem', gap: '0.5rem' }}>
                  <span style={{ color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', fontSize: '0.6rem', letterSpacing: '0.08em', textTransform: 'uppercase', flexShrink: 0 }}>
                    {item.label}
                  </span>
                  {item.link ? (
                    <Link href={item.link} style={{ color: 'var(--sa-accent)', fontFamily: 'Orbitron, monospace', fontSize: '0.7rem' }}>
                      {item.value}
                    </Link>
                  ) : (
                    <span style={{ color: 'var(--sa-text)', textAlign: 'right' }}>{item.value}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
