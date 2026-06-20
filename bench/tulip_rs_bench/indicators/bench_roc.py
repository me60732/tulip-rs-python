# Benchmark: roc (Rate of Change)
from __future__ import annotations

from typing import Any, List

import ta.momentum
import tulipy

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.roc.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.ROCIndicator(close=data.close, window=int(options[0])).roc()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.roc(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.roc(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="roc",
    options_list=[[25.0], [30.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
