use ropey::Rope;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Cursor {
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl Cursor {
    pub(crate) fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub(crate) struct Editor {
    pub(crate) rope: Rope,
    pub(crate) cursors: Vec<Cursor>,
    pub(crate) file_path: Option<String>,
    pub(crate) top_line: usize,
    pub(crate) height: usize,
    show_gutter: bool,
}

pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,

    WordRight,
    WordLeft,
}

impl Editor {
    pub(crate) fn new(initial: Option<&str>, file_path: Option<String>) -> Self {
        let initial = initial.unwrap_or("");
        let rope = Rope::from_str(initial);
        let mut cursors = vec![Cursor { line: 0, col: 0 }];
        cursors[0] = Cursor::new(0, 0);
        Self {
            rope,
            cursors,
            file_path,
            show_gutter: true,
            top_line: 0,
            height: 0,
        }
    }

    pub fn open_file(&mut self, file_path: &str) {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            self.rope = Rope::from_str(&content);
            self.file_path = Some(file_path.to_string());
            self.cursors = vec![Cursor { line: 0, col: 0 }];
            self.cursors[0] = Cursor::new(0, 0);
            self.top_line = 0;
        }
    }
    
    pub fn get_file_extension(&self) -> Option<String> {
        if let Some(ref path) = self.file_path {
            if let Some(ext) = std::path::Path::new(path).extension() {
                return Some(ext.to_string_lossy().to_string());
            }
        }
        None
    }
    
    fn line_visible_len(&self, line: usize) -> usize {
        let len = self.rope.line(line).len_chars();
        if len == 0 {
            return 0;
        }
        let start = self.rope.line_to_char(line);
        // safe because len > 0
        let last = self.rope.char(start + len - 1);
        if last == '\n' {
            len - 1
        } else {
            len
        }
    }

    fn cursor_abs_pos(&self, cur: &Cursor) -> usize {
        self.rope.line_to_char(cur.line) + cur.col
    }

    fn char_under_cursor(&self, cur: &Cursor) -> Option<char> {
        let vis_len = self.line_visible_len(cur.line);
        if cur.col < vis_len {
            let pos = self.cursor_abs_pos(cur);
            Some(self.rope.char(pos))
        } else {
            None
        }
    }

    fn clamp_cursor(rope: &Rope, mut c: Cursor) -> Cursor {
        let line_count = rope.len_lines();
        if c.line >= line_count.saturating_sub(1) + 1 {
            c.line = line_count.saturating_sub(1);
        }
        let line_len = {
            // compute visible len using rope methods
            let len = rope.line(c.line).len_chars();
            if len == 0 {
                0
            } else {
                let start = rope.line_to_char(c.line);
                let last = rope.char(start + len - 1);
                if last == '\n' { len - 1 } else { len }
            }
        };
        if c.col > line_len {
            c.col = line_len;
        }

        c
    }

    fn clamp_cursors(&mut self) {
        let mut taken_lines: Vec<usize> = vec![];
        let mut taken_cols: Vec<usize> = vec![];

        for (i, c) in self.cursors.clone().iter().enumerate() {
            for (idx, line) in taken_lines.iter().enumerate() {
                if line == &c.line && let Some(col) = taken_lines.get(idx) {
                    if col == &c.col {
                        self.cursors.remove(i);
                        continue
                    }
                }
            }

            taken_lines.push(c.line);
            taken_cols.push(c.col);
        }
    }

    pub(crate) fn input(&mut self, ch: char) {
        // Insert at each cursor. To avoid offsets messing up, convert to absolute char indices,
        // sort descending and insert in that order.
        let mut positions: Vec<usize> = self
            .cursors
            .iter()
            .map(|cur| {
                let line_start = self.rope.line_to_char(cur.line);
                line_start + cur.col
            })
            .collect();

        // sort unique descending (if two cursors at same pos, insert once per cursor still OK,
        // but we keep them separate so each receives a char).
        let mut pos_with_idx: Vec<(usize, usize)> =
            positions.iter().copied().enumerate().map(|(i, p)| (p, i)).collect();
        pos_with_idx.sort_by(|a, b| b.0.cmp(&a.0)); // descending by position

        for (pos, _idx) in pos_with_idx {
            self.rope.insert_char(pos, ch);
        }

        // After insert, advance all cursors' columns by 1 (for simplicity).
        for cur in &mut self.cursors {
            if ch == '\n' {
                cur.line += 1;
                cur.col = 0;
            } else {
                cur.col += 1;
            }

            Self::clamp_cursor(&self.rope, cur.clone());
        }
        self.update_scroll(0);
    }

    pub fn input_str(&mut self, input: String) {
        for c in input.chars() {
            self.input(c);
        }
    }

    fn update_scroll(&mut self, idx: usize) {
        // ensure first cursor is visible
        if self.cursors[idx].line < self.top_line {
            self.top_line = self.cursors[idx].line;
        } else if self.cursors[idx].line >= self.top_line + self.height {
            self.top_line = self.cursors[idx].line.saturating_sub(self.height).saturating_add(1);
        }
    }

    pub fn backspace(&mut self) {
        // Delete character before each cursor. We must compute absolute positions and process descending.
        let mut positions: Vec<usize> = self
            .cursors
            .iter()
            .map(|cur| {
                let line_start = self.rope.line_to_char(cur.line);
                line_start + cur.col
            })
            .collect();

        // For each position, if > 0 remove char at pos-1.
        positions.sort_unstable();
        positions.dedup(); // avoid duplicate deletions at same byte pos
        positions.reverse(); // delete descending
        for pos in positions {
            if pos > 0 {
                self.rope.remove(pos - 1..pos);
            }
        }

        let mut idx = 0;
        while idx < self.cursors.len() {
            // read current cursor state with a short immutable borrow
            let (col, line) = {
                let c = &self.cursors[idx];
                (c.col, c.line)
            };

            if col > 0 {
                let cur = &mut self.cursors[idx];
                cur.col -= 1;
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            } else if line > 0 {
                // compute visible length of previous line using only `self.rope`
                let prev_line = line - 1;
                let new_col = {
                    let len = self.rope.line(prev_line).len_chars();
                    if len == 0 {
                        0
                    } else {
                        let start = self.rope.line_to_char(prev_line);
                        let last = self.rope.char(start + len - 1);
                        if last == '\n' { len - 1 } else { len }
                    }
                };
                let cur = &mut self.cursors[idx];
                cur.line = prev_line;
                cur.col = new_col;
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            } else {
                let cur = &mut self.cursors[idx];
                *cur = Self::clamp_cursor(&self.rope, cur.clone());
            }

            self.update_scroll(idx);

            idx += 1;
        }
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        //let mut taken: HashMap<usize, usize> = HashMap::new();
        for (idx, _) in self.cursors.clone().iter().enumerate() {
            //if taken.contains_key(&self.cursors[idx].line) && taken.get(&self.cursors[idx].col).is_some() {
            //    self.input_str(String::from("removed"));
            //    self.cursors.remove(idx);
            //    continue
            //}
//
            //taken.insert(self.cursors[idx].line, self.cursors[idx].col);

            match direction {
                CursorDirection::Left => {
                    if self.cursors[idx].col > 0 {
                        self.cursors[idx].col -= 1;
                    } else if self.cursors[idx].line > 0 {
                        self.cursors[idx].line -= 1;
                        self.cursors[idx].col = self.line_visible_len(self.cursors[idx].line);
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
                }
                CursorDirection::Right => {
                    let line_len = self.line_visible_len(self.cursors[idx].line);
                    if self.cursors[idx].col < line_len {
                        self.cursors[idx].col += 1;
                    } else if self.cursors[idx].line + 1 < self.rope.len_lines() {
                        self.cursors[idx].line += 1;
                        self.cursors[idx].col = 0;
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
                }
                CursorDirection::Up => {
                    if self.cursors[idx].line > 0 {
                        self.cursors[idx].line -= 1;
                        let line_len = self.line_visible_len(self.cursors[idx].line);
                        self.cursors[idx].col = min(self.cursors[idx].col, line_len);
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());

                    if self.cursors[idx].line < self.top_line {
                        self.top_line = self.cursors[idx].line;
                    }

                    self.update_scroll(idx);
                }
                CursorDirection::Down => {
                    if self.cursors[idx].line + 1 < self.rope.len_lines() {
                        self.cursors[idx].line += 1;
                        let line_len = self.line_visible_len(self.cursors[idx].line);
                        self.cursors[idx].col = min(self.cursors[idx].col, line_len);
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());

                    if self.cursors[idx].line >= self.top_line + self.height {
                        self.top_line = self.cursors[idx].line.saturating_sub(self.height).saturating_add(1);
                    }

                    self.update_scroll(idx);
                }
                CursorDirection::WordLeft => { // TODO: should not skip over multiple newlines at once, only one at a time, applies to WordRight too
                    let idx = 0;
                    let mut pos = self.cursor_abs_pos(&self.cursors[idx]);
                    if pos == 0 {
                        // already at start
                    } else {
                        // step left at least one char
                        pos -= 1;
                        // skip whitespace going backward
                        while pos > 0 && self.rope.char(pos).is_whitespace() {
                            pos -= 1;
                        }
                        // move to start of that word
                        while pos > 0 && !self.rope.char(pos - 1).is_whitespace() {
                            pos -= 1;
                        }
                        let line = self.rope.char_to_line(pos);
                        let col = pos - self.rope.line_to_char(line);
                        self.cursors[idx].line = line;
                        self.cursors[idx].col = col;
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
                }
                CursorDirection::WordRight => {
                    let idx = 0;
                    let total = self.rope.len_chars();
                    let mut pos = self.cursor_abs_pos(&self.cursors[idx]);
                    if pos < total {
                        if self.rope.char(pos).is_whitespace() {
                            while pos < total && self.rope.char(pos).is_whitespace() {
                                pos += 1;
                            }
                        } else {
                            while pos < total && !self.rope.char(pos).is_whitespace() {
                                pos += 1;
                            }
                            while pos < total && self.rope.char(pos).is_whitespace() {
                                pos += 1;
                            }
                        }
                        let line = self.rope.char_to_line(pos);
                        let col = pos - self.rope.line_to_char(line);
                        self.cursors[idx].line = line;
                        self.cursors[idx].col = col;
                    }
                    self.cursors[idx] = Self::clamp_cursor(&self.rope, self.cursors[idx].clone());
                }
            }

            self.clamp_cursors();

            //taken.clear();
            //if taken.contains_key(&self.cursors[idx].line) && taken.get(&self.cursors[idx].col).is_some() {
            //    self.input_str(String::from("removed"));
            //    self.cursors.remove(idx);
            //    continue
            //}
//
            //taken.insert(self.cursors[idx].line, self.cursors[idx].col);
        }
    }

    pub fn scroll_up(&mut self) {
        if self.top_line > 0 {
            self.top_line -= 1;

            if self.cursors[0].line > self.height + self.top_line - 1 {
                self.cursors[0].line = self.height + self.top_line - 1;
                self.cursors[0] = Self::clamp_cursor(&self.rope, self.cursors[0].clone());
            }
        }
    }

    pub fn scroll_down(&mut self) {
        if self.top_line + 1 < self.rope.len_lines() {
            self.top_line += 1;

            if self.cursors[0].line < self.top_line {
                self.cursors[0].line = self.top_line;
                self.cursors[0] = Self::clamp_cursor(&self.rope, self.cursors[0].clone());
            }
        }
    }

    pub fn tab(&mut self) {
        for _ in 0..4 {
            self.input(' ');
        }
    }

    fn add_cursor_at(&mut self, line: usize, col: usize) {
        let mut cur = Cursor { line, col };
        cur = Self::clamp_cursor(&self.rope, cur);
        // avoid same cursor twice
        if !self.cursors.contains(&cur) {
            self.cursors.push(cur);
        }
    }

    pub fn add_cursor(&mut self, line: usize, col: usize) { // TODO: Multi-cursor support is terrible currently, need to fix that since i just wanted a working editor fast and i have not implemented good enough.
        self.cursors.push(Cursor::new(line, col));
    }

    fn toggle_gutter(&mut self) {
        self.show_gutter = !self.show_gutter;
    }
}