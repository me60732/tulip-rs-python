# Benchmark: tema (Triple Exponential Moving Average)
from __future__ import annotations

from typing import Any, List

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
    return tulip_rs.indicators.tema.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    n = int(options[0])
    ema1 = data.close.ewm(span=n, adjust=False).mean()
    ema2 = ema1.ewm(span=n, adjust=False).mean()
    ema3 = ema2.ewm(span=n, adjust=False).mean()
    return 3 * ema1 - 3 * ema2 + ema3


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.tema(data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.tema(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="tema",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
