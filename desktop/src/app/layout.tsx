import type { Metadata } from "next";
import "@ghost-frame/theme/style.css";
import "./globals.css";
import { Sidebar } from "@/components/Sidebar";

export const metadata: Metadata = {
  title: "FrameShift",
  description: "Persona management for Claude",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <div className="app-shell">
          <Sidebar />
          <main className="app-content">{children}</main>
        </div>
      </body>
    </html>
  );
}
