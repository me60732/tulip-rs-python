# Benchmark: ccfisher (CC Fisher Transform)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ccfisher.indicator([data.close], options)


BENCHMARK = BenchmarkDef(
    name="ccfisher",
    options_list=[[0.0], [0.05], [0.07], [0.10]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
)
