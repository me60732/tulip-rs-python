# Benchmark: vortex (Vortex Indicator)
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
    return tulip_rs.indicators.vortex.indicator(
        [data.high, data.low, data.close], options
    )


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.close] for stock in stocks]
    return tulip_rs.indicators.vortex.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.vortex.simd_by_options([data.high, data.low, data.close], options_list, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.VortexIndicator(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).vortex_indicator_pos()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.vortex(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="vortex",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
