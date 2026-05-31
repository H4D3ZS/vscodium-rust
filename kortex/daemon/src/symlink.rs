use std::fs;
use std::path::Path;

/// Syncs the "1-Token Parameter" block into active local rules files for IDE auto-parsing.
pub struct NeuralSymlink;

impl NeuralSymlink {
    /// Injects a static block containing the Gist Token string to `CLAUDE.md` & `.cursorrules`
    pub fn inject_static_context_block(project_root: &Path, compressed_gist: &str) {
        let block = format!(
            "<!-- aim-vfs:static-context -->\n\
            AIM_PARAMETRIC_GIST={}\n\
            <!-- aim-vfs:end -->\n",
            compressed_gist
        );

        // Inject to .cursorrules for Cursor Prompt Caching hitting
        let cursor_rules = project_root.join(".cursorrules");
        let _ = fs::write(cursor_rules, &block);

        // Inject to CLAUDE.md for Claude Code
        let claude_md = project_root.join("CLAUDE.md");
        let _ = fs::write(claude_md, &block);
    }
}
