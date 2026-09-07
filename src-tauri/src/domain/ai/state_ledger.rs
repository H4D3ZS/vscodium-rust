//! Long-horizon state ledger — the fix for state drift.
//!
//! The benchmarks are stark: short-task success of 40–50% collapses below 10%
//! once a task is embedded in long history (Long-Horizon Task Mirage, 2026).
//! Agents lose the thread — they forget the goal, re-derive settled decisions
//! wrongly, and let generated code drift toward "structural attractors" that
//! pass tests but decay in quality.
//!
//! This module is the durable spine a long task hangs on: an append-only ledger
//! of the **goal**, the **decisions** made (with rationale), the **facts**
//! established, and the **open questions**. It renders a compact re-grounding
//! block to re-inject each turn (so the goal never falls out of the window), and
//! it flags **drift** — when the current action no longer serves the recorded
//! goal — and **code bloat** — when a file's complexity climbs across edits.
//!
//! Deterministic and self-contained; opt-in via `KORTEX_LEDGER`.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    /// The task goal (usually one, set first).
    Goal,
    /// A decision made, with its rationale — the thing agents re-derive wrongly.
    Decision,
    /// An established fact / constraint discovered during the task.
    Fact,
    /// An open question still to resolve.
    Question,
}

#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    pub kind: EntryKind,
    pub text: String,
    /// Monotonic turn index the entry was recorded on.
    pub turn: usize,
    /// True once an open question has been answered/closed.
    pub resolved: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StateLedger {
    entries: Vec<Entry>,
    turn: usize,
}

impl StateLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance the turn counter (call once per agent turn).
    pub fn tick(&mut self) {
        self.turn += 1;
    }

    pub fn turn(&self) -> usize {
        self.turn
    }

    fn add(&mut self, kind: EntryKind, text: impl Into<String>) {
        let text = text.into();
        if text.trim().is_empty() {
            return;
        }
        self.entries.push(Entry { kind, text, turn: self.turn, resolved: false });
    }

    /// Set (or reset) the goal. Kept as the first Goal entry; a new goal appends
    /// so the trajectory is auditable, but `goal()` returns the latest.
    pub fn set_goal(&mut self, text: impl Into<String>) {
        self.add(EntryKind::Goal, text);
    }
    pub fn decide(&mut self, text: impl Into<String>) {
        self.add(EntryKind::Decision, text);
    }
    pub fn fact(&mut self, text: impl Into<String>) {
        self.add(EntryKind::Fact, text);
    }
    pub fn ask(&mut self, text: impl Into<String>) {
        self.add(EntryKind::Question, text);
    }

    /// The current goal (latest Goal entry).
    pub fn goal(&self) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.kind == EntryKind::Goal)
            .map(|e| e.text.as_str())
    }

    /// Mark the first matching open question resolved.
    pub fn resolve_question(&mut self, needle: &str) -> bool {
        let needle = needle.to_lowercase();
        for e in self.entries.iter_mut() {
            if e.kind == EntryKind::Question && !e.resolved && e.text.to_lowercase().contains(&needle) {
                e.resolved = true;
                return true;
            }
        }
        false
    }

    pub fn open_questions(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.kind == EntryKind::Question && !e.resolved)
            .map(|e| e.text.as_str())
            .collect()
    }

    /// Render a compact re-grounding block to re-inject each turn so the goal
    /// and settled decisions never fall out of the context window. Bounded to
    /// the most recent `max_each` decisions/facts.
    pub fn reground(&self, max_each: usize) -> String {
        let mut out = String::from("## Task state (re-grounding)\n");
        if let Some(g) = self.goal() {
            out.push_str(&format!("Goal: {g}\n"));
        }
        let recent = |kind: EntryKind| -> Vec<&str> {
            self.entries
                .iter()
                .rev()
                .filter(|e| e.kind == kind)
                .take(max_each)
                .map(|e| e.text.as_str())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        };
        let decisions = recent(EntryKind::Decision);
        if !decisions.is_empty() {
            out.push_str("Decisions (don't re-litigate):\n");
            for d in decisions {
                out.push_str(&format!("- {d}\n"));
            }
        }
        let facts = recent(EntryKind::Fact);
        if !facts.is_empty() {
            out.push_str("Established facts:\n");
            for f in facts {
                out.push_str(&format!("- {f}\n"));
            }
        }
        let open = self.open_questions();
        if !open.is_empty() {
            out.push_str("Open questions:\n");
            for q in open {
                out.push_str(&format!("- {q}\n"));
            }
        }
        out
    }

    /// Drift check: does the described current action still share meaningful
    /// vocabulary with the goal? Returns a drift score in [0,1] (1 = fully
    /// drifted). Cheap lexical overlap — a coarse tripwire, not a judge.
    pub fn drift(&self, current_action: &str) -> f32 {
        let Some(goal) = self.goal() else {
            return 0.0; // no goal recorded → can't be drifting
        };
        let g = keyword_set(goal);
        let a = keyword_set(current_action);
        if g.is_empty() || a.is_empty() {
            return 0.0;
        }
        let overlap = g.iter().filter(|w| a.contains(*w)).count();
        1.0 - (overlap as f32 / g.len() as f32)
    }

    /// True when the action has drifted past `threshold` from the goal.
    pub fn is_drifting(&self, current_action: &str, threshold: f32) -> bool {
        self.drift(current_action) > threshold
    }
}

/// Content words of a phrase (lowercased, >3 chars, minus common stopwords).
fn keyword_set(s: &str) -> std::collections::HashSet<String> {
    const STOP: &[&str] = &[
        "the", "and", "for", "with", "that", "this", "from", "into", "your",
        "have", "will", "should", "make", "made", "when", "then", "than", "code",
        "file", "files", "using", "use", "add", "adds", "fix", "fixes",
    ];
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3 && !STOP.contains(w))
        .map(|w| w.to_string())
        .collect()
}

// ── Code-drift guard ─────────────────────────────────────────────────────────

/// A cheap proxy for a file's complexity, tracked across edits to catch the
/// "quality decay under repeated modification" the benchmarks flag: branch
/// density, nesting depth, and size. Not cyclomatic complexity — a fast tripwire.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ComplexitySnapshot {
    pub lines: usize,
    pub branches: usize,
    pub max_nesting: usize,
    /// Composite score — higher is more complex.
    pub score: f32,
}

/// Measure a source string's complexity proxy.
pub fn complexity(src: &str) -> ComplexitySnapshot {
    let lines = src.lines().filter(|l| !l.trim().is_empty()).count();
    let mut branches = 0usize;
    for kw in ["if ", "if(", "else", "for ", "for(", "while", "match ", "case ", "catch", "&&", "||", "?"] {
        branches += src.matches(kw).count();
    }
    // nesting via running brace depth
    let (mut depth, mut max_depth) = (0i32, 0i32);
    for ch in src.chars() {
        match ch {
            '{' | '(' | '[' => {
                depth += 1;
                max_depth = max_depth.max(depth);
            }
            '}' | ')' | ']' => depth = (depth - 1).max(0),
            _ => {}
        }
    }
    let max_nesting = max_depth.max(0) as usize;
    let score = lines as f32 * 0.1 + branches as f32 * 1.0 + max_nesting as f32 * 2.0;
    ComplexitySnapshot { lines, branches, max_nesting, score }
}

/// Did an edit make the file meaningfully worse? True when the new complexity
/// score exceeds the old by more than `ratio` (e.g. 0.5 = +50%) AND grew in
/// absolute terms — so a big refactor that *reduces* complexity never trips it.
pub fn regressed(before: &ComplexitySnapshot, after: &ComplexitySnapshot, ratio: f32) -> bool {
    after.score > before.score * (1.0 + ratio) && after.score > before.score + 3.0
}

// ── Process-global registry ──────────────────────────────────────────────────
//
// One ledger per workspace root, so the model's `task_state` tool calls persist
// across turns in the same session without threading a handle through every
// call site. Keyed by the workspace root path (as a string) — the one value
// every call site already has.

static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, StateLedger>>> =
    std::sync::OnceLock::new();

fn registry() -> &'static std::sync::Mutex<std::collections::HashMap<String, StateLedger>> {
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// Run `f` against the ledger for `workspace_key`, creating one if absent.
pub fn with_ledger<R>(workspace_key: &str, f: impl FnOnce(&mut StateLedger) -> R) -> R {
    let mut map = registry().lock().unwrap_or_else(|p| p.into_inner());
    let ledger = map.entry(workspace_key.to_string()).or_default();
    f(ledger)
}

#[cfg(test)]
pub fn clear_registry() {
    registry().lock().unwrap_or_else(|p| p.into_inner()).clear();
}

#[derive(Debug, Clone)]
pub struct LedgerConfig {
    pub enabled: bool,
    pub drift_threshold: f32,
}
impl LedgerConfig {
    pub fn from_env() -> Self {
        Self {
            enabled: matches!(
                std::env::var("KORTEX_LEDGER").ok().as_deref(),
                Some("1") | Some("true") | Some("on")
            ),
            drift_threshold: 0.85,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_and_regrounds() {
        let mut l = StateLedger::new();
        l.set_goal("implement a JSON parser for config files");
        l.decide("use a recursive-descent approach, not regex");
        l.fact("config files never exceed 1 MB");
        l.ask("should comments be supported?");
        let block = l.reground(5);
        assert!(block.contains("Goal: implement a JSON parser"));
        assert!(block.contains("recursive-descent"));
        assert!(block.contains("never exceed 1 MB"));
        assert!(block.contains("comments be supported"));
    }

    #[test]
    fn resolve_question_closes_it() {
        let mut l = StateLedger::new();
        l.ask("should comments be supported?");
        assert_eq!(l.open_questions().len(), 1);
        assert!(l.resolve_question("comments"));
        assert!(l.open_questions().is_empty());
    }

    #[test]
    fn goal_returns_latest() {
        let mut l = StateLedger::new();
        l.set_goal("first goal about parsing");
        l.set_goal("revised goal about formatting");
        assert_eq!(l.goal(), Some("revised goal about formatting"));
    }

    #[test]
    fn drift_low_when_on_task() {
        let mut l = StateLedger::new();
        l.set_goal("implement a recursive-descent JSON parser");
        assert!(l.drift("writing the recursive descent parser tokenizer") < 0.7);
    }

    #[test]
    fn drift_high_when_off_task() {
        let mut l = StateLedger::new();
        l.set_goal("implement a recursive-descent JSON parser");
        assert!(l.is_drifting("refactor the CSS theme colors in the sidebar", 0.85));
    }

    #[test]
    fn no_goal_means_no_drift() {
        let l = StateLedger::new();
        assert_eq!(l.drift("anything at all"), 0.0);
    }

    #[test]
    fn complexity_rises_with_branches_and_nesting() {
        let simple = complexity("fn a() { return 1; }");
        let complex = complexity(
            "fn a() { if x { if y { for i in z { if w && v { return 1; } } } } }",
        );
        assert!(complex.score > simple.score);
        assert!(complex.max_nesting > simple.max_nesting);
    }

    #[test]
    fn registry_persists_per_workspace_key() {
        clear_registry();
        with_ledger("/ws/a", |l| l.set_goal("goal for workspace a"));
        with_ledger("/ws/b", |l| l.set_goal("goal for workspace b"));
        with_ledger("/ws/a", |l| {
            assert_eq!(l.goal(), Some("goal for workspace a"));
        });
        with_ledger("/ws/b", |l| {
            assert_eq!(l.goal(), Some("goal for workspace b"));
        });
    }

    #[test]
    fn regressed_flags_bloat_not_cleanup() {
        let before = complexity("fn a() { if x { return 1; } }");
        let bloated = complexity(
            "fn a() { if x { if y { if z { for i in n { if a && b || c { return 1; } } } } } }",
        );
        assert!(regressed(&before, &bloated, 0.5));
        // a cleanup (fewer branches) must NOT be flagged
        let cleaned = complexity("fn a() { return lookup(x); }");
        assert!(!regressed(&before, &cleaned, 0.5));
    }
}
