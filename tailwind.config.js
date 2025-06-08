/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          dark: '#090013',
          purple: '#6405d6',
        },
        secondary: {
          gray: '#303030',
        },
        accent: {
          red: '#f1011d',
          pink: '#e52381',
        },
        text: {
          light: '#f5ecee',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backgroundImage: {
        'gradient-primary': 'linear-gradient(135deg, #6405d6, #f1011d)',
        'gradient-hover': 'linear-gradient(135deg, #7a15e6, #ff1127)',
      }
    },
  },
  plugins: [],
}