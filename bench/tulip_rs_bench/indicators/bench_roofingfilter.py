# Benchmark: roofingfilter (Roofing Filter)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.roofingfilter.indicator([data.close], options)


BENCHMARK = BenchmarkDef(
    name="roofingfilter",
    options_list=[[10.0, 20.0], [15.0, 30.0], [20.0, 40.0], [25.0, 50.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
)
