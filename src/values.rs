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