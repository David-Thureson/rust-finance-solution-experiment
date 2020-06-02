#![allow(dead_code)]

use crate::*;
use finance_solution::core::*;
use std::mem;

pub fn main() {
    try_textbook_irr();
}

fn try_textbook_irr() {
    let cashflows: [f64; 5] = [-87.0, 25.0, -40.0, 60.0, 60.0];
    let rate_expected = 0.056;
    let rates = initialized_vector(cashflows.len() - 1, rate_expected);
    let npv_crate = net_present_value_schedule(&rates, &cashflows);
    dbg!(npv_crate);
    let npv_book = npv_book(rate_expected, &cashflows);
    dbg!(npv_book);
    let irr_calc = irr(&cashflows, 0.0, 1.0);
    dbg!(irr_calc);
}

fn npv_book(rate: f64, cashflows: &[f64]) -> f64 {
    let mut npv = 0.0;
    for (period, c) in cashflows.iter().enumerate() {
        let one_period = c / (1.0 + rate).powi(period as i32);
        //dbg!(period, c, one_period);
        npv += one_period;
    }
    npv
}

fn npv_book_vec(rate: f64, cashflows: Vec<f64>) -> f64 {
    let mut npv = 0.0;
    for (period, c) in cashflows.iter().enumerate() {
        let one_period = c / (1.0 + rate).powi(period as i32);
        //dbg!(period, c, one_period);
        npv += one_period;
    }
    npv
}

fn irr(cashflows: &[f64], rate_min: f64, rate_max: f64) -> f64 {
    // let cashflows = cashflows.to_vec();
    // let solver_function = |a| npv_book(a, cashflows);
    // let irr = solve(Box::new(solver_function), 0.0, rate_min, rate_max);
    // let irr = solve_irr(cashflows, 0.0, rate_min, rate_max);
    let cashflows = cashflows.to_vec();
    let irr = solve(Box::new(move |a| npv_book(a, &cashflows)), 0.0, rate_min, rate_max);
    irr
}

fn solve_irr(cashflows: &[f64], y_expected: f64, mut x_min: f64, mut x_max: f64) -> f64 {
    assert!(x_min < x_max);

    let y_min = npv_book(x_min, cashflows);
    let y_max = npv_book(x_max, cashflows);
    assert!(y_min > y_max);

    let x = if is_approx_equal!(y_expected, y_min) {
        x_min
    } else if is_approx_equal!(y_expected, y_max) {
        x_max
    } else {
        assert!(y_expected < y_min);
        assert!(y_expected > y_max);
        let mut x = (x_min + x_max) / 2.0;
        let mut y = npv_book(x, cashflows);
        let mut step_count = 1;
        //while !is_approx_equal!(y_expected, y) {
        while y_expected != round_8(y) {
            dbg!(step_count, x, y);
            if y < y_expected {
                x_max = x;
            } else {
                x_min = x;
            }
            x = (x_min + x_max) / 2.0;
            y = npv_book(x, cashflows);
            step_count += 1;
        }
        x
    };
    x
}

/*
fn solve_irr_ratio(cashflows: &[f64], y_expected: f64, mut x_min: f64, mut x_max: f64) -> f64 {
    assert!(x_min < x_max);

    let y_min = npv_book(x_min, cashflows);
    let y_max = npv_book(x_max, cashflows);
    assert!(y_min > y_max);

    let x = if is_approx_equal!(y_expected, y_min) {
        x_min
    } else if is_approx_equal!(y_expected, y_max) {
        x_max
    } else {
        assert!(y_expected < y_min);
        assert!(y_expected > y_max);
        let pct = y_max / (y_max - y_min);
        let mut x = x_min + ((x_max - x_min) * pct);
        let mut y = npv_book(x, cashflows);
        let mut y_prev
        let mut step_count = 1;
        //while !is_approx_equal!(y_expected, y) {
        while y_expected != round_8(y) {
            dbg!(step_count, x, y);
            if y < y_expected {
                x_max = x;
            } else {
                x_min = x;
            }
            x = (x_min + x_max) / 2.0;
            y = npv_book(x, cashflows);
            step_count += 1;
        }
        x
    };
    x
}
*/

/*
fn solve(f: Box<dyn Fn(f64) -> f64>, y_expected: f64, x_min: f64, x_max: f64) -> f64 {
    assert!(x_min < x_max);
    let y_min = f(x_min);
    let y_max = (&f)(x_max);
    let x = if y_min < y_max {
        // y increases as x increases.
        solve_increasing(f, y_expected, x_min, x_max)
    } else {
        // y decreses as x increases.
        solve_decreasing(f, y_expected, x_min, x_max)
    };
    x
}
*/

fn solve_increasing(f: Box<dyn Fn(f64) -> f64>, y_expected: f64, mut x_min: f64, mut x_max: f64) -> f64 {
    assert!(x_min < x_max);

    let y_min = f(x_min);
    let y_max = f(x_max);
    assert!(y_min < y_max);

    let x = if is_approx_equal!(y_expected, y_min) {
        x_min
    } else if is_approx_equal!(y_expected, y_max) {
        x_max
    } else {
        assert!(y_expected > y_min);
        assert!(y_expected < y_max);
        let mut x = (x_min + x_max) / 2.0;
        let mut y = f(x);
        while !is_approx_equal!(y_expected, y) {
            if y < y_expected {
                x_max = x;
            } else {
                x_min = x;
            }
            x = (x_min + x_max) / 2.0;
            y = f(x);
        }
        x
    };
    x
}

fn solve_decreasing(f: Box<dyn Fn(f64) -> f64>, y_expected: f64, mut x_min: f64, mut x_max: f64) -> f64 {
    assert!(x_min < x_max);

    let y_min = f(x_min);
    let y_max = f(x_max);
    assert!(y_min > y_max);

    let x = if is_approx_equal!(y_expected, y_min) {
        x_min
    } else if is_approx_equal!(y_expected, y_max) {
        x_max
    } else {
        assert!(y_expected < y_min);
        assert!(y_expected > y_max);
        let mut x = (x_min + x_max) / 2.0;
        let mut y = f(x);
        while !is_approx_equal!(y_expected, y) {
            if y > y_expected {
                x_max = x;
            } else {
                x_min = x;
            }
            x = (x_min + x_max) / 2.0;
            y = f(x);
        }
        x
    };
    x
}

fn lib_func(_r: f64, _c: &[f64]) -> f64 {
    0.0
}

fn trial_lib_func(c: &'static [f64]) -> f64 {
    // let r = general_solve(&Box::new(|r| lib_func(r, c)), 0.0, 0.0, 1.0);
    let c = c.to_vec();
    let f= Box::new(move |r| lib_func(r, &c));
    let r = general_solve(f, 0.0, 0.0, 1.0);
    r
}

fn general_solve(_f: Box<dyn Fn(f64) -> f64>, _y_expected: f64, _x_min: f64, _x_max: f64) -> f64 {
    0.0
}

fn try_trial_lib_func() {
    let c= &[0.03, 0.04];
    let _r = trial_lib_func(c);
}

fn solve(f: Box<dyn Fn(f64) -> f64>, y_expected: f64, mut x_min: f64, mut x_max: f64) -> f64 {
    println!("x_min: {}, x_max: {}", x_min, x_max);
    if x_min > x_max {
        mem::swap(&mut x_min, &mut x_max);
    }
    assert!(x_min < x_max);

    let y_from_min_x = f(x_min);
    let y_from_max_x = f(x_max);
    let increasing = y_from_min_x < y_from_max_x;
    println!("x_min: {}, x_max: {}, increasing: {}", x_min, x_max, increasing);

    if increasing {
        if y_expected < y_from_min_x || y_expected > y_from_max_x {
            // The answer is not within the given range of x_min and x_max.
            return std::f64::NAN;
        }
    } else {
        if y_expected < y_from_max_x || y_expected > y_from_min_x {
            // The answer is not within the given range of x_min and x_max.
            return std::f64::NAN;
        }
    }
    let mut x = (x_min + x_max) / 2.0;
    let mut y = f(x);
    let mut step_count = 1;
    while y_expected != round_8(y) {
        println!("step_count: {}, x_range: {}, x_min: {}, x: {}, x_max: {}, y_expected: {}, y: {}", step_count, (x_min - x_max).abs(), x_min, x, x_max, y_expected, y);
        if (increasing && y > y_expected) || (!increasing && y < y_expected) {
            x_max = x;
        } else {
            x_min = x;
        }
        x = (x_min + x_max) / 2.0;
        y = f(x);
        step_count += 1;
    }
    x
}



