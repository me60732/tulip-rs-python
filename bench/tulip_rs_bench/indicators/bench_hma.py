# Benchmark: hma (Hull Moving Average)
from __future__ import annotations

from typing import Any, List

import numpy as np

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.hma.indicator([data.close], options)


def _wma(s, period: int):
    w = np.arange(1, period + 1, dtype=np.float64)
    wsum = w.sum()
    return s.rolling(period).apply(lambda x: np.dot(x, w) / wsum, raw=True)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    n = int(options[0])
    half = max(1, n // 2)
    sq = max(1, int(n**0.5))
    raw = 2 * _wma(data.close, half) - _wma(data.close, n)
    return _wma(raw, sq)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.hma(pd.Series(data.close), length=int(options[0]))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.hma.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.hma.simd_by_options([data.close], options_list, None)


BENCHMARK = BenchmarkDef(
    name="hma",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
