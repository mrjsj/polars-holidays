#![allow(clippy::unused_unit)]
use crate::countries::HOLIDAYS;
use arity::binary_elementwise_values;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

pub struct HolidayChecker {
    holidays: &'static phf::Map<&'static str, &'static phf::Map<i32, &'static str>>,
}

impl HolidayChecker {
    pub fn new() -> Self {
        let holidays = &HOLIDAYS;

        Self { holidays }
    }

    pub fn is_holiday(&self, date: &i32, country_code: &str) -> Result<bool, PolarsError> {
        // self.holidays.iter()
        match self.holidays.get(country_code) {
            Some(holidays) => Ok(holidays.contains_key(date)),
            _ => polars_bail!(InvalidOperation: "Country code `{country_code}` not found."),
        }
    }

    pub fn get_holiday(&self, date: &i32, country_code: &str) -> Result<Option<&str>, PolarsError> {
        match self.holidays.get(country_code) {
            Some(holidays) => match holidays.get(date) {
                Some(holiday) => Ok(Some(*holiday)),
                _ => Ok(None),
            },
            _ => polars_bail!(InvalidOperation: "Country code `{country_code}` not found."),
        }
    }
}

#[polars_expr(output_type=Boolean)]
fn is_holiday(inputs: &[Series]) -> PolarsResult<Series> {
    let s1 = &inputs[0];
    let s2 = &inputs[1];
    let ca1 = s1.date()?;
    let ca2 = s2.str()?;

    match ca2.len() {
        1 => {
            let holiday_checker: HolidayChecker = HolidayChecker::new();
            let holidays = holiday_checker.holidays.get(ca2.first().unwrap()).unwrap();
            let out: BooleanChunked = ca1
                .iter()
                .map(|opt_v| opt_v.map(|v| holidays.contains_key(&v)))
                .collect();
            Ok(out.into_series())
        },
        _ => {
            let holiday_checker: HolidayChecker = HolidayChecker::new();
            let out: BooleanChunked = binary_elementwise_values(ca1, ca2, |lhs: i32, rhs: &str| {
                holiday_checker.is_holiday(&lhs, rhs).unwrap()
            });
            Ok(out.into_series())
        },
    }
}

#[polars_expr(output_type=String)]
fn get_holiday(inputs: &[Series]) -> PolarsResult<Series> {
    let s1 = &inputs[0];
    let s2 = &inputs[1];
    let ca1 = s1.date()?;
    let ca2 = s2.str()?;

    match ca2.len() {
        1 => {
            let holiday_checker: HolidayChecker = HolidayChecker::new();
            let holidays = holiday_checker.holidays.get(ca2.first().unwrap()).unwrap();
            let out: BooleanChunked = ca1
                .iter()
                .map(|opt_v| opt_v.map(|v| holidays.contains_key(&v)))
                .collect();
            Ok(out.into_series())
        },
        _ => {
            let holiday_checker: HolidayChecker = HolidayChecker::new();
            let out: StringChunked = binary_elementwise_values(ca1, ca2, |lhs: i32, rhs: &str| {
                holiday_checker.get_holiday(&lhs, rhs).unwrap_or(None)
            });
            Ok(out.into_series())
        },
    }
}
