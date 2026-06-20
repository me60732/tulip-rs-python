# Benchmark: smaenvelope (SMA Envelope)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.smaenvelope.indicator([data.close], options)


BENCHMARK = BenchmarkDef(
    name="smaenvelope",
    options_list=[[20.0, 2.5], [20.0, 5.0], [50.0, 2.5], [50.0, 5.0]],
    tulip_fn=_tulip,
    ref_fn=None,
)
