# Benchmark: macd (MACD)
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
    return tulip_rs.indicators.macd.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.MACD(
        close=data.close,
        window_fast=int(options[0]),
        window_slow=int(options[1]),
        window_sign=int(options[2]),
    ).macd()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.macd(
        data.close,
        short_period=int(options[0]),
        long_period=int(options[1]),
        signal_period=int(options[2]),
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.macd(pd.Series(data.close), fast=int(options[0]), slow=int(options[1]), signal=int(options[2]))

BENCHMARK = BenchmarkDef(
    name="macd",
    options_list=[
        [5.0, 13.0, 8.0],
        [19.0, 39.0, 9.0],
        [10.0, 30.0, 10.0],
        [6.0, 20.0, 9.0],
    ],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
