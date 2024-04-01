/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  theme: {
    fontFamily: {
      sans: ["NotoSansTC", "Arial"],
      serif: ["Playfair", "Georgia"],
      display: ["Silkscreen"],
    },
    extend: {},
  },
  plugins: [],
};
