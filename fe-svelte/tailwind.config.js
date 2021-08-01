const colors = require("tailwindcss/colors");

module.exports = {
 mode: "jit",
 purge: ["./src/**/*.{html,js,jsx,ts,tsx,svelte}"],
 theme: {
  extend: {
   colors: {
    sky: colors.sky,
    cyan: colors.cyan,
   },
  },
 },
};
