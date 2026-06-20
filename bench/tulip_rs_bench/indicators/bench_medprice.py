# Benchmark: medprice (Median Price)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.medprice.indicator([data.high, data.low], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.medprice(data.high, data.low)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.hl2(pd.Series(data.high), pd.Series(data.low))

BENCHMARK = BenchmarkDef(
    name="medprice",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
