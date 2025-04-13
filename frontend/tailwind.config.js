/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html"
  ],
  theme: {
    extend: {
        keyframes: {
            typing: {
              "0%": { maxWidth: "0", visibility: "visible" },
              "100%": { maxWidth: "1000px" }
            },
            blink: {
              "50%": { borderColor: "transparent" },
              "100%": { borderColor: "black" }
            }
          },
          animation: {
            typing: "typing 2s steps(50, end) infinite alternate, blink 0.7s step-end infinite"
          }
          
    },
  },
  plugins: [
    require("tailwindcss-animated")
  ],
}

