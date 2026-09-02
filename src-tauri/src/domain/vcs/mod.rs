//! Version control domain: git operations, checkpoints, surgical patching,
//! and the shadow workspace used for safe mutation before commit.

pub mod git;
pub mod git_checkpoints;
pub mod patch_engine;
pub mod semantic_firewall;
pub mod shadow_workspace;
