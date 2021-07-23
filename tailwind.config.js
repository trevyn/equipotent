const colors = require('tailwindcss/colors')

module.exports = {
 mode: 'jit',
 purge: ['./*.html', './*.{js,jsx,ts,tsx,svelte}'],
 theme: {
  extend: {
    colors: {
      sky: colors.sky,
      cyan: colors.cyan,
    },
  },
},
 // specify other options here
};