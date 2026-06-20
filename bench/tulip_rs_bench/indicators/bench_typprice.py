# Benchmark: typprice (Typical Price)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.typprice.indicator(
        [data.high, data.low, data.close], options
    )


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.typprice(data.high, data.low, data.close)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.hlc3(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="typprice",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
