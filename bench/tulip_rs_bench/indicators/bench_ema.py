# Benchmark: ema (Exponential Moving Average)
from __future__ import annotations

from typing import Any, List

import ta.trend

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ema.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.EMAIndicator(
        close=data.close, window=int(options[0])
    ).ema_indicator()


BENCHMARK = BenchmarkDef(
    name="ema",
    options_list=[[14.0], [20.0], [26.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)