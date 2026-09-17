// Monaco-grade Myers/LCS Line Difference Engine and Side-by-Side Model.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HunkKind {
    Added,
    Deleted,
    Modified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffLineKind {
    Unchanged,
    Added,
    Deleted,
    Empty, // Spacer row to keep side-by-side panes vertically aligned
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffHunk {
    pub original_start: usize, // 0-based line index
    pub original_count: usize, // number of lines
    pub modified_start: usize, // 0-based line index
    pub modified_count: usize, // number of lines
    pub kind: HunkKind,
}

#[derive(Clone, Debug)]
pub struct AlignedDiffRow {
    pub original_line_no: Option<usize>, // 1-based display
    pub original_text: Option<String>,
    pub original_kind: DiffLineKind,
    pub modified_line_no: Option<usize>, // 1-based display
    pub modified_text: Option<String>,
    pub modified_kind: DiffLineKind,
    pub hunk_idx: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct UnifiedDiffRow {
    pub kind: DiffLineKind,
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub text: String,
    pub hunk_idx: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct DiffModel {
    pub original_title: String,
    pub modified_title: String,
    pub file_path: String,
    pub original_lines: Vec<String>,
    pub modified_lines: Vec<String>,
    pub hunks: Vec<DiffHunk>,
    pub aligned_rows: Vec<AlignedDiffRow>,
    pub unified_rows: Vec<UnifiedDiffRow>,
    pub active_hunk_idx: usize,
    pub is_side_by_side: bool,
    pub scroll_row: usize,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EditOp {
    Equal(usize, usize), // orig_idx, mod_idx
    Delete(usize),       // orig_idx
    Insert(usize),       // mod_idx
}

impl DiffModel {
    pub fn new(
        original_title: String,
        original_content: &str,
        modified_title: String,
        modified_content: &str,
        file_path: String,
    ) -> Self {
        let orig_lines: Vec<String> = original_content.lines().map(|s| s.to_string()).collect();
        let mod_lines: Vec<String> = modified_content.lines().map(|s| s.to_string()).collect();

        let mut model = Self {
            original_title,
            modified_title,
            file_path,
            original_lines: orig_lines,
            modified_lines: mod_lines,
            hunks: Vec::new(),
            aligned_rows: Vec::new(),
            unified_rows: Vec::new(),
            active_hunk_idx: 0,
            is_side_by_side: true,
            scroll_row: 0,
            additions: 0,
            deletions: 0,
        };

        model.recompute();
        model
    }

    pub fn recompute(&mut self) {
        let ops = compute_myers_diff(&self.original_lines, &self.modified_lines);
        let hunks = group_edits_into_hunks(&ops, &self.original_lines, &self.modified_lines);

        let mut additions = 0;
        let mut deletions = 0;
        for hunk in &hunks {
            additions += hunk.modified_count;
            deletions += hunk.original_count;
        }

        let aligned = build_aligned_rows(&ops, &self.original_lines, &self.modified_lines, &hunks);
        let unified = build_unified_rows(&ops, &self.original_lines, &self.modified_lines, &hunks);

        self.hunks = hunks;
        self.aligned_rows = aligned;
        self.unified_rows = unified;
        self.additions = additions;
        self.deletions = deletions;

        if self.hunks.is_empty() {
            self.active_hunk_idx = 0;
        } else if self.active_hunk_idx >= self.hunks.len() {
            self.active_hunk_idx = self.hunks.len() - 1;
        }
    }

    pub fn next_hunk(&mut self) {
        if self.hunks.is_empty() {
            return;
        }
        self.active_hunk_idx = (self.active_hunk_idx + 1) % self.hunks.len();
        self.scroll_to_active_hunk();
    }

    pub fn prev_hunk(&mut self) {
        if self.hunks.is_empty() {
            return;
        }
        if self.active_hunk_idx == 0 {
            self.active_hunk_idx = self.hunks.len() - 1;
        } else {
            self.active_hunk_idx -= 1;
        }
        self.scroll_to_active_hunk();
    }

    pub fn scroll_to_active_hunk(&mut self) {
        if let Some(hunk) = self.hunks.get(self.active_hunk_idx) {
            // Find row in aligned_rows
            if let Some(row_idx) = self
                .aligned_rows
                .iter()
                .position(|r| r.hunk_idx == Some(self.active_hunk_idx))
            {
                self.scroll_row = row_idx.saturating_sub(2);
            } else {
                self.scroll_row = hunk.modified_start.saturating_sub(2);
            }
        }
    }

    pub fn revert_hunk(&mut self, hunk_idx: usize) {
        if hunk_idx >= self.hunks.len() {
            return;
        }
        let hunk = &self.hunks[hunk_idx];
        let orig_slice = if hunk.original_count > 0 {
            self.original_lines[hunk.original_start..(hunk.original_start + hunk.original_count)]
                .to_vec()
        } else {
            Vec::new()
        };

        // Replace range in modified_lines
        let mod_start = hunk.modified_start;
        let mod_end = mod_start + hunk.modified_count;
        if mod_start <= self.modified_lines.len() && mod_end <= self.modified_lines.len() {
            self.modified_lines.splice(mod_start..mod_end, orig_slice);
        }

        self.recompute();
    }

    pub fn toggle_mode(&mut self) {
        self.is_side_by_side = !self.is_side_by_side;
    }

    pub fn modified_content(&self) -> String {
        self.modified_lines.join("\n")
    }
}

// ── Myers Diff Algorithm ─────────────────────────────────────────────────────

fn compute_myers_diff(a: &[String], b: &[String]) -> Vec<EditOp> {
    let n = a.len();
    let m = b.len();

    if n == 0 && m == 0 {
        return Vec::new();
    }
    if n == 0 {
        return (0..m).map(EditOp::Insert).collect();
    }
    if m == 0 {
        return (0..n).map(EditOp::Delete).collect();
    }

    let max_d = n + m;
    let offset = max_d as isize;
    let mut v = vec![0usize; (2 * max_d + 1) as usize];
    let mut trace: Vec<Vec<usize>> = Vec::new();

    let mut found = false;
    for d in 0..=max_d {
        trace.push(v.clone());
        let mut k = -(d as isize);
        while k <= d as isize {
            let k_idx = (k + offset) as usize;
            let mut x = if k == -(d as isize) || (k != d as isize && v[k_idx - 1] < v[k_idx + 1]) {
                v[k_idx + 1]
            } else {
                v[k_idx - 1] + 1
            };

            let mut y = (x as isize - k) as usize;
            while x < n && y < m && a[x] == b[y] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;
            if x >= n && y >= m {
                found = true;
                break;
            }
            k += 2;
        }
        if found {
            break;
        }
    }

    // Backtrack to reconstruct edit script
    let mut ops = Vec::new();
    let mut x = n;
    let mut y = m;

    for d in (1..trace.len()).rev() {
        let v_prev = &trace[d];
        let k = x as isize - y as isize;
        let k_idx = (k + offset) as usize;

        let prev_k =
            if k == -(d as isize) || (k != d as isize && v_prev[k_idx - 1] < v_prev[k_idx + 1]) {
                k + 1
            } else {
                k - 1
            };

        let prev_k_idx = (prev_k + offset) as usize;
        let prev_x = v_prev[prev_k_idx];
        let prev_y = (prev_x as isize - prev_k) as usize;

        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;
            ops.push(EditOp::Equal(x, y));
        }

        if d > 0 {
            if x == prev_x {
                y -= 1;
                ops.push(EditOp::Insert(y));
            } else {
                x -= 1;
                ops.push(EditOp::Delete(x));
            }
        }
    }

    while x > 0 && y > 0 {
        x -= 1;
        y -= 1;
        ops.push(EditOp::Equal(x, y));
    }

    ops.reverse();
    ops
}

fn group_edits_into_hunks(ops: &[EditOp], a: &[String], b: &[String]) -> Vec<DiffHunk> {
    let _ = (a, b);
    let mut hunks = Vec::new();
    let mut i = 0;

    while i < ops.len() {
        match ops[i] {
            EditOp::Equal(_, _) => {
                i += 1;
            }
            EditOp::Delete(_) | EditOp::Insert(_) => {
                let hunk_start_i = i;
                let mut orig_del_count = 0;
                let mut mod_ins_count = 0;
                let mut first_orig = None;
                let mut first_mod = None;

                while i < ops.len() {
                    match ops[i] {
                        EditOp::Delete(o) => {
                            if first_orig.is_none() {
                                first_orig = Some(o);
                            }
                            orig_del_count += 1;
                            i += 1;
                        }
                        EditOp::Insert(m) => {
                            if first_mod.is_none() {
                                first_mod = Some(m);
                            }
                            mod_ins_count += 1;
                            i += 1;
                        }
                        EditOp::Equal(_, _) => break,
                    }
                }

                // If first_orig was None, find from surrounding ops
                let orig_start = first_orig.unwrap_or_else(|| {
                    if hunk_start_i > 0 {
                        if let EditOp::Equal(o, _) = ops[hunk_start_i - 1] {
                            o + 1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                });

                let mod_start = first_mod.unwrap_or_else(|| {
                    if hunk_start_i > 0 {
                        if let EditOp::Equal(_, m) = ops[hunk_start_i - 1] {
                            m + 1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                });

                let kind = if orig_del_count > 0 && mod_ins_count > 0 {
                    HunkKind::Modified
                } else if mod_ins_count > 0 {
                    HunkKind::Added
                } else {
                    HunkKind::Deleted
                };

                hunks.push(DiffHunk {
                    original_start: orig_start,
                    original_count: orig_del_count,
                    modified_start: mod_start,
                    modified_count: mod_ins_count,
                    kind,
                });
            }
        }
    }

    hunks
}

fn build_aligned_rows(
    ops: &[EditOp],
    a: &[String],
    b: &[String],
    hunks: &[DiffHunk],
) -> Vec<AlignedDiffRow> {
    let mut rows = Vec::new();
    let mut hunk_idx_cursor = 0;
    let mut i = 0;

    while i < ops.len() {
        match ops[i] {
            EditOp::Equal(o, m) => {
                rows.push(AlignedDiffRow {
                    original_line_no: Some(o + 1),
                    original_text: Some(a[o].clone()),
                    original_kind: DiffLineKind::Unchanged,
                    modified_line_no: Some(m + 1),
                    modified_text: Some(b[m].clone()),
                    modified_kind: DiffLineKind::Unchanged,
                    hunk_idx: None,
                });
                i += 1;
            }
            EditOp::Delete(_) | EditOp::Insert(_) => {
                let cur_hunk = if hunk_idx_cursor < hunks.len() {
                    let idx = hunk_idx_cursor;
                    hunk_idx_cursor += 1;
                    Some(idx)
                } else {
                    None
                };

                let mut del_indices = Vec::new();
                let mut ins_indices = Vec::new();

                while i < ops.len() {
                    match ops[i] {
                        EditOp::Delete(o) => {
                            del_indices.push(o);
                            i += 1;
                        }
                        EditOp::Insert(m) => {
                            ins_indices.push(m);
                            i += 1;
                        }
                        EditOp::Equal(_, _) => break,
                    }
                }

                let max_len = del_indices.len().max(ins_indices.len());
                for step in 0..max_len {
                    let orig_info = if step < del_indices.len() {
                        let o = del_indices[step];
                        (Some(o + 1), Some(a[o].clone()), DiffLineKind::Deleted)
                    } else {
                        (None, None, DiffLineKind::Empty)
                    };

                    let mod_info = if step < ins_indices.len() {
                        let m = ins_indices[step];
                        (Some(m + 1), Some(b[m].clone()), DiffLineKind::Added)
                    } else {
                        (None, None, DiffLineKind::Empty)
                    };

                    rows.push(AlignedDiffRow {
                        original_line_no: orig_info.0,
                        original_text: orig_info.1,
                        original_kind: orig_info.2,
                        modified_line_no: mod_info.0,
                        modified_text: mod_info.1,
                        modified_kind: mod_info.2,
                        hunk_idx: cur_hunk,
                    });
                }
            }
        }
    }

    rows
}

fn build_unified_rows(
    ops: &[EditOp],
    a: &[String],
    b: &[String],
    hunks: &[DiffHunk],
) -> Vec<UnifiedDiffRow> {
    let mut rows = Vec::new();
    let mut hunk_idx_cursor = 0;
    let mut i = 0;

    while i < ops.len() {
        match ops[i] {
            EditOp::Equal(o, m) => {
                rows.push(UnifiedDiffRow {
                    kind: DiffLineKind::Unchanged,
                    old_line_no: Some(o + 1),
                    new_line_no: Some(m + 1),
                    text: a[o].clone(),
                    hunk_idx: None,
                });
                i += 1;
            }
            EditOp::Delete(_) | EditOp::Insert(_) => {
                let cur_hunk = if hunk_idx_cursor < hunks.len() {
                    let idx = hunk_idx_cursor;
                    hunk_idx_cursor += 1;
                    Some(idx)
                } else {
                    None
                };

                // Group all deletes first, then inserts
                while i < ops.len() {
                    if let EditOp::Delete(o) = ops[i] {
                        rows.push(UnifiedDiffRow {
                            kind: DiffLineKind::Deleted,
                            old_line_no: Some(o + 1),
                            new_line_no: None,
                            text: a[o].clone(),
                            hunk_idx: cur_hunk,
                        });
                        i += 1;
                    } else {
                        break;
                    }
                }

                while i < ops.len() {
                    if let EditOp::Insert(m) = ops[i] {
                        rows.push(UnifiedDiffRow {
                            kind: DiffLineKind::Added,
                            old_line_no: None,
                            new_line_no: Some(m + 1),
                            text: b[m].clone(),
                            hunk_idx: cur_hunk,
                        });
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_identical() {
        let text = "line1\nline2\nline3";
        let model = DiffModel::new("orig".into(), text, "mod".into(), text, "file.rs".into());
        assert_eq!(model.hunks.len(), 0);
        assert_eq!(model.additions, 0);
        assert_eq!(model.deletions, 0);
        assert_eq!(model.aligned_rows.len(), 3);
    }

    #[test]
    fn test_diff_pure_addition() {
        let orig = "line1\nline3";
        let modded = "line1\nline2\nline3";
        let model = DiffModel::new("orig".into(), orig, "mod".into(), modded, "file.rs".into());
        assert_eq!(model.hunks.len(), 1);
        assert_eq!(model.hunks[0].kind, HunkKind::Added);
        assert_eq!(model.additions, 1);
        assert_eq!(model.deletions, 0);
        assert_eq!(model.aligned_rows.len(), 3);
        // Middle row should be Empty on left, Added on right
        assert_eq!(model.aligned_rows[1].original_kind, DiffLineKind::Empty);
        assert_eq!(model.aligned_rows[1].modified_kind, DiffLineKind::Added);
        assert_eq!(
            model.aligned_rows[1].modified_text.as_deref(),
            Some("line2")
        );
    }

    #[test]
    fn test_diff_pure_deletion() {
        let orig = "line1\nline2\nline3";
        let modded = "line1\nline3";
        let model = DiffModel::new("orig".into(), orig, "mod".into(), modded, "file.rs".into());
        assert_eq!(model.hunks.len(), 1);
        assert_eq!(model.hunks[0].kind, HunkKind::Deleted);
        assert_eq!(model.additions, 0);
        assert_eq!(model.deletions, 1);
        assert_eq!(model.aligned_rows.len(), 3);
        // Middle row should be Deleted on left, Empty on right
        assert_eq!(model.aligned_rows[1].original_kind, DiffLineKind::Deleted);
        assert_eq!(model.aligned_rows[1].modified_kind, DiffLineKind::Empty);
    }

    #[test]
    fn test_diff_modification_and_revert() {
        let orig = "fn main() {\n    println!(\"old\");\n}";
        let modded = "fn main() {\n    println!(\"new 1\");\n    println!(\"new 2\");\n}";
        let mut model = DiffModel::new("orig".into(), orig, "mod".into(), modded, "file.rs".into());
        assert_eq!(model.hunks.len(), 1);
        assert_eq!(model.hunks[0].kind, HunkKind::Modified);
        assert_eq!(model.deletions, 1);
        assert_eq!(model.additions, 2);

        // Revert hunk
        model.revert_hunk(0);
        assert_eq!(model.hunks.len(), 0);
        assert_eq!(model.additions, 0);
        assert_eq!(model.deletions, 0);
        assert_eq!(model.modified_content(), orig);
    }

    #[test]
    fn test_diff_navigation() {
        let orig = "a\nb\nc\nd\ne";
        let modded = "a\nB_MOD\nc\nD_MOD\ne";
        let mut model = DiffModel::new("orig".into(), orig, "mod".into(), modded, "file.rs".into());
        assert_eq!(model.hunks.len(), 2);
        assert_eq!(model.active_hunk_idx, 0);

        model.next_hunk();
        assert_eq!(model.active_hunk_idx, 1);

        model.next_hunk();
        assert_eq!(model.active_hunk_idx, 0);

        model.prev_hunk();
        assert_eq!(model.active_hunk_idx, 1);
    }
}
