#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::export::num::Signed;
use pyo3_polars::export::polars_core::utils::CustomIterTools;
use std::fmt::Write;
use std::collections::HashMap;
use serde::Deserialize;


pub struct HolidayChecker{
    holidays: HashMap<String, String>,
}

impl HolidayChecker {
    pub fn new(country_code: &str) -> Result<Self, PolarsError> {
        let holidays = match country_code.to_lowercase().as_str() {
            "uk" => HashMap::from([
                ("2025-01-01".to_string(), "New Year's Day".to_string()),
                ("2025-12-25".to_string(), "Christmas Day".to_string()),
                ("2025-04-10".to_string(), "Good Friday".to_string()),                                
            ]),
            country_code => {
                polars_bail!(
                    InvalidOperation: format!(
                        "country_code `{}` not supported", country_code
                    )
                )
            }
        };

        Ok(Self {holidays })
    }

    pub fn is_holiday(&self, date: &str) -> bool {
        // self.holidays.iter()
        //     .any(|(holiday_date, _)| holiday_date == date)
        self.holidays.contains_key(date)
    }

    pub fn get_holiday(&self, date: &str) -> Option<&str> {
        self.holidays.get(date).map(|s| s.as_str())
    }
}

#[derive(Deserialize)]
struct AddCountryCodeKwargs {
    country_code: String,
}

#[polars_expr(output_type=Boolean)]
fn is_holiday(inputs: &[Series], kwargs: AddCountryCodeKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.date()?;

    let str_ca = ca.strftime("%Y-%m-%d")?;
    let hc = HolidayChecker::new(&kwargs.country_code)?;
    
    // Create a new boolean chunked array by mapping over the string dates
    let out: BooleanChunked = str_ca.into_iter()
        .map(|opt_v| opt_v.map(|v| hc.is_holiday(&v)))
        .collect_trusted();

    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn get_holiday(inputs: &[Series], kwargs: AddCountryCodeKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.date()?;

    let str_ca = ca.strftime("%Y-%m-%d")?;
    let hc = HolidayChecker::new(&kwargs.country_code)?;
    
    // Create a new string chunked array by mapping over the string dates
    let out: StringChunked = str_ca.into_iter()
        .map(|opt_v| opt_v.and_then(|v| hc.get_holiday(&v).map(|s| s.to_string())))
        .collect_trusted();

    Ok(out.into_series())
}



fn impl_abs_numeric<T>(ca: &ChunkedArray<T>) -> ChunkedArray<T>
where
    T: PolarsNumericType,
    T::Native: Signed,
{

    ca.apply(|opt_v: Option<T::Native>| opt_v.map(|v: T::Native| v.abs()))
}

#[polars_expr(output_type=Int64)]
fn abs_numeric(inputs: &[Series]) -> PolarsResult<Series> {
    let s = &inputs[0];

    match s.dtype() {
        DataType::Int32 => Ok(impl_abs_numeric(s.i32()?).into_series()),
        DataType::Int64 => Ok(impl_abs_numeric(s.i64()?).into_series()),
        DataType::Float32 => Ok(impl_abs_numeric(s.f32()?).into_series()),
        DataType::Float64 => Ok(impl_abs_numeric(s.f64()?).into_series()),
        // DateType::DateType => Ok(impl_abs_numeric(s.date()?).into_series()),
        dtype => {
            polars_bail!(
                InvalidOperation: format!(
                    "dtype {} not supported for abs_numeric. Expected one of: Int32, Int64, Float32, Float64",
                    dtype
                )
            )
        },
    }
}
