// Monaco Editor Minimap Code Overview Engine.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinimapMarkerKind {
    None,
    Error,
    Warning,
    SearchMatch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MinimapLine {
    pub line_idx: usize,
    pub indent_px: f32,
    pub width_px: f32,
    pub marker: MinimapMarkerKind,
    pub is_cursor: bool,
}

#[derive(Clone, Debug)]
pub struct MinimapModel {
    pub enabled: bool,
    pub width_px: f32,
    pub show_decorations: bool,
}

impl Default for MinimapModel {
    fn default() -> Self {
        Self {
            enabled: true,
            width_px: 90.0,
            show_decorations: true,
        }
    }
}

impl MinimapModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }

    /// Computes miniature representation for lines in the document.
    pub fn compute_lines(
        &self,
        lines: &[String],
        cursor_row: usize,
        error_lines: &[usize],
        warning_lines: &[usize],
        search_match_lines: &[usize],
        max_display_lines: usize,
    ) -> Vec<MinimapLine> {
        let total = lines.len();
        if total == 0 {
            return Vec::new();
        }

        // Sample lines if document exceeds max displayable lines
        let step = if total > max_display_lines {
            total as f32 / max_display_lines as f32
        } else {
            1.0
        };

        let count = total.min(max_display_lines);
        let mut result = Vec::with_capacity(count);
        let usable_width = (self.width_px - 8.0).max(10.0);
        let err_set: std::collections::HashSet<usize> = error_lines.iter().copied().collect();
        let warn_set: std::collections::HashSet<usize> = warning_lines.iter().copied().collect();
        let match_set: std::collections::HashSet<usize> =
            search_match_lines.iter().copied().collect();

        for i in 0..count {
            let line_idx = ((i as f32 * step) as usize).min(total - 1);
            let text = &lines[line_idx];

            let trimmed = text.trim_start();
            let leading_spaces = text.len() - trimmed.len();
            let content_len = trimmed.chars().count();

            let indent_px = ((leading_spaces as f32) * 1.2).min(usable_width * 0.4);
            let width_px = if content_len == 0 {
                0.0
            } else {
                ((content_len as f32) * 0.7).clamp(4.0, usable_width - indent_px)
            };

            let marker = if err_set.contains(&line_idx) {
                MinimapMarkerKind::Error
            } else if warn_set.contains(&line_idx) {
                MinimapMarkerKind::Warning
            } else if match_set.contains(&line_idx) {
                MinimapMarkerKind::SearchMatch
            } else {
                MinimapMarkerKind::None
            };

            result.push(MinimapLine {
                line_idx,
                indent_px,
                width_px,
                marker,
                is_cursor: line_idx == cursor_row,
            });
        }

        result
    }

    /// Computes the (top_offset_px, height_px) of the visible viewport lens slider.
    pub fn compute_viewport_lens(
        total_lines: usize,
        scroll_row: usize,
        visible_count: usize,
        track_height_px: f32,
    ) -> (f32, f32) {
        if total_lines == 0 || track_height_px <= 0.0 {
            return (0.0, track_height_px);
        }

        let ratio = (visible_count as f32 / total_lines as f32).clamp(0.05, 1.0);
        let lens_h = (track_height_px * ratio).clamp(24.0, track_height_px);

        let max_scroll = total_lines.saturating_sub(visible_count);
        let scroll_ratio = if max_scroll > 0 {
            (scroll_row as f32 / max_scroll as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let lens_top = scroll_ratio * (track_height_px - lens_h);
        (lens_top, lens_h)
    }

    /// Converts a Y coordinate on the minimap track into a document line index.
    pub fn line_from_y(y_px: f32, track_height_px: f32, total_lines: usize) -> usize {
        if total_lines <= 1 || track_height_px <= 0.0 {
            return 0;
        }
        let ratio = (y_px / track_height_px).clamp(0.0, 1.0);
        ((ratio * total_lines as f32) as usize).min(total_lines - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimap_toggle() {
        let mut model = MinimapModel::new();
        assert!(model.enabled);
        assert!(!model.toggle());
        assert!(model.toggle());
    }

    #[test]
    fn test_minimap_compute_lines() {
        let model = MinimapModel::new();
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 42;".to_string(),
            "".to_string(),
            "}".to_string(),
        ];
        let error_lines = vec![1];
        let warning_lines = vec![];
        let search_lines = vec![0];

        let result =
            model.compute_lines(&lines, 1, &error_lines, &warning_lines, &search_lines, 50);
        assert_eq!(result.len(), 4);

        // Line 0 has search match
        assert_eq!(result[0].marker, MinimapMarkerKind::SearchMatch);
        assert_eq!(result[0].indent_px, 0.0);
        assert!(result[0].width_px > 0.0);

        // Line 1 has indent and error marker, is cursor line
        assert!(result[1].indent_px > 0.0);
        assert_eq!(result[1].marker, MinimapMarkerKind::Error);
        assert!(result[1].is_cursor);

        // Line 2 is blank
        assert_eq!(result[2].width_px, 0.0);
    }

    #[test]
    fn test_viewport_lens_calculation() {
        let (top, h) = MinimapModel::compute_viewport_lens(100, 0, 20, 500.0);
        assert_eq!(top, 0.0);
        assert!(h > 24.0);

        let (top_mid, _) = MinimapModel::compute_viewport_lens(100, 40, 20, 500.0);
        assert!(top_mid > 0.0);

        let (top_end, h_end) = MinimapModel::compute_viewport_lens(100, 80, 20, 500.0);
        assert_eq!(top_end + h_end, 500.0);
    }

    #[test]
    fn test_line_from_y() {
        assert_eq!(MinimapModel::line_from_y(0.0, 500.0, 100), 0);
        assert_eq!(MinimapModel::line_from_y(250.0, 500.0, 100), 50);
        assert_eq!(MinimapModel::line_from_y(500.0, 500.0, 100), 99);
    }
}
