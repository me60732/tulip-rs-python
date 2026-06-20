# Benchmark: bbands (Bollinger Bands)
from __future__ import annotations

from typing import Any, List

import ta.volatility
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
    return tulip_rs.indicators.bbands.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volatility.BollingerBands(
        close=data.close, window=int(options[0]), window_dev=options[1]
    ).bollinger_hband()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.bbands(data.close, period=int(options[0]), stddev=options[1])


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.bbands(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="bbands",
    options_list=[[5.0, 2.0], [14.0, 2.0], [20.0, 2.0], [50.0, 2.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
