# Benchmark: willr (Williams %R)
from __future__ import annotations

from typing import Any, List

import ta.momentum
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
    return tulip_rs.indicators.willr.indicator(
        [data.high, data.low, data.close], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.WilliamsRIndicator(
        high=data.high, low=data.low, close=data.close, lbp=int(options[0])
    ).williams_r()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.willr(data.high, data.low, data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.willr(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="willr",
    options_list=[[25.0], [35.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
