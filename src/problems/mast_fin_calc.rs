// Exercises in Mastering Financial Calculations by Bob Steiner
// https://www.amazon.com/Mastering-Financial-Calculations-step-step-ebook-dp-B00A8EZMJM/dp/B00A8EZMJM/ref=mt_kindle?_encoding=UTF8&me=&qid=

#![allow(dead_code)]
#![allow(unused_imports)]

use finance_solution::convert_rate;
use finance_solution::core;
use finance_solution::assert_approx_equal;
use finance_solution::assert_rounded_2;
use finance_solution::assert_rounded_4;

const DAYS_IN_YEAR: f64 = 365_f64;

pub fn main() {

    // Part 1: THE BASICS

    // 1: Financial Arithmetic Basics
    financial_arithmetic_basics();

    // 2 Statistics Basics

}

fn financial_arithmetic_basics() {
    // Chapter 1

    // SOME OPENINNG REMARKS ON FORMULAS

    // USE OF AN HP CALCULATOR

    // SIMPLE AND COMPOUND INTEREST

    // Simple interest

    println!();
    let total_proceeds = proceeds_of_short_term_investment(1.0, 0.08, 92);
    dbg!(total_proceeds);

    // Compound interest

    // Nominal rates and effective rates

    println!();
    let fv = core::future_value(0.025, 4, -1, false);
    dbg!(fv);
    assert_rounded_4!(1.1038, fv);

    // Example 1.1

    println!();
    let rate_nominal = 0.08;
    let n = 4;
    let rate_effective = effective_rate(rate_nominal, n);
    dbg!(rate_effective);
    assert_rounded_4!(0.0824, rate_effective);

    let rate_effective_crate = convert_rate::convert_apr_to_ear(rate_nominal, n);
    dbg!(rate_effective_crate);
    assert_approx_equal!(rate_effective, rate_effective_crate);

    let rate_nominal_round_trip = nominal_rate(rate_effective, n);
    dbg!(rate_nominal_round_trip);
    assert_approx_equal!(rate_nominal, rate_nominal_round_trip);

    let rate_nominal_crate = convert_rate::convert_ear_to_apr(rate_effective, n);
    dbg!(rate_nominal_crate);
    assert_approx_equal!(rate_nominal, rate_nominal_crate);

    // Example 1.2

    println!();
    let quarterly_equivalent = nominal_rate(0.05, 4);
    dbg!(quarterly_equivalent);
    assert_rounded_4!(0.0491, quarterly_equivalent);

    // Example 1.3

    println!();
    let effective_rate_153_days = effective_rate_days(0.102, 153);
    dbg!(effective_rate_153_days);
    assert_rounded_4!(0.1050, effective_rate_153_days);

    // Continuous compounding

    println!();
    let continuous = effective_to_continuous_rate(0.093);
    dbg!(continuous);
    assert_rounded_4!(0.0889, continuous);

    // Example 1.4

    println!();
    let continuous_for_91_days = effective_to_continuous_rate_days(0.064, 91);
    dbg!(continuous_for_91_days);
    assert_rounded_4!(0.0635, continuous_for_91_days);

    // Example 1.5

    println!();
    let effective = continuous_to_effective_rate(0.072);
    dbg!(effective);
    assert_rounded_4!(0.0747, effective);

    // Reinvestment rates

    // Example 1.6

    println!();
    let total_return = future_value_reinvested_to_end(&[0.035, 0.04, 0.05], 100.0);
    dbg!(total_return);
    assert_rounded_2!(110.96, total_return);

    // NOMINAL AND EFFECTIVE RATES

    // FUTURE VALUE / PRESENT VALUE; TIME VALUE OF MONEY

    // Short-term investments

    println!();

    let i = 0.1;
    let p = 100.0;
    let days = 98;
    let fv_98_days = future_value_days(i, days, p);
    dbg!(fv_98_days);
    assert_rounded_2!(102.68, fv_98_days);

    let pv_98_days = present_value_days(i, days, fv_98_days);
    dbg!(pv_98_days);
    assert_approx_equal!(p, pv_98_days);

    let pv = 100.0;
    let fv = 120.0;
    let days = 90;
    let yld = yield_days(days, pv, fv);
    dbg!(yld);
    let eff_yld = effective_yield_days(days, pv, fv);
    dbg!(eff_yld);

    // Long-term investments

    // Example 1.7

    println!();
    let fv = core::future_value(0.08, 5, -120, false);
    dbg!(fv);
    assert_rounded_2!(176.32, fv);

    // Example 1.8

    println!();
    let fv = future_value_days(0.08, 92, 120.0);
    dbg!(fv);
    assert_rounded_2!(122.42, fv);

    // Example 1.9

    println!();
    let pv = core::present_value(0.07, 4, -270, false);
    dbg!(pv);
    assert_rounded_2!(205.98, pv);

    // Example 1.10

    println!();
    let pv = present_value_days(0.07, 180, 270.0);
    dbg!(pv);
    assert_rounded_2!(260.99, pv);

    // Example 1.11

    println!();
    let yld = yield_days(64, 138.0, 139.58);
    dbg!(yld);
    assert_rounded_4!(0.0653, yld);

}

fn proceeds_of_short_term_investment(p: f64, i: f64, days: u32) -> f64 {
    p * (1.0 + (i * (days as f64 / DAYS_IN_YEAR)))
}

fn effective_rate(i: f64, n: u32) -> f64 {
    let n = n as f64;
    (1.0 + (i / n)).powf(n) - 1.0
}

fn effective_rate_days(i: f64, days: u32) -> f64 {
    let days = days as f64;
    (1.0 + (i * (days / DAYS_IN_YEAR))).powf(DAYS_IN_YEAR / days) - 1.0
}

fn nominal_rate(i: f64, n: u32) -> f64 {
    let n = n as f64;
    ((1.0 + i).powf(1.0 / n) -1.0) * n
}

fn effective_to_continuous_rate(i: f64) -> f64 {
    (1.0 + i).ln()
}

fn effective_to_continuous_rate_days(i: f64, days: u32) -> f64 {
    let days = days as f64;
    (DAYS_IN_YEAR / days) * (1.0 + (i * (days / DAYS_IN_YEAR))).ln()
}

fn continuous_to_effective_rate(i: f64) -> f64 {
    std::f64::consts::E.powf(i) - 1.0
}

fn future_value_reinvested_to_end(rates: &[f64], p: f64) -> f64 {
    let display = false;
    let periods = rates.len();
    if display { println!("\nfuture_value_reinvested_to_end(): p = {}, rates = {:?}", p, rates); }
    let mut cashflows: Vec<Vec<f64>> = vec![];
    let mut fv = std::f64::NAN;
    for period in 0..=periods {
        // These are actions taken at the _beginning_ of the period.
        if display { println!("\n\tperiod = {}", period); }
        let cashflow_sum = if period == 0 {
            0.0
        } else {
            let mut cashflow_sum = 0.0;
            for cashflow in cashflows.iter() {
                let one_cashflow_amount: f64 = cashflow[period - 1];
                if display { println!("\tone_cashflow_amount = {}", one_cashflow_amount); }
                cashflow_sum += one_cashflow_amount;
            }
            cashflow_sum
        };
        if display { println!("\tcashflow_sum = {}", cashflow_sum); }
        if period == periods {
            // Last period.
            fv = cashflow_sum;
        } else {
            // Not the last period.
            let r = rates[period];
            let invest_amount = if period == 0 {
                // Start of irst period.
                p
            } else {
                cashflow_sum
            };
            let cashflow_new = cashflow(r, period, periods - 1, invest_amount);
            if display { println!("\tr = {}, invest_amount = {}, cashflow_new = {:?}", r, invest_amount, cashflow_new); }
            cashflows.push(cashflow_new);
        }
    }
    if display { println!("fv = {}", fv); }
    fv
}

fn cashflow(r: f64, index_start: usize, index_end: usize, p: f64) -> Vec<f64> {
    let mut v = vec![];
    for index in 0..=index_end {
        if index < index_start {
            v.push(0.0);
        } else if index < index_end {
            v.push(p * r);
        } else {
            v.push(p * (1.0 + r));
        }
    }
    v
}

fn future_value_days(i: f64, days: u32, p: f64) -> f64 {
    let days = days as f64;
    p * (1.0 + (i * (days / DAYS_IN_YEAR)))
}

fn present_value_days(i: f64, days: u32, p: f64) -> f64 {
    let days = days as f64;
    p / (1.0 + (i * (days / DAYS_IN_YEAR)))
}

fn yield_days(days: u32, pv: f64, fv: f64) -> f64 {
    let days = days as f64;
    ((fv / pv) - 1.0) * (DAYS_IN_YEAR / days)
}

fn effective_yield_days(days: u32, pv: f64, fv: f64) -> f64 {
    let days = days as f64;
    (fv / pv).powf(DAYS_IN_YEAR / days) - 1.0
}

