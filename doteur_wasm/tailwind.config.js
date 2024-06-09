/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "index.html"],
  theme: {
    extend: {
      height: {
        'dvh': '100dvh'
      }
    },
  },
  plugins: [require('daisyui'), require('tailwindcss-animated')],
}

