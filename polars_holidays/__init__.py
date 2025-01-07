from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function

from polars_holidays._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars_holidays.typing import IntoExprColumn

LIB = Path(__file__).parent


def is_holiday(date: IntoExprColumn, country_code: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[date, country_code],
        plugin_path=LIB,
        function_name="is_holiday",
        is_elementwise=True,
    )


def get_holiday(date: IntoExprColumn, country_code: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[date, country_code],
        plugin_path=LIB,
        function_name="get_holiday",
        is_elementwise=True,
    )
