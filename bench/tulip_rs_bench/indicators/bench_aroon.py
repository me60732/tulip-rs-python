# Benchmark: aroon (Aroon)
from __future__ import annotations

from typing import Any, List

import ta.trend
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
    return tulip_rs.indicators.aroon.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.AroonIndicator(
        high=data.high, low=data.low, window=int(options[0])
    ).aroon_up()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.aroon(data.high, data.low, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.aroon(pd.Series(data.high), pd.Series(data.low), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="aroon",
    options_list=[[25.0], [35.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
