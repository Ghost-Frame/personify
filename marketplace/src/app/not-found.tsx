import Link from 'next/link';

export default function NotFound() {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: 'calc(100vh - 56px)',
        textAlign: 'center',
        padding: '3rem 1.5rem',
      }}
    >
      <div
        style={{
          fontFamily: 'Orbitron, monospace',
          fontSize: '5rem',
          fontWeight: 900,
          color: 'var(--sa-border)',
          lineHeight: 1,
          marginBottom: '1rem',
        }}
      >
        404
      </div>
      <h1
        style={{
          fontFamily: 'Orbitron, monospace',
          fontSize: '1rem',
          fontWeight: 600,
          letterSpacing: '0.1em',
          color: 'var(--sa-text-bright)',
          marginBottom: '0.75rem',
        }}
      >
        Pack Not Found
      </h1>
      <p style={{ color: 'var(--sa-text-dim)', fontSize: '0.85rem', marginBottom: '2rem' }}>
        This page does not exist or the pack has been removed.
      </p>
      <Link href="/packs" className="install-btn">
        Browse Packs
      </Link>
    </div>
  );
}
