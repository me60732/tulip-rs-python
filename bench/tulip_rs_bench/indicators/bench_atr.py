# Benchmark: atr (Average True Range)
from __future__ import annotations

from typing import Any, List

import ta.volatility
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
    return tulip_rs.indicators.atr.indicator([data.high, data.low, data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volatility.AverageTrueRange(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).average_true_range()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.atr(data.high, data.low, data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.atr(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="atr",
    options_list=[[5.0], [14.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
