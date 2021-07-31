const colors = require("tailwindcss/colors");

module.exports = {
 mode: "jit",
 purge: ["./src/*"],
 theme: {
  extend: {
   colors: {
    sky: colors.sky,
    cyan: colors.cyan,
   },
  },
 },
};
