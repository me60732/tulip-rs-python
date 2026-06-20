# Benchmark: pvi (Positive Volume Index)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.pvi.indicator([data.close, data.volume], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.pvi(data.close, data.volume)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.pvi(pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="pvi",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
