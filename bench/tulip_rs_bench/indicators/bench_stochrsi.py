# Benchmark: stochrsi (Stochastic RSI)
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
    return tulip_rs.indicators.stochrsi.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.StochRSIIndicator(
        close=data.close, window=int(options[0])
    ).stochrsi()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.stochrsi(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.stochrsi(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="stochrsi",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
