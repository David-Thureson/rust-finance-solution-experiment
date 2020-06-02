use finance_solution::academic::*;

pub fn main() {
    try_single_use_statement();
}

fn try_single_use_statement() {
    let apr = 0.085;
    let epr = convert_apr_to_epr(apr, 12);
    let solution = fv(epr, 60, 10_000);
    assert_rounded_4!(15_273.0060, solution.future_value());
}
