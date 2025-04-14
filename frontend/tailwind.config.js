/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html"
  ],
  safelist: [ // based on colors in src/constants.rs
    "from-red-950",
    "via-red-950",
    "to-red-950",
    "text-red-950",
    "hover:text-red-950",

    "from-gray-800",
    "via-gray-800",
    "to-gray-800",
    "text-gray-800",
    "hover:text-gray-800",

    "from-red-100",
    "via-red-100",
    "to-red-100",
    "text-red-100",
    "hover:text-red-100",

    "from-gray-200",
    "via-gray-200",
    "to-gray-200",
    "text-gray-200",
    "hover:text-gray-200",

    "text-black",
    "border-r-black",
    "hover:text-black",
    "hover:border-r-black",

    "text-gray-600",
    "hover:text-gray-600",

    "decoration-red-500",
    "border-red-500",
    "outline-red-500",
    "text-red-500",
    "hover:decoration-red-500",
    "hover:border-red-500",
    "hover:outline-red-500",
    "hover:text-red-500",

    "text-gray-400",
    "hover:text-gray-400",

    "outline-gray-100",
    "text-gray-100",
    "hover:outline-gray-100",
    "hover:text-gray-100",
  ],
  theme: {
    extend: {
        keyframes: {
            blink: {
              "50%": { borderColor: "transparent" },
              "100%": { borderColor: "black" }
            }
          },
          animation: {
            blinking: "blink 0.7s step-end infinite"
          }
          
    },
  },
  plugins: [
    require("tailwindcss-animated")
  ],
}

