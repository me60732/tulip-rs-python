# Benchmark: macd (MACD)
from __future__ import annotations

from typing import Any, List

import ta.trend

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.macd.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.MACD(
        close=data.close,
        window_fast=int(options[0]),
        window_slow=int(options[1]),
        window_sign=int(options[2]),
    ).macd()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.macd(pd.Series(data.close), fast=int(options[0]), slow=int(options[1]), signal=int(options[2]))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.macd.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.macd.simd_by_options([data.close], options_list, None)


BENCHMARK = BenchmarkDef(
    name="macd",
    options_list=[
        [5.0, 13.0, 8.0],
        [19.0, 39.0, 9.0],
        [10.0, 30.0, 10.0],
        [6.0, 20.0, 9.0],
    ],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
