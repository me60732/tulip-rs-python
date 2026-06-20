# Benchmark: stoch (Stochastic Oscillator)
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
    return tulip_rs.indicators.stoch.indicator(
        [data.high, data.low, data.close], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.StochasticOscillator(
        high=data.high,
        low=data.low,
        close=data.close,
        window=int(options[0]),
        smooth_window=int(options[2]),
    ).stoch()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.stoch(
        data.high,
        data.low,
        data.close,
        pct_k_period=int(options[0]),
        pct_k_slowing_period=int(options[1]),
        pct_d_period=int(options[2]),
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.stoch(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="stoch",
    options_list=[
        [28.0, 16.0, 12.0],
        [35.0, 21.0, 14.0],
        [50.0, 30.0, 21.0],
        [100.0, 50.0, 30.0],
    ],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
