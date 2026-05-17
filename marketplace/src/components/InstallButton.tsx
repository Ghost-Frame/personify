'use client';

interface InstallButtonProps {
  deepLink: string;
  packName: string;
}

export function InstallButton({ deepLink, packName }: InstallButtonProps) {
  const handleInstall = () => {
    window.location.href = deepLink;
  };

  return (
    <div>
      <button
        className="install-btn"
        style={{ width: '100%', justifyContent: 'center', fontSize: '0.8rem', padding: '1rem 2rem' }}
        onClick={handleInstall}
        aria-label={`Open ${packName} in FrameShift Desktop`}
      >
        <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
          <path d="M9 1v10M5 7l4 4 4-4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          <path d="M2 13v2a1 1 0 001 1h12a1 1 0 001-1v-2" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
        </svg>
        Open in FrameShift Desktop
      </button>
      <div
        style={{
          marginTop: '0.75rem',
          fontFamily: 'Orbitron, monospace',
          fontSize: '0.55rem',
          letterSpacing: '0.1em',
          color: 'var(--sa-text-dim)',
          textTransform: 'uppercase',
          textAlign: 'center',
        }}
      >
        Requires FrameShift Desktop to be installed
      </div>
      <div
        style={{
          marginTop: '0.5rem',
          fontFamily: 'monospace',
          fontSize: '0.65rem',
          color: 'var(--sa-border)',
          wordBreak: 'break-all',
          textAlign: 'center',
          padding: '0.4rem 0.6rem',
          background: 'rgba(0,0,0,0.3)',
          borderRadius: '4px',
          border: '1px solid var(--sa-border)',
          userSelect: 'all',
          cursor: 'text',
        }}
      >
        {deepLink}
      </div>
    </div>
  );
}
