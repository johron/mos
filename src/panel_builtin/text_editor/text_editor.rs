use std::path::PathBuf;
use ropey::Rope;

pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub goal_column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize, goal_column: usize) -> Self {
        Self {
            line,
            column,
            goal_column,
        }
    }
}

pub struct TextEditorData {
    pub rope: Rope,
    pub cursors: Vec<Cursor>,
    pub file_path: Option<PathBuf>,
}


pub struct TextEditorController;