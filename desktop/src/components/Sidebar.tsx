"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

interface NavItem {
  href: string;
  label: string;
  icon: React.ReactNode;
}

function HomeIcon() {
  return (
    <svg className="sidebar-link-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path d="M2 6.5L8 2l6 4.5V14H10v-3H6v3H2V6.5z" />
    </svg>
  );
}

function PersonasIcon() {
  return (
    <svg className="sidebar-link-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
      <circle cx="8" cy="5" r="3" />
      <path d="M2 14c0-3.314 2.686-5 6-5s6 1.686 6 5" />
    </svg>
  );
}

function MarketplaceIcon() {
  return (
    <svg className="sidebar-link-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
      <rect x="1" y="7" width="14" height="8" rx="1" />
      <path d="M4 7V5a4 4 0 018 0v2" />
    </svg>
  );
}

function SettingsIcon() {
  return (
    <svg className="sidebar-link-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.5">
      <circle cx="8" cy="8" r="2.5" />
      <path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.05 3.05l1.41 1.41M11.54 11.54l1.41 1.41M3.05 12.95l1.41-1.41M11.54 4.46l1.41-1.41" />
    </svg>
  );
}

const NAV_ITEMS: NavItem[] = [
  { href: "/", label: "Dashboard", icon: <HomeIcon /> },
  { href: "/personas", label: "Personas", icon: <PersonasIcon /> },
  { href: "/marketplace", label: "Marketplace", icon: <MarketplaceIcon /> },
  { href: "/settings", label: "Settings", icon: <SettingsIcon /> },
];

export function Sidebar() {
  const pathname = usePathname();

  function isActive(href: string): boolean {
    if (href === "/") return pathname === "/";
    return pathname.startsWith(href);
  }

  return (
    <aside className="sidebar">
      <div className="sidebar-logo">
        <div className="sidebar-logo-text">FrameShift</div>
        <div className="sidebar-logo-sub">persona runtime</div>
      </div>
      <nav className="sidebar-nav">
        <div className="sidebar-section-label">Navigation</div>
        {NAV_ITEMS.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={`sidebar-link${isActive(item.href) ? " active" : ""}`}
          >
            {item.icon}
            {item.label}
          </Link>
        ))}
      </nav>
    </aside>
  );
}
