# Benchmark: adx (Average Directional Movement Index)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta
import ta.trend

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.adx.indicator([data.high, data.low, data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.ADXIndicator(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).adx()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    import tulipy

    return tulipy.adx(data.high, data.low, data.close, period=int(options[0]))


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.adx(
        pd.Series(data.high),
        pd.Series(data.low),
        pd.Series(data.close),
        length=int(options[0]),
    )


BENCHMARK = BenchmarkDef(
    name="adx",
    options_list=[[5.0], [14.0], [24.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
