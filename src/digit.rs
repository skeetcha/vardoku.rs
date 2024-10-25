use std::num::NonZeroU8;

pub trait DigitSize {
    fn max_digit() -> u8;
    fn create() -> Self;
}

pub struct Four;
pub struct Nine;
pub struct Sixteen;
pub struct TwentyFive;

impl DigitSize for Four {
    fn max_digit() -> u8 {
        4
    }

    fn create() -> Self {
        Self {}
    }
}

impl DigitSize for Nine {
    fn max_digit() -> u8 {
        9
    }

    fn create() -> Self {
        Self {}
    }
}

impl DigitSize for Sixteen {
    fn max_digit() -> u8 {
        16
    }

    fn create() -> Self {
        Self {}
    }
}

impl DigitSize for TwentyFive {
    fn max_digit() -> u8 {
        25
    }

    fn create() -> Self {
        Self {}
    }
}

pub struct Digit<T: DigitSize>(NonZeroU8, T);

impl<T: DigitSize> Digit<T> {
    pub fn new(digit: u8) -> Self {
        Self::new_checked(digit).unwrap()
    }

    pub fn new_checked(digit: u8) -> Option<Self> {
        if digit > T::max_digit() {
            return None;
        }

        Some(Self(NonZeroU8::new(digit)?, T::create()))
    }

    pub fn from_index(idx: u8) -> Self {
        Self::new_checked(idx + 1).unwrap()
    }

    pub fn all() -> impl Iterator<Item = Self> {
        match T::max_digit() {
            4 => (1..5).map(Digit::new),
            9 => (1..10).map(Digit::new),
            16 => (1..17).map(Digit::new),
            25 => (1..26).map(Digit::new),
            _ => panic!("invalid")
        }
    }

    pub fn get(self) -> u8 {
        self.0.get()
    }

    pub fn as_index(self) -> usize {
        self.get() as usize - 1
    }
}