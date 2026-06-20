# Benchmark: ef (Efficiency Ratio)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ef.indicator([data.close], options)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.er(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="ef",
    options_list=[[5.0], [10.0], [14.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
