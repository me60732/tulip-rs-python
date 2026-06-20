# Benchmark: vwma (Volume-Weighted Moving Average)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vwma.indicator([data.close, data.volume], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.vwma(data.close, data.volume, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.vwma(pd.Series(data.close), pd.Series(data.volume), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="vwma",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
