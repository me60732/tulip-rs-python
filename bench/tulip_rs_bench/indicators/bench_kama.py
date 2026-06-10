# Benchmark: kama (Kaufman Adaptive Moving Average)
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
    return tulip_rs.indicators.kama.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.KAMAIndicator(close=data.close, window=int(options[0])).kama()


BENCHMARK = BenchmarkDef(
    name="kama",
    options_list=[[5.0], [10.0], [14.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)