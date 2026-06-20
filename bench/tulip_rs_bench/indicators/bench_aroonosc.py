# Benchmark: aroonosc (Aroon Oscillator)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.aroonosc.indicator([data.high, data.low], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.aroonosc(data.high, data.low, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.aroon(pd.Series(data.high), pd.Series(data.low), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="aroonosc",
    options_list=[[25.0], [35.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
