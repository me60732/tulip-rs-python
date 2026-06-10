# Benchmark: dema (Double Exponential Moving Average)
# Reference: DEMA = 2*EMA(n) - EMA(EMA(n)), implemented with pandas ewm.
# No ta library equivalent.
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.dema.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    n = int(options[0])
    ema1 = data.close.ewm(span=n, adjust=False).mean()
    ema2 = ema1.ewm(span=n, adjust=False).mean()
    return 2 * ema1 - ema2


BENCHMARK = BenchmarkDef(
    name="dema",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)