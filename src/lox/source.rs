#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.line, self.column)
    }
}
