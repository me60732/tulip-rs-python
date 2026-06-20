# Benchmark: wma (Weighted Moving Average)
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
    return tulip_rs.indicators.wma.indicator([data.close], options)


def _wma(s, period: int):
    w = np.arange(1, period + 1, dtype=np.float64)
    wsum = w.sum()
    return s.rolling(period).apply(lambda x: np.dot(x, w) / wsum, raw=True)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return _wma(data.close, int(options[0]))


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.wma(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.wma(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="wma",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
