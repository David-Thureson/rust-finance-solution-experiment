use std::fmt::{Debug, Formatter, Error, Display};
use std::ops::Deref;

pub fn main() {
    try_rate();
}

fn try_rate() {
    dbg!(Rate::new(0.05));
    dbg!(Rate::new(-0.23456789));
    dbg!(Rate::new(123.456789));
    println!("{}", Rate::new(-0.23456789));
    let rate = Rate::new(-0.23456789);
    dbg!(&rate);
    dbg!(&rate.0);
    dbg!(&(rate as f64));
}

pub struct Rate(f64);

impl Rate {
    pub fn new(val: f64) -> Self {
        Self { 0: val }
    }
}

impl Deref for Rate {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Rate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

impl Into<f64> for Rate {
    fn into(self) -> f64 {
        self.0
    }
}

impl Debug for Rate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.4}%", (self.0 * 100.))
    }
}

