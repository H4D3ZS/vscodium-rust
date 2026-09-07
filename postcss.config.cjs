// .cjs because package.json is ESM. Vite auto-loads this for the CSS pipeline.
//
// Tailwind v4's PostCSS plugin moved to a separate package. It also handles
// vendor prefixing internally (Lightning CSS) — autoprefixer is no longer
// wired in here for that reason, not dropped by oversight.
module.exports = {
    plugins: {
        '@tailwindcss/postcss': {},
    },
};
