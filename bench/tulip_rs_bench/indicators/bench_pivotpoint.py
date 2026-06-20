# Benchmark: pivotpoint (Pivot Point)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.pivotpoint.indicator(
        [data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.pivots(pd.Series(data.open), pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="pivotpoint",
    options_list=[[1.0], [5.0], [10.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
