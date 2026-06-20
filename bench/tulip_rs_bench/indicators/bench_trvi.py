# Benchmark: trvi (True Range Volatility Indicator)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.trvi.indicator(
        [data.high, data.low, data.close], options
    )


BENCHMARK = BenchmarkDef(
    name="trvi",
    options_list=[[5.0], [14.0], [20.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=None,
)
