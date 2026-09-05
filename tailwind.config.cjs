const { heroui } = require('@heroui/react');

/**
 * Tailwind v3 config for the HeroUI layer. (.cjs because package.json is ESM.)
 *
 * CRITICAL: `preflight` is OFF. The IDE ships ~183 files of hand-rolled CSS
 * (styles.css / panes.css / settings.css + inline styles). Tailwind's preflight
 * reset would wipe those base styles globally. With it off, Tailwind only adds
 * utility/component classes — HeroUI coexists with the legacy CSS, no big-bang.
 *
 * @type {import('tailwindcss').Config}
 */
module.exports = {
    darkMode: 'class',
    content: [
        './src/**/*.{ts,tsx}',
        './node_modules/@heroui/theme/dist/**/*.{js,mjs}',
    ],
    corePlugins: {
        preflight: false,
    },
    theme: {
        extend: {},
    },
    plugins: [
        heroui({
            defaultTheme: 'dark',
            themes: {
                // Map HeroUI semantic tokens to the existing VS Code dark palette
                // (see :root in src/styles.css) so HeroUI components blend in.
                dark: {
                    colors: {
                        background: '#1e1e1e',
                        foreground: '#d4d4d4',
                        focus: '#007acc',
                        content1: '#252526',
                        content2: '#2d2d30',
                        content3: '#333333',
                        content4: '#3c3c3c',
                        divider: 'rgba(255,255,255,0.10)',
                        primary: { DEFAULT: '#2563eb', foreground: '#ffffff' },
                        secondary: { DEFAULT: '#007acc', foreground: '#ffffff' },
                        default: { DEFAULT: '#3c3c3c', foreground: '#d4d4d4' },
                        danger: { DEFAULT: '#f87171', foreground: '#ffffff' },
                        success: { DEFAULT: '#4ec994', foreground: '#0b1f17' },
                        warning: { DEFAULT: '#f0b429', foreground: '#1f1500' },
                    },
                },
            },
        }),
    ],
};
