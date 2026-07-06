/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#0F0F11',
        surface: '#1A1A1F',
        border: '#2A2A35',
        primary: '#4A90FA',
        secondary: '#7C5FDC',
        success: '#34D399',
        warning: '#FBBF24',
        error: '#F87171',
        textPrimary: '#F0F0F5',
        textSecondary: '#8A8A9A',
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
