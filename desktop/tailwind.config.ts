import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Map @ghost-frame/theme CSS vars for Tailwind usage
        "sa-bg": "var(--sa-bg)",
        "sa-surface": "var(--sa-surface)",
        "sa-surface-hover": "var(--sa-surface-hover)",
        "sa-border": "var(--sa-border)",
        "sa-text": "var(--sa-text)",
        "sa-text-dim": "var(--sa-text-dim)",
        "sa-text-bright": "var(--sa-text-bright)",
        "sa-accent": "var(--sa-accent)",
        "sa-pink": "var(--sa-pink)",
        "sa-download": "var(--sa-download)",
      },
    },
  },
  plugins: [],
};

export default config;
