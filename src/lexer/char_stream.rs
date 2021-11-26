use crate::tokens::Position;

pub struct CharStream {
    stream: Vec<char>,
    index: usize,
    position: Position,
}

impl CharStream {
    pub fn new(input: &str) -> CharStream {
        CharStream{ 
            stream: input.chars().collect(),
            index: 0,
            position: Position::start(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        let out = self.stream.get(self.index).copied();
        if let Some(c) = out {
            self.index += 1;
            self.position.advance(c);
        }
        out
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.stream.get(self.index).copied()
    }

    pub fn peek_char_at_offset(&mut self, offset: usize) -> Option<char> {
        self.stream.get(self.index + offset).copied()
    }

    pub fn read_if_char(&mut self, possible: char) -> bool {
        match self.peek_char() {
            Some(c) if c == possible => {
                let _ = self.read_char();
                true
            },
            _ => false,
        }
    }

    pub fn read_if(&mut self, filter: fn(char) -> bool) -> Option<char> {
        match self.peek_char() {
            Some(possible) if filter(possible) => {
                self.read_char()
            },
            _ => None,
        }
    }

    pub fn read_while(&mut self, filter: fn(char) -> bool) -> String {
        let mut out = String::new();
        while let Some(c) = self.read_if(filter) {
            out.push(c);
        }
        out
    }

    pub fn skip_while(&mut self, filter: fn(char) -> bool) -> usize {
        let mut num_skipped = 0;
        while self.read_if(filter).is_some() { 
            num_skipped += 1;
        }
        num_skipped
    }

    pub fn position(&self) -> Position {
        self.position
    }
}