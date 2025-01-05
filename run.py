import polars as pl
from polars_holidays import (
    # pig_latinnify,
    # noop,
    # abs_numeric,
    is_holiday,
    get_holiday,
)
from datetime import date

df = pl.DataFrame(
    {
        "english": ["this", "is", "not", "pig", "latin"],
        "i64": [1, -2, 3, -4, 5],
        "f64": [1.0, -2.0, 3.0, -4.0, 5.0],
        "mydate": [date(2025,1,1)] * 4 + [date(2025,2,2)]
    }
)
result = df.with_columns(
    # pig_latin=pig_latinnify("english"),
    # noop=noop("english"),
    # abs_i64=abs_numeric("i64"),
    # abs_f64=abs_numeric("f64"),
    # # abs_english=abs_numeric("english"), # Should raise an error
    is_holiday=is_holiday("mydate", country_code="uk"),
    get_holiday=get_holiday("mydate", country_code="uk"),
)
print(result)
