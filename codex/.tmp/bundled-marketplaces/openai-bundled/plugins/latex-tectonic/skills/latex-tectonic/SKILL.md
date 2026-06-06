---
name: LaTeX Tectonic
description: Compile LaTeX and TeX documents with the bundled Tectonic executable.
---

# LaTeX Tectonic

Use this skill when the user asks to compile, preview, build, or troubleshoot a LaTeX or TeX document with Tectonic.

The plugin bundles Tectonic under the plugin root:

- macOS/Linux: `bin/tectonic`
- Windows: `bin/tectonic.exe`

Resolve the plugin root from this `SKILL.md` file by going two directories up from `skills/latex-tectonic/`.

Use `scripts/tectonic-path.mjs` when another script needs the executable path:

```bash
node scripts/tectonic-path.mjs
```

For normal compilation, run the bundled executable directly from the document directory:

```bash
<plugin-root>/bin/tectonic --outdir <output-directory> <tex-file>
```

Prefer writing generated PDFs and auxiliary files into an explicit output directory. Do not install a system TeX distribution unless the user asks for that fallback.
