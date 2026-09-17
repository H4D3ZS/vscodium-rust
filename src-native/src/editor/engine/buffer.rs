// Pure Rust Piece Table Text Buffer for Monaco Editor Engine.
// Provides ultra-fast arbitrary text insertion and deletion,
// line-index caching, and fast offset-to-position mapping.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferSource {
    Original,
    Add,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub source: BufferSource,
    pub start: usize,
    pub length: usize,
    pub line_feeds: usize,
}

#[derive(Clone, Debug)]
pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
    cached_text: Option<String>,
    line_cache: Option<Vec<String>>,
    line_offsets: Option<Vec<(usize, usize)>>,
}

impl PieceTable {
    pub fn new(content: String) -> Self {
        let line_feeds = content.chars().filter(|&c| c == '\n').count();
        let len = content.len();

        let pieces = if len > 0 {
            vec![Piece {
                source: BufferSource::Original,
                start: 0,
                length: len,
                line_feeds,
            }]
        } else {
            Vec::new()
        };

        let mut pt = Self {
            original: content,
            add: String::new(),
            pieces,
            cached_text: None,
            line_cache: None,
            line_offsets: None,
        };
        pt.rebuild_cache();
        pt
    }

    pub fn empty() -> Self {
        Self::new(String::new())
    }

    pub fn len(&self) -> usize {
        self.pieces.iter().map(|p| p.length).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn line_count(&self) -> usize {
        if let Some(ref lines) = self.line_cache {
            lines.len().max(1)
        } else {
            let lf: usize = self.pieces.iter().map(|p| p.line_feeds).sum();
            (lf + 1).max(1)
        }
    }

    pub fn text(&mut self) -> &str {
        if self.cached_text.is_none() {
            self.rebuild_cache();
        }
        self.cached_text.as_deref().unwrap_or("")
    }

    pub fn lines(&mut self) -> &[String] {
        if self.line_cache.is_none() {
            self.rebuild_cache();
        }
        self.line_cache.as_deref().unwrap_or(&[])
    }

    pub fn line(&mut self, row: usize) -> &str {
        let lines = self.lines();
        lines.get(row).map(|s| s.as_str()).unwrap_or("")
    }

    /// Zero-copy immutable slice access to a line without requiring mutable borrow.
    pub fn line_slice(&self, row: usize) -> &str {
        if let (Some(text), Some(offsets)) = (&self.cached_text, &self.line_offsets) {
            if let Some(&(start, end)) = offsets.get(row) {
                return &text[start..end];
            }
        }
        ""
    }

    pub fn line_len(&mut self, row: usize) -> usize {
        self.line(row).chars().count()
    }

    fn rebuild_cache(&mut self) {
        let mut full = String::with_capacity(self.len());
        for p in &self.pieces {
            let src = match p.source {
                BufferSource::Original => &self.original[p.start..p.start + p.length],
                BufferSource::Add => &self.add[p.start..p.start + p.length],
            };
            full.push_str(src);
        }

        let mut line_offsets = Vec::new();
        let mut start = 0;
        for (idx, byte) in full.as_bytes().iter().enumerate() {
            if *byte == b'\n' {
                line_offsets.push((start, idx));
                start = idx + 1;
            }
        }
        line_offsets.push((start, full.len()));

        let lines: Vec<String> = if full.is_empty() {
            vec![String::new()]
        } else {
            full.split('\n').map(|s| s.to_string()).collect()
        };

        self.cached_text = Some(full);
        self.line_cache = Some(lines);
        self.line_offsets = Some(line_offsets);
    }

    pub fn insert(&mut self, offset: usize, text: &str) {
        if text.is_empty() {
            return;
        }

        let add_start = self.add.len();
        self.add.push_str(text);
        let add_len = text.len();
        let add_lf = text.chars().filter(|&c| c == '\n').count();

        let new_piece = Piece {
            source: BufferSource::Add,
            start: add_start,
            length: add_len,
            line_feeds: add_lf,
        };

        if self.pieces.is_empty() {
            self.pieces.push(new_piece);
            self.rebuild_cache();
            return;
        }

        let mut curr_offset = 0;
        let mut target_piece_idx = self.pieces.len();
        let mut offset_in_piece = 0;

        for (idx, p) in self.pieces.iter().enumerate() {
            if offset <= curr_offset + p.length {
                target_piece_idx = idx;
                offset_in_piece = offset - curr_offset;
                break;
            }
            curr_offset += p.length;
        }

        if target_piece_idx >= self.pieces.len() {
            // Append at the end
            self.pieces.push(new_piece);
        } else if offset_in_piece == 0 {
            // Insert before target piece
            self.pieces.insert(target_piece_idx, new_piece);
        } else if offset_in_piece == self.pieces[target_piece_idx].length {
            // Insert after target piece
            self.pieces.insert(target_piece_idx + 1, new_piece);
        } else {
            // Split target piece into left and right
            let target = self.pieces[target_piece_idx].clone();
            let left_text = self.slice_piece(&target, 0, offset_in_piece);
            let right_text =
                self.slice_piece(&target, offset_in_piece, target.length - offset_in_piece);

            let left_lf = left_text.chars().filter(|&c| c == '\n').count();
            let right_lf = right_text.chars().filter(|&c| c == '\n').count();

            let left_piece = Piece {
                source: target.source,
                start: target.start,
                length: offset_in_piece,
                line_feeds: left_lf,
            };

            let right_piece = Piece {
                source: target.source,
                start: target.start + offset_in_piece,
                length: target.length - offset_in_piece,
                line_feeds: right_lf,
            };

            self.pieces[target_piece_idx] = left_piece;
            self.pieces.insert(target_piece_idx + 1, new_piece);
            self.pieces.insert(target_piece_idx + 2, right_piece);
        }

        self.rebuild_cache();
    }

    pub fn delete(&mut self, offset: usize, length: usize) {
        if length == 0 || self.pieces.is_empty() {
            return;
        }

        let end_offset = offset + length;
        let mut new_pieces = Vec::new();
        let mut curr_offset = 0;

        for p in &self.pieces {
            let piece_start = curr_offset;
            let piece_end = curr_offset + p.length;

            if piece_end <= offset || piece_start >= end_offset {
                // Entirely outside deletion range
                new_pieces.push(p.clone());
            } else {
                // Intersects deletion range
                if piece_start < offset {
                    // Retain left part
                    let left_len = offset - piece_start;
                    let left_text = self.slice_piece(p, 0, left_len);
                    let left_lf = left_text.chars().filter(|&c| c == '\n').count();
                    new_pieces.push(Piece {
                        source: p.source,
                        start: p.start,
                        length: left_len,
                        line_feeds: left_lf,
                    });
                }

                if piece_end > end_offset {
                    // Retain right part
                    let skip = end_offset - piece_start;
                    let right_len = piece_end - end_offset;
                    let right_text = self.slice_piece(p, skip, right_len);
                    let right_lf = right_text.chars().filter(|&c| c == '\n').count();
                    new_pieces.push(Piece {
                        source: p.source,
                        start: p.start + skip,
                        length: right_len,
                        line_feeds: right_lf,
                    });
                }
            }

            curr_offset += p.length;
        }

        self.pieces = new_pieces;
        self.rebuild_cache();
    }

    fn slice_piece<'a>(&'a self, piece: &Piece, offset: usize, len: usize) -> &'a str {
        let base = match piece.source {
            BufferSource::Original => &self.original,
            BufferSource::Add => &self.add,
        };
        let s = piece.start + offset;
        &base[s..s + len]
    }

    pub fn position_to_offset(&mut self, row: usize, col: usize) -> usize {
        let lines = self.lines();
        let mut offset = 0;
        for (idx, line) in lines.iter().enumerate() {
            if idx == row {
                let char_indices: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
                let col_byte = if col < char_indices.len() {
                    char_indices[col]
                } else {
                    line.len()
                };
                return offset + col_byte;
            }
            offset += line.len() + 1; // +1 for '\n'
        }
        offset
    }

    pub fn offset_to_position(&mut self, target_offset: usize) -> (usize, usize) {
        let lines = self.lines();
        let mut curr = 0;
        for (row, line) in lines.iter().enumerate() {
            let next = curr + line.len() + 1;
            if target_offset <= curr + line.len() {
                let byte_in_line = target_offset.saturating_sub(curr);
                let col = line[..byte_in_line.min(line.len())].chars().count();
                return (row, col);
            }
            curr = next;
        }
        let last_row = lines.len().saturating_sub(1);
        let last_col = lines.get(last_row).map(|l| l.chars().count()).unwrap_or(0);
        (last_row, last_col)
    }

    pub fn insert_at(&mut self, row: usize, col: usize, text: &str) {
        let offset = self.position_to_offset(row, col);
        self.insert(offset, text);
    }

    pub fn delete_range(
        &mut self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) {
        let start_offset = self.position_to_offset(start_row, start_col);
        let end_offset = self.position_to_offset(end_row, end_col);
        if end_offset > start_offset {
            self.delete(start_offset, end_offset - start_offset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piecetable_basic_insert_delete() {
        let mut pt = PieceTable::new("Hello World".into());
        assert_eq!(pt.text(), "Hello World");
        assert_eq!(pt.line_count(), 1);

        // Insert in middle
        pt.insert(5, " Beautiful");
        assert_eq!(pt.text(), "Hello Beautiful World");

        // Delete " Beautiful"
        pt.delete(5, 10);
        assert_eq!(pt.text(), "Hello World");
    }

    #[test]
    fn test_piecetable_multiline() {
        let mut pt = PieceTable::new("line 1\nline 2\nline 3".into());
        assert_eq!(pt.line_count(), 3);
        assert_eq!(pt.line(0), "line 1");
        assert_eq!(pt.line(1), "line 2");
        assert_eq!(pt.line(2), "line 3");

        // Position to offset
        let off = pt.position_to_offset(1, 2);
        assert_eq!(off, 7 + 2); // "line 1\n" is 7 bytes, + 2 = 9

        let (r, c) = pt.offset_to_position(off);
        assert_eq!(r, 1);
        assert_eq!(c, 2);
    }

    #[test]
    fn test_piecetable_line_slice() {
        let pt = PieceTable::new("first line\nsecond line\nthird line".into());
        assert_eq!(pt.line_slice(0), "first line");
        assert_eq!(pt.line_slice(1), "second line");
        assert_eq!(pt.line_slice(2), "third line");
        assert_eq!(pt.line_slice(3), "");
    }
}
