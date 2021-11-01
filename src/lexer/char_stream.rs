use core::str::Chars;
use std::iter::Peekable;
use crate::tokens::Position;

pub struct CharStream<'a> {
    stream: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> CharStream<'a> {
    pub fn new(input: &'a str) -> CharStream<'a> {
        CharStream{ 
            stream: input.chars().peekable(),
            position: Position::start(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        let out = self.stream.next();
        if let Some(c) = out {
            self.position.advance(c);
        }
        out
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.stream.peek().map(char::to_owned)
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
            out.push(c)
        }
        out
    }

    pub fn skip_while(&mut self, filter: fn(char) -> bool) -> usize {
        let mut num_skipped = 0;
        while let Some(_) = self.read_if(filter) { 
            num_skipped += 1;
        }
        num_skipped
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }
}