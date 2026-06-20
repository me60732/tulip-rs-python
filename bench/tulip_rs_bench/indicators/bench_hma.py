# Benchmark: hma (Hull Moving Average)
from __future__ import annotations

from typing import Any, List

import numpy as np
import tulipy

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


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.hma(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.hma(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="hma",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
