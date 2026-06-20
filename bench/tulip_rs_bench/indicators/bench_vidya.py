# Benchmark: vidya (Variable Index Dynamic Average)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vidya.indicator([data.close], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.vidya(
        data.close,
        short_period=int(options[0]),
        long_period=int(options[1]),
        alpha=options[2],
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.vidya(pd.Series(data.close))

BENCHMARK = BenchmarkDef(
    name="vidya",
    options_list=[
        [2.0, 5.0, 0.2],
        [5.0, 20.0, 0.2],
        [9.0, 30.0, 0.2],
        [12.0, 26.0, 0.1],
    ],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
