# Benchmark: atr (Average True Range)
from __future__ import annotations

from typing import Any, List

import ta.volatility

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.atr.indicator([data.high, data.low, data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volatility.AverageTrueRange(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).average_true_range()


BENCHMARK = BenchmarkDef(
    name="atr",
    options_list=[[5.0], [14.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)