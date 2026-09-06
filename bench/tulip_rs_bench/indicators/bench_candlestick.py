# Benchmark: candlestick (Candlestick Pattern Scanner)
# Note: candlestick has a distinct API — it takes named OHLC args and returns
# per-bar pattern dicts rather than float arrays.
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.candlestick.candlestick(
        data.open,
        data.high,
        data.low,
        data.close,
        options=options if options else None,
    )


BENCHMARK = BenchmarkDef(
    name="candlestick",
    options_list=[[14.0, 20.0, 9.0]],
    tulip_fn=_tulip,
    ref_fn=None,
)
