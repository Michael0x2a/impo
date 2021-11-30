use std::fmt;
use string_cache::DefaultAtom as Atom;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IntLiteral{
    pub base: u32,
    pub digits: Atom,
    pub raw_value: usize,
}

impl IntLiteral {
    pub fn new(base: u32, digits: Atom) -> Result<IntLiteral, std::num::ParseIntError> {
        let raw_value = usize::from_str_radix(&digits, base)?;
        Ok(IntLiteral{
            base: base,
            digits: digits,
            raw_value: raw_value,
        })
    }

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