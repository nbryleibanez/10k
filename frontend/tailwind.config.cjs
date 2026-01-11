const { slate, violet, green, red } = require('@radix-ui/colors');

module.exports = {
  darkMode: 'class',
  content: ['./src/**/*.{html,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        brand: slate,
        accent: violet,
        success: green,
        danger: red
      }
    }
  },
  plugins: [require('@tailwindcss/forms')]
};
