use std::fmt;
use string_cache::DefaultAtom as Atom;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IntLiteral{
    pub base: usize,
    pub digits: Atom,
}

impl IntLiteral {
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.digits.chars().count() + if self.base == 10 { 0 } else { 2 }
    }
}

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.base {
                10 => "",
                2 => "0b",
                8 => "0o",
                16 => "0x",
                _ => "0?",
            },
            self.digits,
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FloatLiteral{
    pub integral_digits: Atom,
    pub fractional_digits: Atom,
    pub power: Atom,
}

impl FloatLiteral{
    #[must_use]
    pub fn char_len(&self) -> usize {
        let mut count = self.integral_digits.chars().count() + 1 + self.fractional_digits.chars().count();
        if !self.power.is_empty() {
            count += 1 + self.power.chars().count();
        }
        count
    }
}

impl fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.power.is_empty() {
            write!(f, "{}.{}", self.integral_digits, self.fractional_digits)
        } else {
            write!(f, "{}.{}e{}", self.integral_digits, self.fractional_digits, self.power)
        }
    }
}