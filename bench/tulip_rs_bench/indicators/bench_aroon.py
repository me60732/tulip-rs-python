# Benchmark: aroon (Aroon)
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
    return tulip_rs.indicators.aroon.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.AroonIndicator(
        high=data.high, low=data.low, window=int(options[0])
    ).aroon_up()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.aroon(pd.Series(data.high), pd.Series(data.low), length=int(options[0]))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low] for stock in stocks]
    return tulip_rs.indicators.aroon.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.aroon.simd_by_options([data.high, data.low], options_list, None)


BENCHMARK = BenchmarkDef(
    name="aroon",
    options_list=[[25.0], [35.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
