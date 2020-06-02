#![allow(dead_code)]
#![allow(unused_imports)]

use finance_solution::*;
use finance_solution::academic::loan::*;

pub fn main() {
    // try_solution_doc_example_1();
    // try_series_doc_example_1();
    // try_series_summarize_by_year_doc_example_1();
    // try_series_filter_doc_example_1();
    // try_solution_ab_doc_example_1();
    // try_full_builder();
    // print_tables_for_docs();

    // let solution = loan_solution(200_000, 0.085, 30 * 12);
    // solution.print_ab_comparison(&solution.with_apr(0.095), false);
}

fn try_solution_doc_example_1() {
    // This is a mortgage loan for $200,000.
    let principal = 200_000;

    // The annual percentage rate is 6%
    let apr = 0.06;

    // The loan will be paid off in 15 years.
    let months = 15 * 12;

    let solution = loan_solution(principal, apr, months);

    // Display the inputs, payment amount, formulas, sum of interest, etc.
    dbg!(&solution);

    // The payment is $1,687.71/month.
    assert_rounded_2!(solution.payment(), 1687.71);

    // The sum of payments is simply the monthly payment times the number of months.
    assert_rounded_2!(solution.sum_of_payments(), solution.payment() * months as f64);
    assert_rounded_2!(solution.sum_of_payments(), 303_788.46);

    // The sum of interest is the portion of the sum of payments that is over and above the original
    // loan amount.
    assert_rounded_2!(solution.sum_of_interest(), solution.sum_of_payments() - principal as f64);
    assert_rounded_2!(solution.sum_of_interest(), 103_788.46);

    // Calculate the period-by-period values including the amount of the payment that goes toward
    // interest and principle as well as the running tally of the remaining amounts.
    let series = solution.series();
    // Note that all of the period-by-period values are shown as of the end of the period after that
    // period's payment has been made.
    dbg!(&series);

    // Print the period-by-period values in a formatted table.
    solution.print_table();
}

fn try_series_doc_example_1() {
    // $20,000 loan at 15% APR for 5 years.
    let solution = finance_solution::academic::loan_solution(20_000, 0.15, 5 * 12);
    dbg!(&solution);

    // Calculate the month-by-month details including the principal and interest paid every month.
    let series = solution.series();
    dbg!(&series);

    // Print the monthly detail numbers as a formatted table.
    series.print_table();

    // As above but print only the last period for every yeor of the loan, with each entry
    // showing a summary of payments, interest, and principal paid down for that year.
    series
        .summarize_by_year()
        .print_table();
}

fn try_series_summarize_by_year_doc_example_1() {
    let principal = 200_000;
    let apr = 0.06;
    let months = 15 * 12;
    let solution = loan_solution(principal, apr, months);

    // Group the month-by-month detail into one entry per year, then print these entries in a
    // formatted table.
    solution
        .series()
        .summarize_by_year()
        .print_table_locale(true, true, &num_format::Locale::en, 0);
}

fn try_series_filter_doc_example_1() {
    let principal = 200_000;
    let series = finance_solution::academic::loan_solution(principal, 0.06, 180)
        .series();

    // Print the first five and last five monthly entries.
    series.filter(|entry| entry.period() <= 5 || entry.period() > 175)
        .print_table();

    // Print the monthly entries starting from the point where 95% of the principal is paid off.
    series.filter(|entry| entry.principal_to_date() >= principal as f64 * 0.95)
        .print_table();
}

fn try_solution_ab_doc_example_1() {
    // $200,000 at 6% APR for 15 years.
    let solution_six_pct = finance_solution::academic::loan_solution(200_000, 0.06, 180);

    // Create a variation on the first loan with a 5% APR.
    let solution_five_pct = solution_six_pct.with_apr(0.05);

    // Compare the two loans but don't show the monthly details.
    let include_period_detail = false;
    solution_six_pct.print_ab_comparison(&solution_five_pct, include_period_detail);
}

/*
fn try_full_builder() {
    dbg!(loan().years(30).apr(0.085).principal())
}
*/

fn print_tables_for_docs() {
    /*
    finance_solution::simple::loan_solution(200_000, 0.06, 180)
        //.print_table();
        // .print_table_custom(true, false);
        .print_table_locale(false, true, &num_format::Locale::en, 0);
        */

    /*
    let solution_six_pct = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    let solution_five_pct = solution_six_pct.with_apr(0.05);
    let _include_period_detail = false;
    // solution_six_pct.print_ab_comparison(&solution_five_pct, include_period_detail);

    let include_period_detail = true;
    let include_running_totals = false;
    let include_remaining_amounts = false;
    // solution_six_pct.print_ab_comparison_custom(&solution_five_pct,
    // include_period_detail, include_running_totals, include_remaining_amounts);
    solution_six_pct.print_ab_comparison_locale(&solution_five_pct,
                                                include_period_detail, include_running_totals, include_remaining_amounts, &num_format::Locale::en, 0);
    */

    /*
    let solution_200 = finance_solution::simple::loan_solution(200_000, 0.06, 180);
    let solution_180 = solution_200.with_principal(180_000);
    solution_200.print_ab_comparison(&solution_180, false);
    */

    let solution_15_years = finance_solution::academic::loan_solution(200_000, 0.06, 15 * 12);
    let solution_20_years = solution_15_years.with_months(20 * 12);
    solution_15_years.print_ab_comparison(&solution_20_years, false);

}

struct LoanBuilder {
}

struct LoanBuilderPrincipal {
    principal: f64,
}

struct LoanBuilderApr {
    apr: f64,
}

struct LoanBuilderMonths {
    months: u32,
}

struct LoanBuilderPrincipalApr {
    principal: f64,
    apr: f64,
}

struct LoanBuilderPrincipalMonths {
    principal: f64,
    months: u32,
}

struct LoanBuilderAprMonths {
    apr: f64,
    months: u32,
}

impl LoanBuilder {

    pub fn principal<T>(&self, principal: T) -> LoanBuilderPrincipal
    where T: Into<f64> + Copy
    {
        LoanBuilderPrincipal {
            principal: principal.into(),
        }
    }

    pub fn apr(&self, apr: f64) -> LoanBuilderApr {
        LoanBuilderApr {
            apr,
        }
    }

    pub fn months(&self, months: u32) -> LoanBuilderMonths {
        LoanBuilderMonths {
            months,
        }
    }

    pub fn years<T>(&self, years: T) -> LoanBuilderMonths
        where T: Into<f64> + Copy
    {
        LoanBuilderMonths {
            months: years_to_months(years),
        }
    }
}

impl LoanBuilderPrincipal {
    pub fn apr(&self, apr: f64) -> LoanBuilderPrincipalApr {
        LoanBuilderPrincipalApr {
            principal: self.principal,
            apr,
        }
    }

    pub fn months(&self, months: u32) -> LoanBuilderPrincipalMonths {
        LoanBuilderPrincipalMonths {
            principal: self.principal,
            months,
        }
    }

    pub fn years<T>(&self, years: T) -> LoanBuilderPrincipalMonths
        where T: Into<f64> + Copy
    {
        LoanBuilderPrincipalMonths {
            principal: self.principal,
            months: years_to_months(years),
        }
    }
}

impl LoanBuilderApr {
    pub fn principal<T>(&self, principal: T) -> LoanBuilderPrincipalApr
    where T: Into<f64> + Copy
    {
        LoanBuilderPrincipalApr {
            principal: principal.into(),
            apr: self.apr,
        }
    }

    pub fn months(&self, months: u32) -> LoanBuilderAprMonths {
        LoanBuilderAprMonths {
            apr: self.apr,
            months,
        }
    }

    pub fn years<T>(&self, years: T) -> LoanBuilderAprMonths
        where T: Into<f64> + Copy
    {
        LoanBuilderAprMonths {
            apr: self.apr,
            months: years_to_months(years),
        }
    }
}

impl LoanBuilderMonths {
    pub fn principal<T>(&self, principal: T) -> LoanBuilderPrincipalMonths
        where T: Into<f64> + Copy
    {
        LoanBuilderPrincipalMonths {
            principal: principal.into(),
            months: self.months,
        }
    }

    pub fn apr(&self, apr: f64) -> LoanBuilderAprMonths {
        LoanBuilderAprMonths {
            apr,
            months: self.months,
        }
    }
}

impl LoanBuilderPrincipalApr {
    pub fn months(&self, months: u32) -> LoanSolution {
        loan_solution(self.principal, self.apr, months)
    }

    pub fn years<T>(&self, years: T) -> LoanSolution
        where T: Into<f64> + Copy
    {
        loan_solution(self.principal, self.apr, years_to_months(years))
    }
}

impl LoanBuilderPrincipalMonths {
    pub fn apr(&self, apr: f64) -> LoanSolution {
        loan_solution(self.principal, apr, self.months)
    }
}

impl LoanBuilderAprMonths {
    pub fn principal<T>(&self, principal: T) -> LoanSolution
        where T: Into<f64> + Copy
    {
        loan_solution(principal, self.apr, self.months)
    }
}

fn loan() -> LoanBuilder {
    LoanBuilder{ }
}

fn years_to_months<T>(years: T) -> u32
    where T: Into<f64> + Copy
{
    let years = years.into();
    (years / 12.0).round() as u32
}
