/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"],
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // ConvertSave Color Palette
        "dark-text": "#24262e",
        "secondary-text": "#919296",
        "light-bg": "#f4f1ed",
        "lighter-bg": "#e7e3df",
        "muted-bg": "#dbd7d5",
        white: "#ffffff",
        "blue-accent": "#3562e3",
        "mint-accent": "#91f4c2",
        "pink-accent": "#ef87ad",

        // Legacy color names (for backwards compatibility)
        "dark-purple": "#24262e",
        "light-purple": "#919296",
        "light-grey": "#e7e3df",
        "off-white": "#ffffff",
        "light-tan": "#f4f1ed",
        tan: "#dbd7d5",
        pink: "#ef87ad",
        aquamarine: "#91f4c2",
        yellow: "#3562e3",

        // Semantic mappings
        background: "#ffffff",
        surface: "#f4f1ed",
        border: "#e7e3df",

        // Text colors
        primary: "#24262e",
        secondary: "#919296",
        muted: "#919296",

        // Status colors
        success: {
          bg: "#91f4c2",
          text: "#24262e",
        },
        warning: {
          bg: "#3562e3",
          text: "#ffffff",
        },
        error: {
          bg: "#ef87ad",
          text: "#24262e",
        },
      },
      fontFamily: {
        sans: [
          "Ubuntu",
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "sans-serif",
        ],
      },
      fontWeight: {
        normal: "500", // Ubuntu Medium
        bold: "700", // Ubuntu Bold
      },
      boxShadow: {
        chunky:
          "0 4px 6px -1px rgba(36, 38, 46, 0.1), 0 2px 4px -2px rgba(36, 38, 46, 0.1)",
        "chunky-hover":
          "0 6px 8px -1px rgba(36, 38, 46, 0.15), 0 4px 6px -2px rgba(36, 38, 46, 0.1)",
      },
    },
  },
  plugins: [],
};
