/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css,tsx}",
    "./dist/**/*.html",
    "../frontend/**/*.{rs,html,css,tsx}",
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
        inter: ["Inter", "sans-serif"],
      },
      colors: {
        primary: {
          light: "#4f46e5",
          DEFAULT: "#3b82f6",
          dark: "#2563eb",
        },
      },
      screens: {
        sm: "640px",
        md: "768px",
        lg: "1024px",
        xl: "1280px",
        "2xl": "1536px",
      },
    },
    container: {
      padding: {
        DEFAULT: "1rem",
        sm: "2rem",
      },
    },
  },
  plugins: [],
  safelist: [
    "bg-gray-800",
    "hidden",
    "lg:block",
    "lg:hidden",
    "p-6",
    "fixed",
    "min-h-screen",
    "overflow-y-auto",
    "transition-all",
  ],
};
