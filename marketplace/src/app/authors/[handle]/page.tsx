import { notFound } from 'next/navigation';
import Link from 'next/link';
import { getAuthor } from '@/lib/api';
import { MOCK_AUTHORS } from '@/lib/mock-data';
import type { Metadata } from 'next';

interface Props {
  params: Promise<{ handle: string }>;
}

export async function generateStaticParams() {
  return MOCK_AUTHORS.map(a => ({ handle: a.handle }));
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { handle } = await params;
  const author = await getAuthor(handle);
  if (!author) return { title: 'Author not found' };
  return {
    title: `${author.display_name} -- FrameShift Authors`,
    description: author.bio,
  };
}

export default async function AuthorPage({ params }: Props) {
  const { handle } = await params;
  const author = await getAuthor(handle);

  if (!author) {
    notFound();
  }

  const joinedDate = new Date(author.joined_at).toLocaleDateString('en-US', {
    year: 'numeric', month: 'long',
  });

  const sortedPacks = [...author.packs].sort((a, b) => b.downloads - a.downloads);

  return (
    <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '2.5rem 1.5rem 4rem' }}>
      {/* Breadcrumb */}
      <div style={{ marginBottom: '1.5rem', fontSize: '0.75rem', color: 'var(--sa-text-dim)', fontFamily: 'Orbitron, monospace', letterSpacing: '0.08em' }}>
        <span style={{ color: 'var(--sa-text-dim)' }}>Authors</span>
        <span style={{ margin: '0 0.5rem', opacity: 0.5 }}>/</span>
        <span style={{ color: 'var(--sa-text)' }}>{author.handle}</span>
      </div>

      {/* Profile header */}
      <div
        style={{
          background: 'var(--sa-surface)',
          border: '1px solid var(--sa-border)',
          borderRadius: '12px',
          padding: '2rem',
          marginBottom: '2rem',
          display: 'flex',
          alignItems: 'flex-start',
          gap: '2rem',
          flexWrap: 'wrap',
        }}
      >
        {/* Avatar */}
        <div
          style={{
            width: '80px',
            height: '80px',
            borderRadius: '50%',
            background: 'linear-gradient(135deg, var(--sa-accent), var(--sa-pink))',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
          }}
        >
          <span style={{ fontFamily: 'Orbitron, monospace', fontWeight: 900, fontSize: '1.6rem', color: '#fff' }}>
            {author.display_name.slice(0, 1).toUpperCase()}
          </span>
        </div>

        {/* Info */}
        <div style={{ flex: 1 }}>
          <h1
            style={{
              fontFamily: 'Orbitron, monospace',
              fontWeight: 700,
              fontSize: '1.5rem',
              letterSpacing: '0.06em',
              color: 'var(--sa-text-bright)',
              margin: '0 0 0.4rem',
            }}
          >
            {author.display_name}
          </h1>
          <div
            style={{
              fontFamily: 'Orbitron, monospace',
              fontSize: '0.65rem',
              color: 'var(--sa-accent)',
              letterSpacing: '0.1em',
              marginBottom: '0.75rem',
            }}
          >
            @{author.handle}
          </div>
          <p style={{ fontSize: '0.85rem', color: 'var(--sa-text-dim)', lineHeight: 1.6, margin: '0 0 1rem' }}>
            {author.bio}
          </p>
          <div style={{ display: 'flex', gap: '1.5rem', flexWrap: 'wrap' }}>
            {[
              { value: author.packs.length, label: 'Packs' },
              { value: author.total_downloads.toLocaleString(), label: 'Total Installs' },
              { value: joinedDate, label: 'Member Since' },
            ].map(stat => (
              <div key={stat.label}>
                <div style={{ fontFamily: 'Orbitron, monospace', fontWeight: 700, fontSize: '1rem', color: 'var(--sa-accent)' }}>
                  {stat.value}
                </div>
                <div style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.6rem', letterSpacing: '0.1em', color: 'var(--sa-text-dim)', textTransform: 'uppercase' }}>
                  {stat.label}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Packs grid */}
      <div style={{ marginBottom: '1rem' }}>
        <h2
          style={{
            fontFamily: 'Orbitron, monospace',
            fontSize: '0.85rem',
            fontWeight: 700,
            letterSpacing: '0.1em',
            textTransform: 'uppercase',
            color: 'var(--sa-text-bright)',
            marginBottom: '1.25rem',
          }}
        >
          Published Packs ({author.packs.length})
        </h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '1rem' }}>
          {sortedPacks.map(pack => (
            <Link key={pack.name} href={`/packs/${pack.name}`} style={{ textDecoration: 'none' }}>
              <div
                className="hover-card"
                style={{
                  padding: '1.2rem',
                  flexDirection: 'column',
                  gap: '0.5rem',
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                  <span style={{ fontFamily: 'Orbitron, monospace', fontWeight: 700, fontSize: '0.85rem', letterSpacing: '0.06em', color: 'var(--sa-text-bright)' }}>
                    {pack.display_name}
                  </span>
                  <span style={{ fontFamily: 'Orbitron, monospace', fontSize: '0.7rem', color: 'var(--sa-accent)', fontWeight: 700 }}>
                    {pack.conformance_score}%
                  </span>
                </div>
                <p style={{ fontSize: '0.78rem', color: 'var(--sa-text-dim)', lineHeight: 1.45, flex: 1, margin: 0 }}>
                  {pack.description.slice(0, 90)}{pack.description.length > 90 ? '...' : ''}
                </p>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.3rem' }}>
                  {pack.tags.slice(0, 3).map(tag => (
                    <span key={tag} className="grid-tag">{tag}</span>
                  ))}
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.7rem', color: 'var(--sa-text-dim)', marginTop: '0.25rem' }}>
                  <span>v{pack.version}</span>
                  <span>{pack.downloads.toLocaleString()} installs</span>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
