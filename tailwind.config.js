/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html, js, svelte, ts}",
    "./static/**/*.{html, js, ts}",
    "./node_modules/preline/preline.js",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("preline/plugin")],
  darkMode: "selector",
};
