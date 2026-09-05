// .cjs because package.json is ESM. Vite auto-loads this for the CSS pipeline.
module.exports = {
    plugins: {
        tailwindcss: { config: './tailwind.config.cjs' },
        autoprefixer: {},
    },
};
