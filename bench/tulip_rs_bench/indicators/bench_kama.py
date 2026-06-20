# Benchmark: kama (Kaufman Adaptive Moving Average)
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
    return tulip_rs.indicators.kama.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.KAMAIndicator(close=data.close, window=int(options[0])).kama()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.kama(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.kama(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="kama",
    options_list=[[5.0], [10.0], [14.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
