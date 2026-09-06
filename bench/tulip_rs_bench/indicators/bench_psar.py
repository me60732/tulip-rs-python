# Benchmark: psar (Parabolic SAR)
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
    return tulip_rs.indicators.psar.indicator([data.high, data.low], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low] for stock in stocks]
    return tulip_rs.indicators.psar.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.psar.simd_by_options([data.high, data.low], options_list, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.PSARIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        step=options[0],
        max_step=options[1],
    ).psar_up()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.psar(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="psar",
    options_list=[[0.02, 0.2], [0.01, 0.2], [0.02, 0.1], [0.04, 0.4]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
