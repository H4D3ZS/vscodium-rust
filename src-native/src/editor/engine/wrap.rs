// Soft Word Wrapping Engine: maps logical buffer rows into wrapped visual lines
// without mutating the underlying Piece Table text buffer.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualLine {
    pub buffer_row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub is_continuation: bool,
}

#[derive(Clone, Debug)]
pub struct WordWrapEngine {
    pub wrap_width: usize,
    pub enabled: bool,
    pub visual_lines: Vec<VisualLine>,
    // Mapping from buffer_row -> index of first visual line for that buffer row
    buffer_to_visual_map: Vec<usize>,
}

impl Default for WordWrapEngine {
    fn default() -> Self {
        Self::new(100)
    }
}

impl WordWrapEngine {
    pub fn new(wrap_width: usize) -> Self {
        Self {
            wrap_width: wrap_width.max(20),
            enabled: false,
            visual_lines: Vec::new(),
            buffer_to_visual_map: Vec::new(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }

    pub fn set_wrap_width(&mut self, width: usize) {
        self.wrap_width = width.max(20);
    }

    /// Recompute visual lines from the buffer's logical lines
    pub fn recompute(&mut self, buffer_lines: &[String]) {
        self.visual_lines.clear();
        self.buffer_to_visual_map.clear();
        self.buffer_to_visual_map.reserve(buffer_lines.len());

        for (buf_row, line) in buffer_lines.iter().enumerate() {
            let first_vis_idx = self.visual_lines.len();
            self.buffer_to_visual_map.push(first_vis_idx);

            if !self.enabled || line.chars().count() <= self.wrap_width {
                // No wrapping needed for this line
                self.visual_lines.push(VisualLine {
                    buffer_row: buf_row,
                    start_col: 0,
                    end_col: line.chars().count(),
                    is_continuation: false,
                });
            } else {
                // Break into visual chunks at word boundaries where possible
                let chars: Vec<char> = line.chars().collect();
                let char_len = chars.len();
                let mut start = 0;

                while start < char_len {
                    let mut end = (start + self.wrap_width).min(char_len);

                    // If not at the end of the line, search backward for a whitespace break
                    if end < char_len {
                        let mut break_pos = None;
                        for i in (start + (self.wrap_width / 2)..=end).rev() {
                            if chars[i].is_whitespace()
                                || chars[i] == '-'
                                || chars[i] == '_'
                                || chars[i] == ','
                                || chars[i] == ';'
                            {
                                break_pos = Some(i + 1); // break after punctuation/space
                                break;
                            }
                        }

                        if let Some(bp) = break_pos {
                            end = bp;
                        }
                    }

                    self.visual_lines.push(VisualLine {
                        buffer_row: buf_row,
                        start_col: start,
                        end_col: end,
                        is_continuation: start > 0,
                    });

                    start = end;
                }
            }
        }
    }

    pub fn visual_line_count(&self) -> usize {
        self.visual_lines.len()
    }

    pub fn get_visual_line(&self, visual_idx: usize) -> Option<&VisualLine> {
        self.visual_lines.get(visual_idx)
    }

    /// Map (buffer_row, buffer_col) to (visual_line_idx, visual_col)
    pub fn buffer_to_visual(&self, buf_row: usize, buf_col: usize) -> (usize, usize) {
        if !self.enabled || self.buffer_to_visual_map.is_empty() {
            return (buf_row, buf_col);
        }

        let first_vis_idx = match self.buffer_to_visual_map.get(buf_row) {
            Some(&idx) => idx,
            None => return (buf_row, buf_col),
        };

        // Walk visual lines that belong to this buffer_row
        let mut idx = first_vis_idx;
        while idx < self.visual_lines.len() && self.visual_lines[idx].buffer_row == buf_row {
            let vis = &self.visual_lines[idx];
            if buf_col >= vis.start_col && buf_col <= vis.end_col {
                return (idx, buf_col.saturating_sub(vis.start_col));
            }
            idx += 1;
        }

        // Default to the last visual line of this buffer row
        let last_idx = idx.saturating_sub(1).max(first_vis_idx);
        let vis = &self.visual_lines[last_idx];
        (last_idx, buf_col.saturating_sub(vis.start_col))
    }

    /// Map (visual_line_idx, visual_col) to (buffer_row, buffer_col)
    pub fn visual_to_buffer(&self, visual_idx: usize, visual_col: usize) -> (usize, usize) {
        if !self.enabled || visual_idx >= self.visual_lines.len() {
            return (visual_idx, visual_col);
        }

        let vis = &self.visual_lines[visual_idx];
        let buf_col = (vis.start_col + visual_col).min(vis.end_col);
        (vis.buffer_row, buf_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_disabled() {
        let lines = vec![
            "short line".to_string(),
            "a very long line that would exceed forty characters easily in normal circumstances"
                .to_string(),
        ];
        let mut engine = WordWrapEngine::new(40);
        engine.recompute(&lines);

        assert_eq!(engine.visual_line_count(), 2);
        assert_eq!(engine.buffer_to_visual(0, 5), (0, 5));
        assert_eq!(engine.visual_to_buffer(1, 10), (1, 10));
    }

    #[test]
    fn test_wrap_enabled() {
        let lines = vec![
            "short".to_string(),
            "hello world this is a test of the soft wrapping algorithm in pure rust".to_string(),
        ];
        let mut engine = WordWrapEngine::new(25);
        engine.set_enabled(true);
        engine.recompute(&lines);

        assert!(engine.visual_line_count() > 2);
        let (vis_row, _) = engine.buffer_to_visual(0, 2);
        assert_eq!(vis_row, 0);

        // Continuation line has is_continuation == true
        assert!(engine.visual_lines[2].is_continuation);
    }

    #[test]
    fn test_coordinate_roundtrip() {
        let lines = vec![
            "one two three four five six seven eight nine ten eleven twelve thirteen fourteen"
                .to_string(),
        ];
        let mut engine = WordWrapEngine::new(30);
        engine.set_enabled(true);
        engine.recompute(&lines);

        for col in [0, 10, 25, 35, 50] {
            let (v_row, v_col) = engine.buffer_to_visual(0, col);
            let (b_row, b_col) = engine.visual_to_buffer(v_row, v_col);
            assert_eq!(b_row, 0);
            assert_eq!(b_col, col);
        }
    }
}
