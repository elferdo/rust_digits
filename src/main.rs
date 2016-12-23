extern crate num;

use num::{Integer, Unsigned};
use num::cast::ToPrimitive;
use num::FromPrimitive;

struct Digits<T: Integer + Unsigned + ToPrimitive>{
    n: T,
    len: usize
}

impl<T> Digits<T>
    where T: Copy + Integer + Unsigned + ToPrimitive + FromPrimitive
{
    fn new(n: T) -> Digits<T> {
        Digits{n: n, len: Self::digit_length(n)}
    }

    fn digit_length(mut n: T) -> usize {
        let mut digits: usize = 0;

        while n >= FromPrimitive::from_u8(10).unwrap() {
            n = n / FromPrimitive::from_u8(10).unwrap();

            digits += 1;
        }

        digits
    }

    fn shift_digit(n: T) -> T {
        n / FromPrimitive::from_u8(10).unwrap()
    }

    fn next_digit(n: T) -> T {
        n % FromPrimitive::from_u8(10).unwrap()
    }
}

impl<T> Iterator for Digits<T>
    where T: Copy + Integer + Unsigned + ToPrimitive + FromPrimitive
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {return None};

        let result = Self::next_digit(self.n);

        self.n = Self::shift_digit(self.n);

        self.len -= 1;

        Some(result)
    }
}

trait AsDigits<T>
    where T: Copy + Integer + Unsigned + ToPrimitive + FromPrimitive
{
    fn digits(self) -> Digits<T>;
}

impl<T> AsDigits<T> for T
    where T: Copy + Integer + Unsigned + ToPrimitive + FromPrimitive
{
    fn digits(self) -> Digits<T> {
        Digits::new(self)
    }
}

fn main() {
    let n: u32 = 123456;

    for i in n.digits() {
        println!("{:?}", i);
    }
}
