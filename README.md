# Polars Holidays
Plugin for the polars dataframe library to work with holidays.

## Installation

```bash
pip install polars-holidays
```

## Usage
To use the is_holiday and get_holiday functions in Polars, follow the examples below.
Importing the Library
```python
import polars as pl
import polars_holidays as plh
```

### Check if a Date is a Holiday
You can check if a specific date is a holiday for a given country using the is_holiday function.

Use ISO-2 Country code in lowercase.

```python
df = pl.DataFrame(
    {
        "date": ["2023-01-01", "2023-12-25"],
        "country": ["us", "us"]
    }
)

df = df.with_columns(
    is_holiday=plh.is_holiday("date", "country")
)
```

### Get the name of the holiday(s)
To retrieve the name of the holiday for a specific date and country, use the get_holiday function. If there are multiple holidays on the same day, it will return as a semi-colon separated list of holidays.

```python
df = df.with_columns(
    holiday_name=plh.get_holiday("date", "country")
)
```


### Single country
If you don't have a column with country code, and just want to specify a single country, you can use `pl.Lit("us")`:

df = df.with_columns(
    holiday_name=plh.get_holiday("date", pl.Lit("us"))
)