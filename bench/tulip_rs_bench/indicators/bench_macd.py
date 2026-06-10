# Benchmark: macd (MACD)
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
    return tulip_rs.indicators.macd.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.MACD(
        close=data.close,
        window_fast=int(options[0]),
        window_slow=int(options[1]),
        window_sign=int(options[2]),
    ).macd()


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
)