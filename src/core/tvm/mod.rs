#![allow(dead_code)]

pub mod future_value;
pub mod periods;
pub mod present_value;
pub mod rate;

pub fn main() {
    // try_future_value_vary_compounding_periods();
    // try_doc_example_time_value_of_money_tvm_solution_1();
    // try_doc_example_future_value_vary_compounding_periods();
    // try_doc_example_present_value_vary_compounding_periods();
    // try_doc_example_tvm_solution_rate_solution();
    try_doc_example_tvm_solution_future_value_solution();
}

/*
fn try_future_value_vary_compounding_periods() {
    let solution = finance::future_value_solution(0.2, 4, 10_000);
    dbg!(&solution);

    let scenarios = solution.future_value_vary_compounding_periods(&[1, 2, 4, 6, 12, 24, 52, 365], true);
    dbg!(&scenarios);
    scenarios.print_table();
}

fn try_doc_example_future_value_vary_compounding_periods() {
    // The interest rate is 5% per quarter.
    let rate = 0.05;

    // The interest will be applied once per quarter for one year.
    let periods = 4;

    // The starting value is $100.00.
    let present_value = 100;

    let solution = finance::future_value_solution(rate, periods, present_value);
    dbg!(&solution);

    // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    let compounding_periods = [1, 4, 12, 52, 365];

    // Add a final scenario with continuous compounding.
    let include_continuous_compounding = true;

    // Compile a list of the future values with each of the above compounding periods as well as
    // continous compounding.
    let scenarios = solution.future_value_vary_compounding_periods(&compounding_periods, include_continuous_compounding);
    // The description in the `setup` field states that the rate is 20% since that's 5% times the
    // number of periods in the original calculation. The final entry has `input: inf` indicating
    // that we used continuous compounding.
    dbg!(&scenarios);

    // Print the results in a formatted table.
    scenarios.print_table();
}

fn try_doc_example_present_value_vary_compounding_periods() {
    // Calculate the future value of an investment that starts at $83.33 and grows 20% in one year.
    // Note that we're going to examine how the present value varies by the number of compounding
    // periods, but we're starting with a future value calculation. It would have been fine to start
    // with a rate, periods, or present value calculation as well. It just depends on what
    // information we have to work with.
    let solution = finance::future_value_solution(0.20, 1, 83.333);
    dbg!(&solution);

    // The present value of $83.33 gives us a future value of about $100.00.
    finance::assert_rounded_2!(100.00, solution.future_value());

    // We'll experiment with compounding annually, quarterly, monthly, weekly, and daily.
    let compounding_periods = [1, 4, 12, 52, 365];

    // Add a final scenario with continuous compounding.
    let include_continuous_compounding = true;

    // Compile a list of the present values needed to arrive at the calculated future value of $100
    // each of the above compounding periods as well a continous compounding.
    let scenarios = solution.present_value_vary_compounding_periods(&compounding_periods, include_continuous_compounding);

    // The final entry has `input: inf` indicating that we used continuous compounding.
    dbg!(&scenarios);

    // Print the results in a formatted table.
    scenarios.print_table();
}

fn try_doc_example_time_value_of_money_tvm_solution_1() {
    // Set up inputs for a variety of calculations that should all be equivalent.
    let rate = 0.015;
    let periods = 24;
    let present_value = 10_000.0;
    let future_value = finance::future_value(rate, periods, present_value);

    // Create a list of solution structs. For simplicity, instead of holding references to a
    // `RateSolution`, a `PeriodsSolution`, and so on turn each result into a more general
    // `TvmSolution`.
    let list = vec![
        finance::rate_solution(periods, present_value, future_value).tvm_solution(),
        finance::periods_solution(rate, present_value, future_value).tvm_solution(),
        finance::present_value_solution(rate, periods, future_value).tvm_solution(),
        finance::future_value_solution(rate, periods, present_value).tvm_solution(),
    ];
    dbg!(&list);

    // Print the formulas used by the four types of calculations.
    for solution in list.iter() {
        println!("{}:\n\t{}\n\t{}", solution.calculated_field(), solution.symbolic_formula(), solution.formula());
    }

    // An alternative would be to create a vector of references to the solutions.
    let _list: Vec<& dyn TimeValueOfMoneySolution> = vec![
        &finance::rate_solution(periods, present_value, future_value),
        &finance::periods_solution(rate, present_value, future_value),
        &finance::present_value_solution(rate, periods, future_value),
        &finance::future_value_solution(rate, periods, present_value),
    ];

}

fn check_formulas() {
    let solution = finance::rate_solution(4, 100, 200);
    dbg!(&solution, &solution.series());
}
*/

fn try_doc_example_tvm_solution_rate_solution() {
    use finance_solution::core::*;

    // First calculate a future value given the other three inputs. $10,000 grows at 10% per year
    // for 12 years using year-by-year compounding.
    let solution_fv = future_value_solution(0.1, 12, -10_000, false);
    dbg!(&solution_fv);
    assert_rounded_2!(31_384.28, solution_fv.future_value());

    // Find out what the rate would have to be to get the same results with continuous compounding
    // instead of compounding once per year.
    let continuous = true;
    let compounding_periods = None; // Don't change the periods.
    let solution_continuous = solution_fv.rate_solution(continuous, compounding_periods);
    dbg!(&solution_continuous);

    // Compare the two calculations.
    solution_fv.print_ab_comparison(&solution_continuous, false);
}

fn try_doc_example_tvm_solution_future_value_solution() {
    use finance_solution::core::*;

    // First calculate a future value given the other three inputs. $10,000 grows at 10% per year
    // for 12 years using year-by-year compounding.
    let years = 12;
    let solution_annual = future_value_solution(0.1, years, -10_000, false);
    dbg!(&solution_annual);
    assert_rounded_2!(31_384.28, solution_annual.future_value());

    // Calculate what the future value would be if we compounded quarterly instead of annualy. We
    // could have started with something other than a future value calculation such as present value
    // or periods, but for this example we'll calculate the future value both times.
    let continuous = false;
    let compounding_periods = Some(years * 4);
    let solution_quarterly = solution_annual.future_value_solution(continuous, compounding_periods);
    dbg!(&solution_quarterly);

    // The rate was automatically changed to 2.5% (10% / 4) because each period is now a quarter
    // instead of a year. Had we used the original rate the future value would be completely
    // different and the calculation would have made no sense.
    assert_rounded_4!(0.0250, solution_quarterly.rate());

    // The future value is slightly higher as we'd expect from increasing the frequency of
    // compounding while holding everything else constant.
    assert_rounded_2!(32_714.90, solution_quarterly.future_value());

    // Compare the two calculations.
    solution_annual.print_ab_comparison(&solution_quarterly, false);
}



