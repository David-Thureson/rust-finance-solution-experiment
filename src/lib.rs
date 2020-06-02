extern crate finance_solution;

pub mod academic;
pub mod core;
pub mod npv;
pub mod problems;
pub mod simple;

pub fn initialized_vector<L, V>(length: L, value: V) -> Vec<V>
    where
        L: Into<usize>,
        V: Copy,
{
    let mut v = vec![];
    for _ in 0..length.into() {
        v.push(value);
    }
    v
}
