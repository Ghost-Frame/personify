import type { Metadata } from 'next';
import Link from 'next/link';
import './globals.css';

export const metadata: Metadata = {
  title: 'FrameShift Marketplace',
  description: 'Browse and install FrameShift persona packs for your AI workflows.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        <link
          href="https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;900&family=Inter:wght@300;400;500;600&display=swap"
          rel="stylesheet"
        />
      </head>
      <body>
        <div className="scanlines" aria-hidden="true" />
        <header
          style={{
            background: 'rgba(10, 11, 16, 0.85)',
            backdropFilter: 'blur(12px)',
            borderBottom: '1px solid var(--sa-border)',
            position: 'sticky',
            top: 0,
            zIndex: 100,
          }}
        >
          <div
            style={{
              maxWidth: '1100px',
              margin: '0 auto',
              padding: '0 1.5rem',
              height: '56px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              gap: '1rem',
            }}
          >
            {/* Logo */}
            <Link href="/" style={{ display: 'flex', alignItems: 'center', gap: '0.6rem', textDecoration: 'none' }}>
              <svg width="22" height="22" viewBox="0 0 22 22" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <polygon points="11,1 21,8 21,17 11,21 1,17 1,8" fill="none" stroke="var(--sa-accent)" strokeWidth="1.5" />
                <polygon points="11,5 17,9 17,15 11,18 5,15 5,9" fill="var(--sa-accent)" opacity="0.15" />
                <line x1="11" y1="1" x2="11" y2="21" stroke="var(--sa-accent)" strokeWidth="0.75" opacity="0.4" />
                <line x1="1" y1="8" x2="21" y2="17" stroke="var(--sa-accent)" strokeWidth="0.75" opacity="0.4" />
                <line x1="21" y1="8" x2="1" y2="17" stroke="var(--sa-accent)" strokeWidth="0.75" opacity="0.4" />
              </svg>
              <span
                style={{
                  fontFamily: 'Orbitron, monospace',
                  fontWeight: 700,
                  fontSize: '0.9rem',
                  letterSpacing: '0.1em',
                  color: 'var(--sa-text-bright)',
                }}
              >
                FrameShift
              </span>
            </Link>

            {/* Nav links */}
            <nav style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
              <Link href="/packs" className="nav-link">Browse</Link>
              <Link href="/authors/ghost-frame" className="nav-link">Authors</Link>
            </nav>
          </div>
        </header>

        <main style={{ minHeight: 'calc(100vh - 56px)' }}>
          {children}
        </main>

        <footer
          style={{
            borderTop: '1px solid var(--sa-border)',
            padding: '2rem 1.5rem',
            textAlign: 'center',
            color: 'var(--sa-text-dim)',
            fontSize: '0.75rem',
            fontFamily: 'Orbitron, monospace',
            letterSpacing: '0.08em',
          }}
        >
          FrameShift Marketplace -- persona packs for structured AI workflows
        </footer>
      </body>
    </html>
  );
}
