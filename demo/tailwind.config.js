/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: "class",
  content: [
    "./src/**/*.{rs,html,css,tsx}",
    "./dist/**/*.html",
    "../frontend/**/*.{rs,html,css,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        poppins: ["Poppins", "serif"],
      },
      keyframes: {
        highlight: {
          "0%": { background: "#8f8" },
          "100%": { background: "auto" },
        },
        "fade-in": {
          from: { opacity: 0 },
          to: { opacity: 100 },
        },
        "fade-out": {
          from: { opacity: 100 },
          to: { opacity: 0 },
        },
      },
      animation: {
        highlight: "highlight 1s",
        "fade-in": "fade-in 1s linear",
        "fade-out": "fade-out 1s linear",
      },
      colors: {
        primary: {
          light: "#4f46e5",
          DEFAULT: "#3b82f6",
          dark: "#2563eb",
        },
        danger: "tomato",
      },
      screens: {
        sm: "640px",
        md: "768px",
        lg: "1024px",
        xl: "1280px",
        "2xl": "1536px",
        "3xl": "1728px",
      },
    },
    container: {
      center: true,
      padding: {
        DEFAULT: "2rem",
        sm: 0,
      },
    },
  },
  plugins: [],
};
