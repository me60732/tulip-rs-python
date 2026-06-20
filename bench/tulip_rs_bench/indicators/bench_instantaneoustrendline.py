# Benchmark: instantaneoustrendline (Instantaneous Trendline)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.instantaneoustrendline.indicator([data.close], [])


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ht_trendline(pd.Series(data.close))


BENCHMARK = BenchmarkDef(
    name="instantaneoustrendline",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
