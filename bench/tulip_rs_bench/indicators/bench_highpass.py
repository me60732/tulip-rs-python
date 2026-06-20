# Benchmark: highpass (High Pass Filter)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.highpass.indicator([data.close], options)


BENCHMARK = BenchmarkDef(
    name="highpass",
    options_list=[[20.0], [40.0], [60.0], [80.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
)
