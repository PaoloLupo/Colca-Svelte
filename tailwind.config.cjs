const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
  daisyui: {
    styled: true,
    themes: [
      {
        colcadark: {
          "primary": "#2b2e44ff",
          "secondary": "#f45b69",
          "accent": "#EEAF3A",
          "neutral": "#1c1c27",
          "base-100": "#15151e",
          "info": "#038df7ff",
          "success": "#36D399",
          "warning": "#eec84a",
          "error": "#d90429",
        },
      },
    ],
  },
};

module.exports = config;
