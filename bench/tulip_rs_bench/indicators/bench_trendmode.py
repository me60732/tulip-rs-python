# Benchmark: trendmode (Trend Mode)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.trendmode.indicator([data.close], options)


BENCHMARK = BenchmarkDef(
    name="trendmode",
    options_list=[[0.0], [0.05], [0.07], [0.10]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
)
