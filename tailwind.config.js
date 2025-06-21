/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"],
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Your custom color palette
        'dark-purple': '#130D2E',
        'light-purple': '#C5C2D3',
        'light-grey': '#F3F3F4',
        'off-white': '#FFFDF9',
        'light-tan': '#FFF7EF',
        'tan': '#F9EAD8',
        'pink': '#FF80AE',
        'aquamarine': '#32FFB4',
        'yellow': '#FFF832',
        
        // Semantic mappings
        background: '#FFFDF9', // off-white
        surface: '#FFF7EF', // light-tan
        border: '#F3F3F4', // light-grey
        
        // Text colors
        primary: '#130D2E', // dark-purple
        secondary: '#C5C2D3', // light-purple
        muted: '#C5C2D3', // light-purple
        
        // Status colors
        success: {
          bg: '#32FFB4', // aquamarine
          text: '#130D2E', // dark-purple
        },
        warning: {
          bg: '#FFF832', // yellow
          text: '#130D2E', // dark-purple
        },
        error: {
          bg: '#FF80AE', // pink
          text: '#130D2E', // dark-purple
        },
      },
      fontFamily: {
        sans: ["Ubuntu", "ui-sans-serif", "system-ui", "-apple-system", "sans-serif"],
      },
      fontWeight: {
        normal: '500', // Ubuntu Medium
        bold: '700', // Ubuntu Bold
      },
      boxShadow: {
        'chunky': '0 4px 6px -1px rgba(19, 13, 46, 0.1), 0 2px 4px -2px rgba(19, 13, 46, 0.1)',
        'chunky-hover': '0 6px 8px -1px rgba(19, 13, 46, 0.15), 0 4px 6px -2px rgba(19, 13, 46, 0.1)',
      },
    },
  },
  plugins: [],
}
