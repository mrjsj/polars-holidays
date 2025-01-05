from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function

from polars_holidays._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars_holidays.typing import IntoExprColumn

LIB = Path(__file__).parent


# def pig_latinnify(expr: IntoExprColumn) -> pl.Expr:
#     return register_plugin_function(
#         args=[expr],
#         plugin_path=LIB,
#         function_name="pig_latinnify",
#         is_elementwise=True,
#     )

# def noop(expr: IntoExprColumn) -> pl.Expr:
#     return register_plugin_function(
#         args=[expr],
#         plugin_path=LIB,
#         function_name="noop",
#         is_elementwise=True,
#     )

# def abs_i64(expr: IntoExprColumn) -> pl.Expr:
#     return register_plugin_function(
#         args=[expr],
#         plugin_path=LIB,
#         function_name="abs_i64",
#         is_elementwise=True,
#     )

# def abs_numeric(expr: IntoExprColumn) -> pl.Expr:
#     return register_plugin_function(
#         args=[expr],
#         plugin_path=LIB,
#         function_name="abs_numeric",
#         is_elementwise=True,
#     )

def is_holiday(expr: IntoExprColumn, *, country_code: str) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="is_holiday",
        is_elementwise=True,
        kwargs={"country_code": country_code},
    )

def get_holiday(expr: IntoExprColumn, *, country_code: str) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="get_holiday",
        is_elementwise=True,
        kwargs={"country_code": country_code},
    )