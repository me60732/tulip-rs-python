# Benchmark: chandelierexit (Chandelier Exit)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.chandelierexit.indicator(
        [data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.chandelier_exit(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="chandelierexit",
    options_list=[[14.0, 3.0], [20.0, 3.0], [22.0, 3.0], [22.0, 2.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
