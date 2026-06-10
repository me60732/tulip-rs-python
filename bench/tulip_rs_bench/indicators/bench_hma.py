# Benchmark: hma (Hull Moving Average)
# Reference: HMA = WMA(2*WMA(n/2) - WMA(n), sqrt(n)), implemented with pandas/numpy.
# No ta library equivalent.
from __future__ import annotations

from typing import Any, List

import numpy as np

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


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


BENCHMARK = BenchmarkDef(
    name="hma",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)