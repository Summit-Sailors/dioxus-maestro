/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
    "../frontend/**/*.{rs,html,css}",
  ],
  theme: {
    extend: {
      keyframes: {
        highlight: {
          "0%": { background: "#8f8" },
          "100%": { background: "auto" },
        },
      },
      animation: { highlight: "highlight 1s" },
      fontFamily: {
        inter: ['Inter', 'sans-serif'],
      },
      colors: {
        primary: {
          light: "#4f46e5",
          DEFAULT: "#3b82f6",
          dark: "#2563eb",
        },
      },
    },
  },
  plugins: [],
};
