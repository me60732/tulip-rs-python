# Benchmark: kvo (Klinger Volume Oscillator)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.kvo.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.kvo(
        data.high,
        data.low,
        data.close,
        data.volume,
        short_period=int(options[0]),
        long_period=int(options[1]),
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.kvo(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="kvo",
    options_list=[[34.0, 55.0], [20.0, 40.0], [10.0, 30.0], [5.0, 20.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
