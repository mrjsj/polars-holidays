import polars as pl
from polars.testing import assert_series_equal
import polars_holidays as plh
import holidays
import random
from pytest import mark
from datetime import date, timedelta


def countries():
    return list(holidays.list_supported_countries().keys())


def generate_dates(n: int):
    # Days after 1970
    random_dates = [
        date(1970, 1, 1) + timedelta(days=random.randint(10957, 29219))
        for _ in range(n)
    ]
    return sorted(random_dates)


@mark.parametrize("country", countries())
@mark.parametrize("n_rows", [10, 100, 1000])
def test_fuzzy_is_holiday(country: str, n_rows: int):
    dates = generate_dates(n_rows)

    country_holidays = holidays.country_holidays(country)

    expected_is_holiday = [date in country_holidays for date in dates]

    df = pl.DataFrame(
        {
            "date": dates,
            "country": [country.lower()] * n_rows,
            "expected_is_holiday": expected_is_holiday,
        }
    )

    actual_df = df.with_columns(
        is_holiday=plh.is_holiday("date", "country"),
    )
    actual_series = actual_df.get_column("is_holiday")
    expected_series = actual_df.get_column("expected_is_holiday")

    assert_series_equal(actual_series, expected_series, check_names=False)


@mark.parametrize("country", countries())
@mark.parametrize("n_rows", [10, 100, 1000])
def test_fuzzy_get_holiday(country: str, n_rows: int):
    dates = generate_dates(n_rows)

    country_holidays = holidays.country_holidays(country, language="en_US")

    expected_get_holiday = pl.Series(
        "is_holiday", [country_holidays.get(date) or "" for date in dates]
    )

    df = pl.DataFrame(
        {
            "date": dates,
            "country": [country.lower()] * n_rows,
            "expected_get_holiday": expected_get_holiday,
        }
    )

    actual_df = df.with_columns(
        get_holiday=plh.get_holiday("date", "country")
        .str.to_lowercase()
        .str.replace_all("-", " "),
        expected_get_holiday=pl.col("expected_get_holiday")
        .str.to_lowercase()
        .str.replace_all("-", " "),
    )
    actual_series = actual_df.get_column("get_holiday")
    expected_series = actual_df.get_column("expected_get_holiday")

    assert_series_equal(actual_series, expected_series, check_names=False)


