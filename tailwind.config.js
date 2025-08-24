/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // POE2 inspired dark theme colors
        poe: {
          bg: {
            primary: '#0a0a0a',
            secondary: '#1a1a1a',
            tertiary: '#2a2a2a',
          },
          border: {
            primary: '#404040',
            secondary: '#606060',
          },
          text: {
            primary: '#ffffff',
            secondary: '#cccccc',
            muted: '#888888',
          },
          accent: {
            primary: '#d4af37', // Gold
            secondary: '#8b4513', // Brown
            danger: '#dc3545',
            success: '#28a745',
            warning: '#ffc107',
            info: '#17a2b8',
          }
        }
      },
      fontFamily: {
        'mono': ['JetBrains Mono', 'Consolas', 'Monaco', 'monospace'],
      },
      backdropBlur: {
        xs: '2px',
      }
    },
  },
  plugins: [],
}
