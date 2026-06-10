# Benchmark: stoch (Stochastic Oscillator)
from __future__ import annotations

from typing import Any, List

import ta.momentum

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.stoch.indicator(
        [data.high, data.low, data.close], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # options: [k_period, k_slow_period, d_period]
    # ta.StochasticOscillator: window=K period, smooth_window=D period
    return ta.momentum.StochasticOscillator(
        high=data.high,
        low=data.low,
        close=data.close,
        window=int(options[0]),
        smooth_window=int(options[2]),
    ).stoch()


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
)