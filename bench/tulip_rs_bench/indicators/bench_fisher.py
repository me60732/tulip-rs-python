# Benchmark: fisher (Fisher Transform)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.fisher.indicator([data.high, data.low], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.fisher(data.high, data.low, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.fisher(pd.Series(data.high), pd.Series(data.low), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="fisher",
    options_list=[[5.0], [9.0], [14.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
