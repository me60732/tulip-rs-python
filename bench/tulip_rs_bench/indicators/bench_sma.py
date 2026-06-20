# Benchmark: sma (Simple Moving Average)
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
    return tulip_rs.indicators.sma.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.SMAIndicator(
        close=data.close, window=int(options[0])
    ).sma_indicator()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.sma(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.sma(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="sma",
    options_list=[[50.0], [100.0], [200.0], [300.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
