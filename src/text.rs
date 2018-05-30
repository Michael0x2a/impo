#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TextOffset {
    pub offset: usize,
}

impl TextOffset {
    pub fn new(offset: usize) -> TextOffset {
        TextOffset { offset }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TextRange {
    pub offset: TextOffset,
    pub length: usize,
}

impl TextRange {
    pub fn new(offset: TextOffset, length: usize) -> TextRange {
        TextRange { offset, length }
    }

    pub fn new_absolute(start_offset: TextOffset, end_offset: TextOffset) -> TextRange {
        assert!(start_offset.offset <= end_offset.offset);
        TextRange {
            offset: start_offset,
            length: end_offset.offset - start_offset.offset,
        }
    }

    pub fn start(&self) -> TextOffset {
        self.offset
    }

    pub fn end(&self) -> TextOffset {
        TextOffset::new(self.offset.offset + self.length)
    }

    pub fn offset(&self) -> usize {
        self.offset.offset
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

// Courtesy of https://stackoverflow.com/a/40457615/646543
pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings {
            input: input,
        }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self.input.find('\n').map(|i| i + 1).unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}
