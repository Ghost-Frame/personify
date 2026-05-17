import { PacksBrowser } from '@/components/PacksBrowser';
import { listPacks } from '@/lib/api';
import { ALL_TAGS } from '@/lib/mock-data';

export default async function PacksPage() {
  const { packs } = await listPacks();

  return (
    <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '2.5rem 1.5rem 4rem' }}>
      <div style={{ marginBottom: '2rem' }}>
        <h1
          style={{
            fontFamily: 'Orbitron, monospace',
            fontSize: '1.4rem',
            fontWeight: 700,
            letterSpacing: '0.06em',
            color: 'var(--sa-text-bright)',
            marginBottom: '0.4rem',
          }}
        >
          Browse Packs
        </h1>
        <p style={{ color: 'var(--sa-text-dim)', fontSize: '0.85rem', margin: 0 }}>
          {packs.length} persona packs available
        </p>
      </div>
      <PacksBrowser initialPacks={packs} allTags={ALL_TAGS} />
    </div>
  );
}
