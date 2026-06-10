# Benchmark: trix (TRIX)
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
    return tulip_rs.indicators.trix.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.TRIXIndicator(close=data.close, window=int(options[0])).trix()


BENCHMARK = BenchmarkDef(
    name="trix",
    options_list=[[14.0], [18.0], [20.0], [25.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
