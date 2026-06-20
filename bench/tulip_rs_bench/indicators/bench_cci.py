# Benchmark: cci (Commodity Channel Index)
from __future__ import annotations

from typing import Any, List

import ta.trend
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
    return tulip_rs.indicators.cci.indicator([data.high, data.low, data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.CCIIndicator(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).cci()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.cci(data.high, data.low, data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.cci(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="cci",
    options_list=[[20.0], [25.0], [30.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
