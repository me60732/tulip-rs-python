# Benchmark: dx (Directional Movement Index)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.dx.indicator([data.high, data.low, data.close], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.dx(data.high, data.low, data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.adx(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="dx",
    options_list=[[5.0], [14.0], [24.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
