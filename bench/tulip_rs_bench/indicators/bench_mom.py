# Benchmark: mom (Momentum)
# Reference: pandas Series.diff — no ta library equivalent
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.mom.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # Momentum = close[n] - close[n - period]
    return data.close.diff(int(options[0]))


BENCHMARK = BenchmarkDef(
    name="mom",
    options_list=[[25.0], [30.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)