# Benchmark: ultosc (Ultimate Oscillator)
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
    return tulip_rs.indicators.ultosc.indicator(
        [data.high, data.low, data.close], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.UltimateOscillator(
        high=data.high,
        low=data.low,
        close=data.close,
        window1=int(options[0]),
        window2=int(options[1]),
        window3=int(options[2]),
    ).ultimate_oscillator()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.ultosc(
        data.high,
        data.low,
        data.close,
        short_period=int(options[0]),
        medium_period=int(options[1]),
        long_period=int(options[2]),
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.uo(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="ultosc",
    options_list=[
        [7.0, 14.0, 28.0],
        [4.0, 8.0, 16.0],
        [5.0, 10.0, 20.0],
        [6.0, 12.0, 24.0],
    ],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
