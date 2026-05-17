import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "export",
  // Disable image optimization for static export
  images: {
    unoptimized: true,
  },
  // Tauri expects static files -- no trailing slash needed for file:// protocol
  trailingSlash: true,
};

export default nextConfig;
